contract;

mod data_structures;
mod errors;
mod events;
mod interface;
mod utils;

use ::data_structures::{DistributionState, TokenDistribution};
use ::errors::{AccessError, AssetError};
use ::events::{
    BuybackEvent,
    CreateEvent,
    EndEvent,
    PurchaseAdminEvent,
    PurchaseEvent,
    ReserveEvent,
    SellEvent,
    TokenPriceEvent,
    WithdrawEvent,
};
use ::interface::{Info, TokenDistributor};
use std::{
    auth::msg_sender,
    call_frames::{
        contract_id,
        msg_asset_id,
    },
    context::{
        msg_amount,
        this_balance,
    },
    token::transfer,
};
use ::utils::{create_fractional_nft, fractional_nft_supply, withdraw_fractional_nft};

storage {
    /// Maintains a mapping of information on every token distribution started using this contract.
    /// Mapping(Token Contract -> Token Distribution Information)
    token_distributions: StorageMap<ContractId, TokenDistribution> = StorageMap {},
}

impl TokenDistributor for Contract {
    #[payable]
    #[storage(read, write)]
    fn buyback(fractional_nft_id: ContractId, token_price: u64) {
        require(storage.token_distributions.get(fractional_nft_id).is_some(), AccessError::DistributionDoesNotExist);
        let mut token_distribution = storage.token_distributions.get(fractional_nft_id).unwrap();

        require(token_distribution.admin.is_some() && token_distribution.admin.unwrap() == msg_sender().unwrap(), AccessError::NotTokenAdmin);
        require(token_distribution.state == DistributionState::Distributed, AccessError::InvalidState);

        // Ensure the seller is only providing the amount to buyback the number of tokens that have been sold
        let token_supply = fractional_nft_supply(fractional_nft_id);
        require(msg_amount() == (token_supply - this_balance(fractional_nft_id)) * token_price && msg_asset_id() == token_distribution.external_asset, AssetError::InvalidAssetTransfer);

        // Store the buyback price and change the state
        token_distribution.token_price = token_price;
        token_distribution.state = DistributionState::Buyback;
        storage.token_distributions.insert(fractional_nft_id, token_distribution);

        log(BuybackEvent {
            fractional_nft_id,
            token_price,
        });
    }

    #[storage(read, write)]
    fn create(
        nft_asset_id: ContractId,
        external_asset: ContractId,
        fractional_nft_id: ContractId,
        reserve_price: Option<u64>,
        token_admin: Option<Identity>,
        token_price: u64,
        token_supply: u64,
        nft_token_id: u64,
    ) {
        require(storage.token_distributions.get(fractional_nft_id).is_none(), AccessError::DistributionAlreadyExists);

        match token_admin {
            Option::Some(admin) => {
                create_fractional_nft(Option::Some(Identity::ContractId(contract_id())), fractional_nft_id, nft_asset_id, token_supply, nft_token_id);
            },
            Option::None => {
                create_fractional_nft(Option::None, fractional_nft_id, nft_asset_id, token_supply, nft_token_id);
            }
        }

        // Store the newly created token distribution information
        let token_distribution = TokenDistribution::new(token_admin, external_asset, nft_asset_id, reserve_price, nft_token_id, token_price);
        storage.token_distributions.insert(fractional_nft_id, token_distribution);

        log(CreateEvent {
            fractional_nft_id,
            token_distribution,
        });
    }

    #[storage(read, write)]
    fn end(fractional_nft_id: ContractId) {
        require(storage.token_distributions.get(fractional_nft_id).is_some(), AccessError::DistributionDoesNotExist);
        let mut token_distribution = storage.token_distributions.get(fractional_nft_id).unwrap();

        require(token_distribution.admin.is_some() && token_distribution.admin.unwrap() == msg_sender().unwrap(), AccessError::NotTokenAdmin);
        require(token_distribution.state == DistributionState::Buyback || token_distribution.state == DistributionState::Started, AccessError::InvalidState);

        // Update distribution state
        token_distribution.state = DistributionState::Ended;
        storage.token_distributions.insert(fractional_nft_id, token_distribution);

        // Send the remaining tokens to the fractionalized NFT contract and transfer ownership of the NFT to the admin
        transfer(this_balance(fractional_nft_id), fractional_nft_id, Identity::ContractId(fractional_nft_id));
        withdraw_fractional_nft(fractional_nft_id, token_distribution.admin.unwrap());

        log(EndEvent {
            fractional_nft_id,
        });
    }

    #[payable]
    #[storage(read, write)]
    fn purchase(amount: u64, fractional_nft_id: ContractId) {
        require(storage.token_distributions.get(fractional_nft_id).is_some(), AccessError::DistributionDoesNotExist);
        let mut token_distribution = storage.token_distributions.get(fractional_nft_id).unwrap();

        // Make sure we are in purchasing state and if this is the first purchase change the state to distributing
        require(token_distribution.state == DistributionState::Started || token_distribution.state == DistributionState::Distributed, AccessError::InvalidState);
        if token_distribution.state == DistributionState::Started {
            token_distribution.state = DistributionState::Distributed;
        }

        require(amount <= this_balance(fractional_nft_id), AssetError::NotEnoughTokensAvailable);
        require(amount * token_distribution.token_price == msg_amount() && msg_asset_id() == token_distribution.external_asset, AssetError::InvalidAssetTransfer);

        // Store the amount deposited and send the buyer some fractionalized NFT tokens
        token_distribution.external_deposits += msg_amount();
        storage.token_distributions.insert(fractional_nft_id, token_distribution);
        transfer(amount, fractional_nft_id, msg_sender().unwrap());

        log(PurchaseEvent {
            amount,
            buyer: msg_sender().unwrap(),
            fractional_nft_id,
        });
    }

    #[payable]
    #[storage(read, write)]
    fn purchase_admin(
        admin: Option<Identity>,
        fractional_nft_id: ContractId,
        reserve: Option<u64>,
    ) {
        require(storage.token_distributions.get(fractional_nft_id).is_some(), AccessError::DistributionDoesNotExist);
        let mut token_distribution = storage.token_distributions.get(fractional_nft_id).unwrap();

        require(token_distribution.reserve_price.is_some() && token_distribution.admin.is_some(), AccessError::NoReserveAvailable);
        require(token_distribution.state != DistributionState::Ended, AccessError::InvalidState);
        require(msg_asset_id() == token_distribution.external_asset && msg_amount() == token_distribution.reserve_price.unwrap(), AssetError::InvalidAssetTransfer);

        let previous_admin = token_distribution.admin;
        // Store the new admin and reserve information
        token_distribution.reserve_price = reserve;
        token_distribution.admin = admin;
        storage.token_distributions.insert(fractional_nft_id, token_distribution);

        // Send the payment to the old admin
        transfer(msg_amount(), token_distribution.external_asset, previous_admin.unwrap());

        log(PurchaseAdminEvent {
            admin,
            fractional_nft_id,
            reserve,
        });
    }

    #[payable]
    #[storage(read)]
    fn sell(fractional_nft_id: ContractId) {
        require(storage.token_distributions.get(fractional_nft_id).is_some(), AccessError::DistributionDoesNotExist);
        let mut token_distribution = storage.token_distributions.get(fractional_nft_id).unwrap();

        require(token_distribution.state == DistributionState::Buyback, AccessError::InvalidState);
        require(msg_asset_id() == fractional_nft_id, AssetError::InvalidAssetTransfer);

        // Send the fractionalized NFT token seller their payment and the sold tokens to the fractionalized NFT contract
        transfer(msg_amount() * token_distribution.token_price, token_distribution.external_asset, msg_sender().unwrap());
        transfer(msg_amount(), fractional_nft_id, Identity::ContractId(fractional_nft_id));

        log(SellEvent {
            fractional_nft_id,
            seller: msg_sender().unwrap(),
        });
    }

    #[storage(read, write)]
    fn set_reserve(fractional_nft_id: ContractId, reserve: Option<u64>) {
        require(storage.token_distributions.get(fractional_nft_id).is_some(), AccessError::DistributionDoesNotExist);
        let mut token_distribution = storage.token_distributions.get(fractional_nft_id).unwrap();

        require(token_distribution.admin.is_some() && token_distribution.admin.unwrap() == msg_sender().unwrap(), AccessError::NotTokenAdmin);

        token_distribution.reserve_price = reserve;
        storage.token_distributions.insert(fractional_nft_id, token_distribution);

        log(ReserveEvent {
            fractional_nft_id,
            reserve,
        });
    }

    #[storage(read, write)]
    fn set_token_price(fractional_nft_id: ContractId, token_price: u64) {
        require(storage.token_distributions.get(fractional_nft_id).is_some(), AccessError::DistributionDoesNotExist);
        let mut token_distribution = storage.token_distributions.get(fractional_nft_id).unwrap();

        require(token_distribution.admin.is_some() && token_distribution.admin.unwrap() == msg_sender().unwrap(), AccessError::NotTokenAdmin);
        require(token_distribution.state == DistributionState::Started || token_distribution.state == DistributionState::Distributed, AccessError::InvalidState);

        token_distribution.token_price = token_price;
        storage.token_distributions.insert(fractional_nft_id, token_distribution);

        log(TokenPriceEvent {
            fractional_nft_id,
            token_price,
        });
    }

    #[storage(read, write)]
    fn withdraw(fractional_nft_id: ContractId) {
        require(storage.token_distributions.get(fractional_nft_id).is_some(), AccessError::DistributionDoesNotExist);
        let mut token_distribution = storage.token_distributions.get(fractional_nft_id).unwrap();

        require(token_distribution.admin.is_some() && token_distribution.admin.unwrap() == msg_sender().unwrap(), AccessError::NotTokenAdmin);

        // Update the amount available to withdraw to zero and send the admin their tokens
        let amount = token_distribution.external_deposits;
        token_distribution.external_deposits = 0;
        storage.token_distributions.insert(fractional_nft_id, token_distribution);

        transfer(amount, token_distribution.external_asset, token_distribution.admin.unwrap());

        log(WithdrawEvent {
            amount,
            external_asset: token_distribution.external_asset,
            fractional_nft_id,
        });
    }
}

impl Info for Contract {
    #[storage(read)]
    fn token_distribution(fractional_nft_id: ContractId) -> Option<TokenDistribution> {
        storage.token_distributions.get(fractional_nft_id)
    }
}

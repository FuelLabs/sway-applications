contract;

dep data_structures;
dep errors;
dep events;
dep interface;
dep utils;

use data_structures::{DistributionState, TokenDistribution};
use errors::{AccessError, AssetError};
use events::{
    Buyback,
    Canceled,
    Closed,
    Created,
    Purchased,
    PurchasedOwnership,
    Reserve,
    Sell,
    TokenPrice,
    Withdraw,
};
use interface::TokenDistributor;
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
    logging::log,
    storage::StorageMap,
    token::transfer,
};
use utils::{
    create_fractional_nft,
    fractional_nft_supply,
    require_fractional_nft_exists,
    withdraw_fractional_nft,
};

storage {
    /// Maintains a mapping of information on every token distribution started using this contract.
    /// Mapping(Token Contract -> Token Distribution Information)
    token_distributions: StorageMap<ContractId, Option<TokenDistribution>> = StorageMap {},
}

impl TokenDistributor for Contract {
    #[storage(read, write)]
    fn buyback(fractional_nft: ContractId, token_price: u64) {
        let mut token_distribution = require_fractional_nft_exists(storage.token_distributions.get(fractional_nft));

        require(token_distribution.owner.is_some() && token_distribution.owner.unwrap() == msg_sender().unwrap(), AccessError::NotFNftOwner);
        require(token_distribution.state == DistributionState::Distributed, AccessError::InvalidState);

        // Ensure the seller is only providing the amount to buyback the number of tokens that have been sold
        let token_supply = fractional_nft_supply(fractional_nft);
        require(msg_amount() == (token_supply - this_balance(fractional_nft)) * token_price && msg_asset_id() == token_distribution.external_asset, AssetError::InvalidAssetTransfer);

        // Store the buyback price and change the state
        token_distribution.token_price = token_price;
        token_distribution.state = DistributionState::Buyback;
        storage.token_distributions.insert(fractional_nft, Option::Some(token_distribution));

        log(Buyback {
            fractional_nft,
            token_price,
        });
    }

    #[storage(read, write)]
    fn create(
        external_asset: ContractId,
        fractional_nft: ContractId,
        nft: ContractId,
        owner: Option<Identity>,
        reserve_price: Option<u64>,
        token_price: u64,
        token_supply: u64,
        nft_token_id: u64,
    ) {
        require(storage.token_distributions.get(fractional_nft).is_none(), AccessError::DistributionAlreadyExists);

        create_fractional_nft(fractional_nft, nft, Identity::ContractId(contract_id()), token_supply, nft_token_id);

        // Store the newly created token distribution information
        let token_distribution = TokenDistribution::new(external_asset, nft, owner, reserve_price, nft_token_id, token_price);
        storage.token_distributions.insert(fractional_nft, Option::Some(token_distribution));

        log(Created {
            fractional_nft,
            token_distribution,
        });
    }

    #[storage(read, write)]
    fn end(fractional_nft: ContractId) {
        let mut token_distribution = require_fractional_nft_exists(storage.token_distributions.get(fractional_nft));

        require(token_distribution.owner.is_some() && token_distribution.owner.unwrap() == msg_sender().unwrap(), AccessError::NotFNftOwner);
        require(token_distribution.state == DistributionState::Buyback || token_distribution.state == DistributionState::Started, AccessError::InvalidState);

        // Update distribution state
        token_distribution.state = DistributionState::Ended;
        storage.token_distributions.insert(fractional_nft, Option::Some(token_distribution));

        // Send the remaining tokens to the fractionalized NFT contract and transfer ownership of the NFT to the owner
        transfer(this_balance(fractional_nft), fractional_nft, Identity::ContractId(fractional_nft));
        withdraw_fractional_nft(fractional_nft, token_distribution.owner.unwrap());

        log(Closed { fractional_nft });
    }

    #[storage(read, write)]
    fn purchase(amount: u64, fractional_nft: ContractId) {
        let mut token_distribution = require_fractional_nft_exists(storage.token_distributions.get(fractional_nft));

        // Make sure we are in purchasing state and if this is the first purchase change the state to distributing
        require(token_distribution.state == DistributionState::Started || token_distribution.state == DistributionState::Distributed, AccessError::InvalidState);
        if token_distribution.state == DistributionState::Started {
            token_distribution.state = DistributionState::Distributed;
        }

        require(amount <= this_balance(fractional_nft), AssetError::NotEnoughTokensAvailable);
        require(amount * token_distribution.token_price == msg_amount() && msg_asset_id() == token_distribution.external_asset, AssetError::InvalidAssetTransfer);

        // Store the amount deposited and send the buyer some fractionalized NFT tokens
        token_distribution.external_deposits += msg_amount();
        storage.token_distributions.insert(fractional_nft, Option::Some(token_distribution));
        transfer(amount, fractional_nft, msg_sender().unwrap());

        log(Purchased {
            amount,
            buyer: msg_sender().unwrap(),
            fractional_nft,
        });
    }

    #[storage(read, write)]
    fn purchase_ownership(
        fractional_nft: ContractId,
        owner: Option<Identity>,
        reserve: Option<u64>,
    ) {
        let mut token_distribution = require_fractional_nft_exists(storage.token_distributions.get(fractional_nft));

        require(token_distribution.reserve_price.is_some() && token_distribution.owner.is_some(), AccessError::NoReserveAvailable);
        require(token_distribution.state != DistributionState::Ended, AccessError::InvalidState);
        require(msg_asset_id() == token_distribution.external_asset && msg_amount() == token_distribution.reserve_price.unwrap(), AssetError::InvalidAssetTransfer);

        // Store the new owner and reserve information
        let previous_owner = token_distribution.owner;
        token_distribution.reserve_price = reserve;
        token_distribution.owner = owner;
        storage.token_distributions.insert(fractional_nft, Option::Some(token_distribution));

        // Send the payment to the old owner
        transfer(msg_amount(), token_distribution.external_asset, previous_owner.unwrap());

        log(PurchasedOwnership {
            fractional_nft,
            owner,
            reserve,
        });
    }

    #[storage(read)]
    fn sell(fractional_nft: ContractId) {
        let token_distribution = require_fractional_nft_exists(storage.token_distributions.get(fractional_nft));

        require(token_distribution.state == DistributionState::Buyback, AccessError::InvalidState);
        require(msg_asset_id() == fractional_nft, AssetError::InvalidAssetTransfer);

        // Send the fractionalized NFT token seller their payment and the sold tokens to the fractionalized NFT contract
        transfer(msg_amount() * token_distribution.token_price, token_distribution.external_asset, msg_sender().unwrap());
        transfer(msg_amount(), fractional_nft, Identity::ContractId(fractional_nft));

        log(Sell {
            fractional_nft,
            seller: msg_sender().unwrap(),
        });
    }

    #[storage(read, write)]
    fn set_token_price(fractional_nft: ContractId, token_price: u64) {
        let mut token_distribution = require_fractional_nft_exists(storage.token_distributions.get(fractional_nft));

        require(token_distribution.owner.is_some() && token_distribution.owner.unwrap() == msg_sender().unwrap(), AccessError::NotFNftOwner);
        require(token_distribution.state == DistributionState::Started || token_distribution.state == DistributionState::Distributed, AccessError::InvalidState);

        token_distribution.token_price = token_price;
        storage.token_distributions.insert(fractional_nft, Option::Some(token_distribution));

        log(TokenPrice {
            fractional_nft,
            token_price,
        });
    }

    #[storage(read, write)]
    fn set_reserve(fractional_nft: ContractId, reserve: Option<u64>) {
        let mut token_distribution = require_fractional_nft_exists(storage.token_distributions.get(fractional_nft));

        require(token_distribution.owner.is_some() && token_distribution.owner.unwrap() == msg_sender().unwrap(), AccessError::NotFNftOwner);

        token_distribution.reserve_price = reserve;
        storage.token_distributions.insert(fractional_nft, Option::Some(token_distribution));

        log(Reserve {
            fractional_nft,
            reserve,
        });
    }

    #[storage(read)]
    fn token_distribution(fractional_nft: ContractId) -> Option<TokenDistribution> {
        storage.token_distributions.get(fractional_nft)
    }

    #[storage(read, write)]
    fn withdraw(fractional_nft: ContractId) {
        let mut token_distribution = require_fractional_nft_exists(storage.token_distributions.get(fractional_nft));

        require(token_distribution.owner.is_some() && token_distribution.owner.unwrap() == msg_sender().unwrap(), AccessError::NotFNftOwner);

        // Update the amount available to withdraw to zero and send the owner his tokens
        let amount = token_distribution.external_deposits;
        token_distribution.external_deposits = 0;
        storage.token_distributions.insert(fractional_nft, Option::Some(token_distribution));

        transfer(amount, token_distribution.external_asset, token_distribution.owner.unwrap());

        log(Withdraw {
            amount,
            external_asset: token_distribution.external_asset,
            fractional_nft,
        });
    }
}

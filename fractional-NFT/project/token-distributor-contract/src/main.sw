contract;

dep data_structures;
dep interface;
dep utils;

use data_structures::{DistributionState, TokenDistribution};
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
    token_distributions: StorageMap<ContractId, Option<TokenDistribution>> = StorageMap {},
}

impl TokenDistributor for Contract {
    #[storage(read, write)]
    fn cancel(fractional_nft: ContractId) {
        let mut token_distribution = require_fractional_nft_exists(storage.token_distributions.get(fractional_nft));

        require(token_distribution.owner.is_some() && token_distribution.owner.unwrap() == msg_sender().unwrap(), "Not NFT owner");
        require(token_distribution.state == DistributionState::Created, "Not available");

        token_distribution.state = DistributionState::Closed;
        storage.token_distributions.insert(fractional_nft, Option::Some(token_distribution));

        transfer(this_balance(fractional_nft), fractional_nft, Identity::ContractId(fractional_nft));
        withdraw_fractional_nft(fractional_nft);
    }

    #[storage(read, write)]
    fn close(fractional_nft: ContractId) {
        let mut token_distribution = require_fractional_nft_exists(storage.token_distributions.get(fractional_nft));

        require(token_distribution.state == DistributionState::Returning, "Not in returning state");
        token_distribution.state = DistributionState::Closed;
        storage.token_distributions.insert(fractional_nft, Option::Some(token_distribution));

        withdraw_fractional_nft(fractional_nft);
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
        token_id: u64,
    ) {
        require(storage.token_distributions.get(fractional_nft).is_none(), "Fractional NFT already exists");
        create_fractional_nft(fractional_nft, nft, Identity::ContractId(contract_id()), token_supply, token_id);

        storage.token_distributions.insert(fractional_nft, Option::Some(TokenDistribution::new(external_asset, nft, owner, reserve_price, token_id, token_price)));
    }

    #[storage(read, write)]
    fn purchase(amount: u64, fractional_nft: ContractId) {
        let mut token_distribution = require_fractional_nft_exists(storage.token_distributions.get(fractional_nft));

        require(token_distribution.state == DistributionState::Created || token_distribution.state == DistributionState::Distributing, "Not available");
        if token_distribution.state == DistributionState::Created {
            token_distribution.state = DistributionState::Distributing;
        }

        require(amount <= this_balance(fractional_nft), "Buying too many tokens");
        require(amount * token_distribution.token_price == msg_amount() && msg_asset_id() == token_distribution.external_asset, "Incorrect asset");

        token_distribution.external_deposits += msg_amount();
        storage.token_distributions.insert(fractional_nft, Option::Some(token_distribution));
        transfer(amount, fractional_nft, msg_sender().unwrap());
    }

    #[storage(read, write)]
    fn purchase_reserve(
        fractional_nft: ContractId,
        owner: Option<Identity>,
        reserve: Option<u64>,
    ) {
        let mut token_distribution = require_fractional_nft_exists(storage.token_distributions.get(fractional_nft));

        require(token_distribution.reserve_price.is_some() && token_distribution.owner.is_some(), "No reserve");
        require(token_distribution.state != DistributionState::Closed, "NFT no longer purchasable");
        require(msg_asset_id() == token_distribution.external_asset && msg_amount() == token_distribution.reserve_price.unwrap(), "Incorrect Asset");

        let previous_owner = token_distribution.owner;
        token_distribution.reserve_price = reserve;
        token_distribution.owner = owner;
        storage.token_distributions.insert(fractional_nft, Option::Some(token_distribution));

        transfer(msg_amount(), token_distribution.external_asset, previous_owner.unwrap());
    }

    #[storage(read, write)]
    fn request_return(fractional_nft: ContractId, token_price: u64) {
        let mut token_distribution = require_fractional_nft_exists(storage.token_distributions.get(fractional_nft));

        require(token_distribution.owner.is_some() && token_distribution.owner.unwrap() == msg_sender().unwrap(), "Not NFT owner");
        require(token_distribution.state == DistributionState::Distributing, "Not available");

        let token_supply = fractional_nft_supply(fractional_nft);
        require(msg_amount() == (token_supply - this_balance(fractional_nft)) * token_price && msg_asset_id() == token_distribution.external_asset, "Incorrect asset");

        token_distribution.token_price = token_price;
        token_distribution.state = DistributionState::Returning;
        storage.token_distributions.insert(fractional_nft, Option::Some(token_distribution));
    }

    #[storage(read)]
    fn sell(fractional_nft: ContractId) {
        let token_distribution = require_fractional_nft_exists(storage.token_distributions.get(fractional_nft));

        require(token_distribution.state == DistributionState::Returning, "Not available");
        require(msg_asset_id() == fractional_nft, "Incorrect Asset");

        transfer(msg_amount() * token_distribution.token_price, token_distribution.external_asset, msg_sender().unwrap());
        transfer(msg_amount(), fractional_nft, Identity::ContractId(fractional_nft));
    }

    #[storage(read, write)]
    fn withdraw(fractional_nft: ContractId) {
        let mut token_distribution = require_fractional_nft_exists(storage.token_distributions.get(fractional_nft));

        require(token_distribution.owner.is_some() && token_distribution.owner.unwrap() == msg_sender().unwrap(), "Not NFT owner");

        let amount = token_distribution.external_deposits;
        token_distribution.external_deposits = 0;
        storage.token_distributions.insert(fractional_nft, Option::Some(token_distribution));

        transfer(amount, token_distribution.external_asset, token_distribution.owner.unwrap());
    }
}

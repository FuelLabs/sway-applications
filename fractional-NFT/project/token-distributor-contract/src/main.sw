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
use utils::{create_fractional_nft, withdraw_fractional_nft};

storage {
    token_distributions: StorageMap<ContractId, Option<TokenDistribution>> = StorageMap {},
}

impl TokenDistributor for Contract {
    #[storage(read, write)]
    fn close(fractional_nft: ContractId) {
        let token_distribution = storage.token_distributions.get(fractional_nft);
        require(token_distribution.is_some(), "Fractional NFT distribution doesn't exist");
        let mut token_distribution = token_distribution.unwrap();

        require(token_distribution.state == DistributionState::Returning, "Not in returning state");
        withdraw_fractional_nft(fractional_nft);

        token_distribution.state == DistributionState::Closed;
        storage.token_distributions.insert(fractional_nft, Option::Some(token_distribution));
    }

    #[storage(read, write)]
    fn create(
        buy_asset: ContractId,
        fractional_nft: ContractId,
        nft: ContractId,
        reserve_price: u64,
        token_price: u64,
        token_supply: u64,
        token_id: u64,
    ) {
        create_fractional_nft(fractional_nft, nft, msg_sender().unwrap(), token_supply, token_id);

        storage.token_distributions.insert(fractional_nft, Option::Some(TokenDistribution::new(buy_asset, nft, reserve_price, token_id, token_price)));
    }

    #[storage(read, write)]
    fn purchase(amount: u64, fractional_nft: ContractId) {
        let token_distribution = storage.token_distributions.get(fractional_nft);
        require(token_distribution.is_some(), "Fractional NFT distribution doesn't exist");
        let mut token_distribution = token_distribution.unwrap();

        require(token_distribution.state == DistributionState::Created || token_distribution.state == DistributionState::Distributing, "Not available");
        if token_distribution.state == DistributionState::Created {
            token_distribution.state = DistributionState::Distributing;
            storage.token_distributions.insert(fractional_nft, Option::Some(token_distribution));
        }

        require(amount <= this_balance(fractional_nft), "Buying too many tokens");
        require(amount * token_distribution.token_price == msg_amount() && msg_asset_id() == token_distribution.buy_asset, "Incorrect asset");

        transfer(amount, fractional_nft, msg_sender().unwrap());
    }

    #[storage(read, write)]
    fn request_return(fractional_nft: ContractId) {
        
    }

    #[storage(read)]
    fn sell(fractional_nft: ContractId) {
        let token_distribution = storage.token_distributions.get(fractional_nft);
        require(token_distribution.is_some(), "Fractional NFT distribution doesn't exist");
        let token_distribution = token_distribution.unwrap();

        require(token_distribution.state == DistributionState::Returning, "Not available");
        require(msg_asset_id() == fractional_nft, "Incorrect Asset");

        transfer(msg_amount() * token_distribution.token_price, token_distribution.buy_asset, msg_sender().unwrap());
    }
}

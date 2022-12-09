contract;

dep data_structures;
dep interface;
dep utils;

use data_structures::{DistributionState, TokenDistribution};
use interface::TokenDistributor;
use std::{auth::msg_sender, storage::StorageMap};
use utils::create_fractional_nft;

storage {
    token_distributions: StorageMap<ContractId, TokenDistribution> = StorageMap {},
}

impl TokenDistributor for Contract {
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

        storage.token_distributions.insert(fractional_nft, TokenDistribution::new(buy_asset, nft, reserve_price, token_id, token_price));
    }
}

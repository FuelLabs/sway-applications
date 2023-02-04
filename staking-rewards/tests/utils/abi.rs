use core::fmt::Debug;
use fuels::{
    core::traits::Tokenizable,
    prelude::*,
    programs::{call_response::FuelCallResponse, contract::ContractCallHandler},
    types::{Identity, SizedAsciiString},
    // tx::UniqueIdentifier,
    // types::transaction_response::TransactionStatus,
};

use crate::utils::StakingRewards;

use super::STAKING_ASSET;

pub async fn balance_of(instance: &StakingRewards, id: &Identity) -> u64 {
    instance.methods().balance_of(id.to_owned()).call().await.unwrap().value
}

pub async fn stake(instance: &StakingRewards, amount: u64) -> FuelCallResponse<()> {
    instance
        .methods()
        .stake()
        .call_params(CallParameters {
            amount,
            asset_id: STAKING_ASSET,
            gas_forwarded: Some(1000000),
        })
        .call()
        .await
        .unwrap()
}
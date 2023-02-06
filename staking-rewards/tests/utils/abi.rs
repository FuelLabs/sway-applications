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
    instance
        .methods()
        .balance_of(id.to_owned())
        .call()
        .await
        .unwrap()
        .value
}

pub async fn earned(instance: &StakingRewards, wallet_identity: Identity) -> u64 {
    instance
        .methods()
        .earned(wallet_identity)
        .call()
        .await
        .unwrap()
        .value
}

pub async fn reward_per_token(instance: &StakingRewards) -> u64 {
    instance
        .methods()
        .reward_per_token()
        .call()
        .await
        .unwrap()
        .value
}

pub async fn reward_per_token_stored(instance: &StakingRewards) -> u64 {
    instance
        .methods()
        .reward_per_token_stored()
        .call()
        .await
        .unwrap()
        .value
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

pub async fn notify_reward_amount(instance: &StakingRewards, reward: u64) -> FuelCallResponse<()> {
    instance
        .methods()
        .notify_reward_amount(reward)
        .call()
        .await
        .unwrap()
}

pub async fn total_supply(instance: &StakingRewards) -> u64 {
    instance
        .methods()
        .total_supply()
        .call()
        .await
        .unwrap()
        .value
}

pub async fn get_reward(instance: &StakingRewards) -> FuelCallResponse<()> {
    instance.methods().get_reward().call().await.unwrap()
}

pub async fn exit(instance: &StakingRewards) -> FuelCallResponse<()> {
    instance
        .methods()
        .exit()
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap()
}

pub async fn reward_rate(instance: &StakingRewards) -> u64 {
    instance.methods().reward_rate().call().await.unwrap().value
}

pub async fn reward_duration(instance: &StakingRewards) -> u64 {
    instance
        .methods()
        .rewards_duration()
        .call()
        .await
        .unwrap()
        .value
}

pub async fn get_reward_for_duration(instance: &StakingRewards) -> u64 {
    instance
        .methods()
        .get_reward_for_duration()
        .call()
        .await
        .unwrap()
        .value
}

pub async fn period_finish(instance: &StakingRewards) -> u64 {
    instance
        .methods()
        .period_finish()
        .call()
        .await
        .unwrap()
        .value
}

pub async fn last_time_reward_applicable(instance: &StakingRewards) -> u64 {
    instance
        .methods()
        .last_time_reward_applicable()
        .call()
        .await
        .unwrap()
        .value
}
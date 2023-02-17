
use fuels::{types::{Identity, Address}, signers::provider::TimeParameters};
use chrono::{DateTime, Duration, Utc};

use crate::utils::{
    abi::{rewards, stake, notify_reward_amount, set_rewards_duration, exit},
    setup, INITIAL_STAKE, STAKING_ASSET,
};

#[tokio::test]
async fn can_get_rewards() {
    let (staking_contract, _id, wallet, _wallet2, _inittimestamp) = setup().await;
    let wallet_identity = Identity::Address(Address::from(wallet.address()));

    let reward = rewards(&staking_contract, wallet_identity.clone()).await;

    assert_eq!(reward, 0);

    set_rewards_duration(&staking_contract, 1).await;

    let time = TimeParameters { 
        start_time: DateTime::from(Utc::now()) + Duration::seconds(2),
        block_time_interval: Duration::seconds(2),
    };

    wallet.get_provider().unwrap().produce_blocks(2, Some(time)).await.unwrap();

    // notify_reward_amount(&staking_contract, 1000).await;

    // to update the rewards
    let _ = stake(&staking_contract, 1).await;
    let reward = rewards(&staking_contract, wallet_identity.clone()).await;

    assert_eq!(reward, 1000);
}
use chrono::{DateTime, Utc, Duration};
use fuels::prelude::*;
use fuels::types::Identity;

use crate::utils::{
    abi::{earned, notify_reward_amount, reward_per_token, reward_rate},
    setup, INITIAL_STAKE, ONE,
};

#[tokio::test]
async fn calculate_earned_tokens() {
    let (staking_contract, _id, wallet, _wallet2, inittimestamp) = setup().await;

    let time = TimeParameters { 
        start_time: DateTime::from(Utc::now()) + Duration::seconds(2),
        block_time_interval: Duration::seconds(2),
    };

    wallet.get_provider().unwrap().produce_blocks(2, Some(time)).await.unwrap();

    // let _ = notify_reward_amount(&staking_contract, 1000).await;
    let notify_reward_amount = staking_contract.methods().notify_reward_amount(1 * ONE).call().await.unwrap();
    println!("notify_reward_amount balance: {:?}", notify_reward_amount.get_logs().unwrap());

    let time = TimeParameters { 
        start_time: DateTime::from(Utc::now()) + Duration::seconds(2),
        block_time_interval: Duration::seconds(2),
    };
    wallet.get_provider().unwrap().produce_blocks(2, Some(time)).await.unwrap();

    let notify_reward_amount = staking_contract.methods().notify_reward_amount(1 * ONE).call().await.unwrap();
    println!("notify_reward_amount balance 2: {:?}", notify_reward_amount.get_logs().unwrap());
    let time = TimeParameters { 
        start_time: DateTime::from(Utc::now()) + Duration::seconds(2),
        block_time_interval: Duration::seconds(2),
    };
    wallet.get_provider().unwrap().produce_blocks(2, Some(time)).await.unwrap();

    let notify_reward_amount = staking_contract.methods().notify_reward_amount(1 * ONE).call().await.unwrap();
    println!("notify_reward_amount balance 2: {:?}", notify_reward_amount.get_logs().unwrap());
    // let reward_per_token = reward_per_token(&staking_contract).await;
    let reward_per_token = staking_contract.methods().reward_per_token().call().await.unwrap();

    println!("reward_per_token balance: {:?}", reward_per_token.get_logs().unwrap());
    // assert_eq!(reward_per_token, 0); // why is reward per token = 0? This assert should fail

    // Total accrued per token is time_elapsed * rate / total_supply
    let reward_per_token_timestamp = wallet
        .get_provider()
        .unwrap()
        .chain_info()
        .await
        .unwrap()
        .latest_block
        .header
        .time
        .unwrap()
        .timestamp() as u64;
    let expected_reward_per_token: u64 =
        ((reward_per_token_timestamp - inittimestamp) * reward_rate(&staking_contract).await * ONE) / INITIAL_STAKE;


    assert_eq!(reward_per_token.value, expected_reward_per_token);

    let wallet_identity = Identity::Address(Address::from(wallet.address()));
    let expected_reward = expected_reward_per_token * INITIAL_STAKE / ONE;

    let earned = earned(&staking_contract, wallet_identity).await;
    assert_eq!(earned, expected_reward);
}

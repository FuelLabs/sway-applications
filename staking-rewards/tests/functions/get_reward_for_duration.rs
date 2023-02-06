use crate::utils::{setup, abi::{get_reward_for_duration, reward_rate, reward_duration}};

#[tokio::test]
async fn can_get_reward_for_duration() {
    let (staking_contract, _id, _wallet, _wallet2, _inittimestamp) = setup().await;

    let reward_rate = reward_rate(&staking_contract).await;
    let reward_duration = reward_duration(&staking_contract).await;

    let expected_reward = reward_rate * reward_duration;
    let actual_reward = get_reward_for_duration(&staking_contract).await;

    assert_eq!(expected_reward, actual_reward);
}
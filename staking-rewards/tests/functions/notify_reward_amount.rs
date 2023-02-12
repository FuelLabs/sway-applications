use crate::utils::{
    abi::{get_reward_for_duration, notify_reward_amount},
    setup,
};

#[tokio::test]
async fn can_notify_reward_amount() {
    let (staking_contract, _id, _wallet, _wallet2, _inittimestamp) = setup().await;

    let rewardbefore = get_reward_for_duration(&staking_contract).await;
    notify_reward_amount(&staking_contract, 5000).await;
    let rewardafter = get_reward_for_duration(&staking_contract).await;

    assert_eq!(rewardbefore, 42000);
    assert_eq!(rewardafter, 5000);
}
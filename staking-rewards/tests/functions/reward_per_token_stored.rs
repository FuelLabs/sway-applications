use crate::utils::{
    abi::{notify_reward_amount, reward_per_token_stored, reward_per_token, total_supply, last_time_reward_applicable, last_update_time, reward_rate},
    setup, ONE,
};

// TODO: Investigate this test furthur
#[tokio::test]
async fn can_get_reward_per_token_stored() {
    let (staking_contract, _id, _wallet, _wallet2, _inittimestamp) = setup().await;

    let reward = reward_per_token_stored(&staking_contract).await;

    assert_eq!(reward, 0);

    notify_reward_amount(&staking_contract, 5000).await;

    let reward_per_token = reward_per_token(&staking_contract).await;
    
    let expected_reward = match total_supply(&staking_contract).await {
        0 => reward_per_token,
        _ => reward_per_token +  ((last_time_reward_applicable(&staking_contract).await - last_update_time(&staking_contract).await) * reward_rate(&staking_contract).await * ONE / total_supply(&staking_contract).await),
    };

    let reward = reward_per_token_stored(&staking_contract).await;

    assert_eq!(reward, expected_reward);
}
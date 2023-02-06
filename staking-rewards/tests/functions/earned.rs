use fuels::prelude::*;
use fuels::types::Identity;

use crate::utils::{
    abi::{earned, notify_reward_amount, reward_per_token},
    setup, INITIAL_STAKE, ONE,
};

#[tokio::test]
async fn calculate_earned_tokens() {
    let (staking_contract, _id, wallet, _wallet2, inittimestamp) = setup().await;

    staking_contract
        .methods()
        .reward_per_token()
        .call()
        .await
        .unwrap();

    let _ = notify_reward_amount(&staking_contract, 1000);
    let reward_per_token = reward_per_token(&staking_contract).await;
    assert_eq!(reward_per_token, 0);

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
        ((reward_per_token_timestamp - inittimestamp) * 42 * ONE) / INITIAL_STAKE;

    assert_eq!(reward_per_token, expected_reward_per_token);

    let wallet_identity = Identity::Address(Address::from(wallet.address()));
    let expected_reward = expected_reward_per_token * INITIAL_STAKE / ONE;

    let earned = earned(&staking_contract, wallet_identity).await;
    assert_eq!(earned, expected_reward);
}

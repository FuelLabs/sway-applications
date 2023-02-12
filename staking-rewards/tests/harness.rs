mod functions;
mod utils;

use fuels::{prelude::*, signers::provider};
// use utils::{
//     balance_of, earned, exit, get_balance, get_reward, get_reward_for_duration,
//     last_time_reward_applicable, last_update_time, notify_reward_amount, owner, period_finish,
//     recover_tokens, reward_duration, reward_per_token, reward_per_token_paid,
//     reward_per_token_stored, reward_rate, rewards, rewards_distribution, rewards_duration,
//     rewards_token, set_rewards_duration, setup, stake, staking_token, total_supply, withdraw, ONE,
//     RANDOM_ASSET, REWARDS_ASSET, STAKING_ASSET,
// };
use utils::{setup, ONE};

// Until timestamp supported in Sways of each action must be specified. Contract is deployed at t=0

// #[tokio::test]
// async fn constructed() {
//     let (staking_contract, _id, wallet, _wallet2, _inittimestamp) = setup().await;
//     let wallet_identity = Identity::Address(Address::from(wallet.address()));

//     let owner_identity = owner(&staking_contract).await;
//     assert_eq!(wallet_identity, owner_identity.0.value);
// }








// #[tokio::test]
// async fn can_get_rewards_distribution() {
//     let (staking_contract, _id, wallet, _wallet2, _inittimestamp) = setup().await;
//     let wallet_identity = Identity::Address(Address::from(wallet.address()));

//     let rewards_distribution = rewards_distribution(&staking_contract).await;

//     assert_eq!(wallet_identity, rewards_distribution.0.value);
// }

// #[tokio::test]
// async fn can_get_rewards_duration() {
//     let (staking_contract, _id, _wallet, _wallet2, _inittimestamp) = setup().await;

//     let rewards_dur = rewards_duration(&staking_contract).await;

//     assert_eq!(1000, rewards_dur.0.value);
// }

// #[tokio::test]
// async fn can_get_rewards_token() {
//     let (staking_contract, _id, _wallet, _wallet2, _inittimestamp) = setup().await;

//     let token = rewards_token(&staking_contract).await;

//     assert_eq!(ContractId::new([2_u8; 32]), token.0.value);
// }

// #[tokio::test]
// async fn can_set_rewards_duration() {
//     let (staking_contract, _id, _wallet, _wallet2, _inittimestamp) = setup().await;

//     let old_rewards_dur = rewards_duration(&staking_contract).await;

//     assert_eq!(old_rewards_dur.0.value, 1000);

//     set_rewards_duration(&staking_contract, 2000).await;

//     let new_rewards_dur = rewards_duration(&staking_contract).await;

//     assert_eq!(2000, new_rewards_dur.0.value);
// }

// #[tokio::test]
// async fn can_get_staking_token() {
//     let (staking_contract, _id, _wallet, _wallet2, _inittimestamp) = setup().await;

//     let token = staking_token(&staking_contract).await;

//     assert_eq!(ContractId::new([1_u8; 32]), token.0.value);
// }

// #[tokio::test]
// async fn can_withdraw() {
//     let (staking_contract, _id, wallet, _wallet2, _inittimestamp) = setup().await;

//     let staking_balance_before = get_balance(&wallet, STAKING_ASSET).await;

//     withdraw(&staking_contract, 500).await;

//     let staking_balance_after = get_balance(&wallet, STAKING_ASSET).await;

//     assert_eq!(staking_balance_before + 500, staking_balance_after);
// }

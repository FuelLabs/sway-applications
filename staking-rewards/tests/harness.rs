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
// async fn can_get_last_time_reward_applicable() {
//     let (staking_contract, _id, _wallet, _wallet2, _inittimestamp) = setup().await;

//     let period_finish = period_finish(&staking_contract).await;
//     let expected = std::cmp::min(period_finish.1, period_finish.0.value);
//     let actual = last_time_reward_applicable(&staking_contract).await;

//     assert_eq!(actual.0.value, expected);
// }

// #[tokio::test]
// async fn can_get_last_updated() {
//     let (staking_contract, _id, _wallet, _wallet2, _inittimestamp) = setup().await;

//     let reciepts = notify_reward_amount(&staking_contract, 5000).await;
//     let last_updated = last_update_time(&staking_contract).await;

//     assert_eq!(last_updated.0.value, reciepts.1);
// }

// #[tokio::test]
// async fn can_notify_reward_amount() {
//     let (staking_contract, _id, _wallet, _wallet2, _inittimestamp) = setup().await;

//     let rewardbefore = get_reward_for_duration(&staking_contract).await;
//     notify_reward_amount(&staking_contract, 5000).await;
//     let rewardafter = get_reward_for_duration(&staking_contract).await;

//     assert_eq!(rewardbefore.0.value, 42000);
//     assert_eq!(rewardafter.0.value, 5000);
// }

// #[tokio::test]
// async fn can_get_owner() {
//     let (staking_contract, _id, wallet, _wallet2, _inittimestamp) = setup().await;

//     let actualowner = owner(&staking_contract).await;
//     let expectedowner = Identity::Address(Address::from(wallet.address()));

//     assert_eq!(actualowner.0.value, expectedowner);
// }

// #[tokio::test]
// async fn can_get_period_finish() {
//     let (staking_contract, _id, _wallet, _wallet2, _inittimestamp) = setup().await;

//     let period_finish = period_finish(&staking_contract).await;

//     assert_eq!(period_finish.0.value, 1000);
// }

// #[tokio::test]
// async fn can_recover_tokens() {
//     let (staking_contract, id, wallet, wallet2, _inittimestamp) = setup().await;

//     let _receipt = wallet2
//         .force_transfer_to_contract(&id, 50000, RANDOM_ASSET, TxParameters::default())
//         .await
//         .unwrap();

//     let owner_balance_before = get_balance(&wallet, RANDOM_ASSET).await;

//     recover_tokens(&staking_contract, ContractId::new([3u8; 32]), 50000).await;

//     let owner_balance_after = get_balance(&wallet, RANDOM_ASSET).await;

//     assert_eq!(owner_balance_before + 50000, owner_balance_after);
// }

// #[tokio::test]
// async fn can_get_reward_per_token_stored() {
//     let (staking_contract, _id, _wallet, _wallet2, _inittimestamp) = setup().await;

//     let reward = reward_per_token_stored(&staking_contract).await;

//     assert_eq!(reward.0.value, 0);

//     notify_reward_amount(&staking_contract, 5000).await;

//     let reward = reward_per_token_stored(&staking_contract).await;

//     assert_eq!(reward.0.value, 516);
// }

// #[tokio::test]
// async fn can_get_reward_per_token_paid() {
//     let (staking_contract, _id, wallet, _wallet2, _inittimestamp) = setup().await;
//     let wallet_identity = Identity::Address(Address::from(wallet.address()));

//     let reward = reward_per_token_paid(&staking_contract, wallet_identity.clone()).await;

//     assert_eq!(reward.0.value, 0);

//     notify_reward_amount(&staking_contract, 5000).await;

//     let reward = reward_per_token_paid(&staking_contract, wallet_identity.clone()).await;

//     assert_eq!(reward.0.value, 516);
// }

// #[tokio::test]
// async fn can_get_reward() {
//     let (staking_contract, _id, wallet, _wallet2, _inittimestamp) = setup().await;
//     let wallet_identity = Identity::Address(Address::from(wallet.address()));

//     let reward = rewards(&staking_contract, wallet_identity.clone()).await;

//     assert_eq!(reward.0.value, 0);

//     set_rewards_duration(&staking_contract, 1).await;
//     notify_reward_amount(&staking_contract, 1000).await;

//     std::thread::sleep(std::time::Duration::new(2, 0));

//     // to update the rewards
//     let _ = stake(&staking_contract, 1).await;
//     let reward = rewards(&staking_contract, wallet_identity.clone()).await;

//     assert_eq!(reward.0.value, 1000);
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

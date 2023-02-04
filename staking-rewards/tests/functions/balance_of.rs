use fuels::types::Identity;
use fuels::prelude::*;

use crate::utils::abi::stake;
use crate::utils::{setup, ONE, abi::balance_of};

const INITIAL_STAKE: u64 = 10 * ONE;

// #[tokio::test]
// async fn can_get_balance_of() {
//     let (staking_contract, _id, wallet, _wallet2, _inittimestamp) = setup().await;

//     // User balance has updated
//     let wallet_identity = Identity::Address(Address::from(wallet.address()));
//     let user_balance = balance_of(&staking_contract, &wallet_identity).await;
//     assert_eq!(user_balance.0.value, INITIAL_STAKE);

//     // User balance updates again
//     stake(&staking_contract, 50000).await;
//     let user_balance = balance_of(&staking_contract, &wallet_identity).await;
//     assert_eq!(user_balance.0.value, INITIAL_STAKE + 50000);
// }

#[tokio::test]
async fn can_get_balance_of() {
    let (staking_contract, _id, wallet, _wallet2, _inittimestamp) = setup().await;

    // User balance has updated
    let wallet_identity = Identity::Address(Address::from(wallet.address()));
    let user_balance = balance_of(&staking_contract, &wallet_identity).await;
    assert_eq!(user_balance, INITIAL_STAKE);

    // User balance updates again
    stake(&staking_contract, 50000).await;
    let user_balance = balance_of(&staking_contract, &wallet_identity).await;
    assert_eq!(user_balance, INITIAL_STAKE + 50000);
}
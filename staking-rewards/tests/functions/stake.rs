use fuels::prelude::*;
use fuels::types::Identity;

use crate::utils::{
    abi::{balance_of, stake, total_supply},
    setup, INITIAL_STAKE,
};

#[tokio::test]
async fn stake_tokens() {
    let (staking_contract, _id, wallet, _wallet2, _inittimestamp) = setup().await;

    // User balance has updated
    let wallet_identity = Identity::Address(Address::from(wallet.address()));
    let user_balance = balance_of(&staking_contract, &wallet_identity).await;
    assert_eq!(user_balance, INITIAL_STAKE);

    // Total_supply has updated
    let total_supply = total_supply(&staking_contract).await;
    assert_eq!(total_supply, INITIAL_STAKE);
}

#[tokio::test]
async fn stake_tokens_twice() {
    let (staking_contract, _id, wallet, _wallet2, _inittimestamp) = setup().await;

    // User balance has updated
    let wallet_identity = Identity::Address(Address::from(wallet.address()));
    let user_balance = balance_of(&staking_contract, &wallet_identity).await;
    assert_eq!(user_balance, INITIAL_STAKE);

    // Total_supply has updated
    let total_supply1 = total_supply(&staking_contract).await;
    assert_eq!(total_supply1, INITIAL_STAKE);

    // User stakes again
    let _receipt = stake(&staking_contract, INITIAL_STAKE).await;

    // User balance has updated
    let user_balance = balance_of(&staking_contract, &wallet_identity).await;
    assert_eq!(user_balance, INITIAL_STAKE * 2);

    // Total_supply has updated
    let total_supply2 = total_supply(&staking_contract).await;
    assert_eq!(total_supply2, INITIAL_STAKE * 2);
}
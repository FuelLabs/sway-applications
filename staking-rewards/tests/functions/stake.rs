use fuels::types::Identity;
use fuels::prelude::*;

use crate::utils::{setup, INITIAL_STAKE, abi::{balance_of, stake, total_supply}};

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
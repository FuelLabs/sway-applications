use fuels::types::{Identity, Address};
use crate::utils::{
    abi::owner,
    setup,
};

#[tokio::test]
async fn can_get_owner() {
    let (staking_contract, _id, wallet, _wallet2, _inittimestamp) = setup().await;

    let actualowner = owner(&staking_contract).await;
    let expectedowner = Identity::Address(Address::from(wallet.address()));

    assert_eq!(actualowner, expectedowner);
}
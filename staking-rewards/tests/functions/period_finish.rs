use crate::utils::{
    abi::period_finish,
    setup,
};

#[tokio::test]
async fn can_get_period_finish() {
    let (staking_contract, _id, _wallet, _wallet2, _inittimestamp) = setup().await;

    let period_finish = period_finish(&staking_contract).await;

    assert_eq!(period_finish, 1000);
}
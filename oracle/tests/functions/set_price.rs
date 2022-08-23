use crate::utils::{
    abi_calls::{price, set_price},
    test_helpers::setup,
};

mod success {
    use super::*;

    #[tokio::test]
    async fn can_set_price() {
        let (user, _) = setup().await;
        let set_price_amount: u64 = 1000;
        set_price(&user.oracle, set_price_amount).await;
        let price = price(&user.oracle).await;
        assert_eq!(price, set_price_amount);
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic]
    async fn panics_when_not_owner() {
        let (user, wallets) = setup().await;
        user.oracle
            ._with_wallet(wallets[1].clone())
            .unwrap()
            .set_price(1000)
            .call()
            .await
            .unwrap();
    }
}

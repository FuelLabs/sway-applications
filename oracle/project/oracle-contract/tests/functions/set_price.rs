use utils::{
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
    #[should_panic(expected = "Revert(18446744073709486080)")]
    async fn when_not_owner() {
        let (user, wallets) = setup().await;
        user.oracle
            .with_wallet(wallets[1].clone())
            .unwrap()
            .methods()
            .set_price(1000)
            .call()
            .await
            .unwrap();
    }
}

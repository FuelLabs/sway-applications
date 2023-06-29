use utils::{
    abi_calls::{price, set_price},
    test_helpers::setup,
};

mod success {
    use super::*;
    use utils::PriceUpdateEvent;

    #[tokio::test]
    async fn can_set_price() {
        let (user, _) = setup().await;
        let set_price_amount: u64 = 1000;

        let response = set_price(&user.oracle, set_price_amount).await;
        let price = price(&user.oracle).await;

        let log = response
            .decode_logs_with_type::<PriceUpdateEvent>()
            .unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(
            *event,
            PriceUpdateEvent {
                price: set_price_amount
            }
        );
        assert_eq!(price, Some(set_price_amount));
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "NotOwner")]
    async fn when_not_owner() {
        let (user, wallets) = setup().await;
        user.oracle
            .with_account(wallets[1].clone())
            .unwrap()
            .methods()
            .set_price(1000)
            .call()
            .await
            .unwrap();
    }
}

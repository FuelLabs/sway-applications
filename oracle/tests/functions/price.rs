use crate::utils::{
    abi_calls::{price, set_price},
    test_helpers::setup,
    Option,
};

mod success {
    use super::*;

    #[tokio::test]
    async fn can_get_price() {
        let (user, _) = setup().await;
        let set_price_amount: u64 = 1000;
        set_price(&user.oracle, Option::Some(set_price_amount)).await;
        let price = price(&user.oracle).await.value;
        assert_eq!(price, Option::Some(set_price_amount));
    }

    #[tokio::test]
    async fn can_get_price_when_not_initialized() {
        let (user, _) = setup().await;
        let price = price(&user.oracle).await.value;
        assert_eq!(price, Option::None());
    }
}

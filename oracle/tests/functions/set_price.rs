use fuels::{prelude::*, tx::Address};
use crate::utils::{
    abi_calls::{constructor, price, set_price},
    test_helpers::setup,
    Identity,
};

mod success {
    use super::*;

    #[tokio::test]
    async fn can_set_price() {
        let user = setup().await;
        constructor(&user.oracle, Identity::Address(Address::from(user.wallet.address()))).await;
        let set_price_amount = 1000;
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
        let user = setup().await;
        constructor(&user.oracle, Identity::Address(Address::from(user.wallet.address()))).await;
        let not_owner = launch_provider_and_get_wallet().await;
        user.oracle._with_wallet(not_owner).unwrap().set_price(1000).call().await.unwrap();
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_not_initialized() {
        let user = setup().await;
        set_price(&user.oracle, 1000).await;
    }
}
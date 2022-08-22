use crate::utils::{
    abi_calls::{constructor, price, set_price},
    test_helpers::setup,
    Identity,
};
use fuels::{prelude::*, tx::Address};

mod success {
    use super::*;

    #[tokio::test]
    async fn can_get_price() {
        let user = setup().await;
        constructor(
            &user.oracle,
            Identity::Address(Address::from(user.wallet.address())),
        )
        .await;
        let set_price_amount = 1000;
        set_price(&user.oracle, set_price_amount).await;
        let price = price(&user.oracle).await;
        assert_eq!(price, set_price_amount);
    }
}

mod success {
    use crate::utils::{
        abi::{expiry, extend, register},
        setup, EXTEND_DURATION, REGISTER_DURATION,
    };
    use fuels::prelude::*;

    #[tokio::test]
    async fn can_get_expiry() {
        let (instance, _id, wallet, _wallet2) = setup().await;
        let wallet_identity = Identity::Address(Address::from(wallet.address()));
        let name = String::from("SwaySway");

        register(
            &instance,
            &name,
            REGISTER_DURATION,
            &wallet_identity,
            &wallet_identity,
        )
        .await;
        let previous_expiry_response = expiry(&instance, &name).await;

        extend(&instance, &name, EXTEND_DURATION).await;

        let new_expiry_response = expiry(&instance, &name).await;

        assert_eq!(
            previous_expiry_response.0.value.unwrap() + EXTEND_DURATION,
            new_expiry_response.0.value.unwrap()
        );
    }
}

mod revert {
    use crate::utils::{abi::expiry, setup};

    #[tokio::test]
    #[should_panic(expected = "`Result::unwrap()` on an `Err` value")]
    async fn cant_get_expiry() {
        let (instance, _id, _wallet, _wallet2) = setup().await;
        let name = String::from("SwaySway");

        let expiry = expiry(&instance, &name).await;
        expiry.0.value.unwrap();
    }
}

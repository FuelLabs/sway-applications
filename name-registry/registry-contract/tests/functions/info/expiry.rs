use crate::utils::{interface::info::expiry, setup::setup};

mod success {
    use super::*;
    use crate::utils::{
        interface::core::{extend, register, set_asset},
        setup::{EXTEND_DURATION, REGISTER_DURATION},
    };
    use fuels::prelude::AssetId;

    #[tokio::test]
    async fn can_get_expiry() {
        let (instance, acc, _wallet2) = setup().await;
        set_asset(&instance, AssetId::default(), Some(1)).await;

        register(
            &instance,
            acc.name.clone(),
            REGISTER_DURATION,
            &acc.identity(),
            &acc.identity(),
            AssetId::default(),
        )
        .await;
        let previous_expiry_response = expiry(&instance, acc.name.clone()).await;

        extend(
            &instance,
            acc.name.clone(),
            EXTEND_DURATION,
            AssetId::default(),
        )
        .await;

        let new_expiry_response = expiry(&instance, acc.name.clone()).await;

        assert_eq!(
            previous_expiry_response.value.unwrap() + EXTEND_DURATION,
            new_expiry_response.value.unwrap()
        );
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "NameNotRegistered")]
    async fn cant_get_expiry() {
        let (instance, acc, _wallet2) = setup().await;

        let expiry = expiry(&instance, acc.name.clone()).await;
        expiry.value.unwrap();
    }
}

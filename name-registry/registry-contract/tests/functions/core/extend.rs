use crate::utils::{
    interface::core::{register, set_asset},
    setup::{setup, EXTEND_DURATION, REGISTER_DURATION},
};
use fuels::prelude::AssetId;

mod success {
    use super::*;
    use crate::utils::{
        interface::{core::extend_with_time, info::expiry},
        setup::RegistrationExtendedEvent,
    };

    #[tokio::test]
    #[ignore]
    async fn can_extend() {
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

        let previous_expiry = expiry(&instance, acc.name.clone()).await;

        // TODO: Breaking changes by SDK prevent retention of time
        let (extend_response, latest_block_time) = extend_with_time(
            &instance,
            acc.name.clone(),
            EXTEND_DURATION,
            AssetId::default(),
        )
        .await;
        let log = extend_response
            .decode_logs_with_type::<RegistrationExtendedEvent>()
            .unwrap();

        let new_expiry = expiry(&instance, acc.name.clone()).await;

        assert_eq!(
            previous_expiry.value.unwrap() + EXTEND_DURATION,
            new_expiry.value.unwrap()
        );
        assert_eq!(
            log,
            vec![RegistrationExtendedEvent {
                duration: EXTEND_DURATION,
                name: acc.name.clone(),
                new_expiry: latest_block_time + REGISTER_DURATION + EXTEND_DURATION
            }]
        );
    }
}

mod revert {
    use super::*;
    use crate::utils::interface::core::extend;

    // TODO: missing test for incorrect asset

    #[tokio::test]
    #[should_panic(expected = "InsufficientPayment")]
    async fn cant_extend_insufficient_payment() {
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

        extend(&instance, acc.name.clone(), u64::MAX, AssetId::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "NameNotRegistered")]
    async fn cant_extend_name_not_registered() {
        let (instance, acc, _wallet2) = setup().await;
        set_asset(&instance, AssetId::default(), Some(1)).await;
        extend(
            &instance,
            acc.name.clone(),
            EXTEND_DURATION,
            AssetId::default(),
        )
        .await;
    }
}

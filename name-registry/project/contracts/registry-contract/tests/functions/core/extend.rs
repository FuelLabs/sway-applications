use crate::utils::{
    interface::core::register,
    setup::{setup, EXTEND_DURATION, REGISTER_DURATION},
};

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

        register(
            &instance,
            &acc.name,
            REGISTER_DURATION,
            &acc.identity(),
            &acc.identity(),
        )
        .await;

        let previous_expiry = expiry(&instance, &acc.name).await;

        // TODO: Breaking changes by SDK prevent retention of time
        let (extend_response, latest_block_time) =
            extend_with_time(&instance, &acc.name, EXTEND_DURATION).await;
        let log = extend_response
            .decode_logs_with_type::<RegistrationExtendedEvent>()
            .unwrap();

        let new_expiry = expiry(&instance, &acc.name).await;

        assert_eq!(
            previous_expiry.value.unwrap() + EXTEND_DURATION,
            new_expiry.value.unwrap()
        );
        assert_eq!(
            log,
            vec![RegistrationExtendedEvent {
                duration: EXTEND_DURATION,
                name: acc.name,
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

        register(
            &instance,
            &acc.name,
            REGISTER_DURATION,
            &acc.identity(),
            &acc.identity(),
        )
        .await;

        extend(&instance, &acc.name, u64::MAX).await;
    }

    #[tokio::test]
    #[should_panic(expected = "NameNotRegistered")]
    async fn cant_extend_name_not_registered() {
        let (instance, acc, _wallet2) = setup().await;

        extend(&instance, &acc.name, EXTEND_DURATION).await;
    }
}

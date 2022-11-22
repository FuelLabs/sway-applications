mod success {
    use crate::utils::{
        abi::{expiry, extend, register},
        setup, string_to_ascii, RegistrationExtendedEvent, EXTEND_DURATION, REGISTER_DURATION,
    };

    #[tokio::test]
    async fn can_extend() {
        let (instance, acc, _wallet2) = setup().await;

        let (_, register_time) = register(
            &instance,
            &acc.name,
            REGISTER_DURATION,
            &acc.identity(),
            &acc.identity(),
        )
        .await;

        let previous_expiry = expiry(&instance, &acc.name).await;

        let extend_response = extend(&instance, &acc.name, EXTEND_DURATION).await;
        let log = instance
            .logs_with_type::<RegistrationExtendedEvent>(&extend_response.0.receipts)
            .unwrap();

        let new_expiry = expiry(&instance, &acc.name).await;

        assert_eq!(
            previous_expiry.0.value.unwrap() + EXTEND_DURATION,
            new_expiry.0.value.unwrap()
        );
        assert_eq!(
            log,
            vec![RegistrationExtendedEvent {
                duration: EXTEND_DURATION,
                name: string_to_ascii(&acc.name),
                new_expiry: register_time + REGISTER_DURATION + EXTEND_DURATION
            }]
        );
    }
}

mod revert {
    use crate::utils::{
        abi::{extend, register},
        setup, EXTEND_DURATION, REGISTER_DURATION,
    };

    #[tokio::test]
    #[should_panic]
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
    #[should_panic(expected = "Revert(18446744073709486080)")]
    async fn cant_extend_name_not_registered() {
        let (instance, acc, _wallet2) = setup().await;

        extend(&instance, &acc.name, EXTEND_DURATION).await;
    }
}

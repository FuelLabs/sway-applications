mod success {
    use crate::utils::{
        abi::register, setup, string_to_ascii, NameRegisteredEvent, REGISTER_DURATION,
    };

    #[tokio::test]
    async fn can_register() {
        let (instance, acc, _wallet2) = setup().await;

        let response = register(
            &instance,
            &acc.name,
            REGISTER_DURATION,
            &acc.identity(),
            &acc.identity(),
        )
        .await;
        let log = instance
            .logs_with_type::<NameRegisteredEvent>(&response.0.receipts)
            .unwrap();

        assert_eq!(
            log,
            vec![NameRegisteredEvent {
                expiry: response.1 + REGISTER_DURATION,
                name: string_to_ascii(&acc.name),
                owner: acc.identity().clone(),
                identity: acc.identity()
            }]
        )
    }
}

mod revert {
    use crate::utils::{abi::register, setup, REGISTER_DURATION};

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn cant_repeat_register() {
        let (instance, acc, _wallet2) = setup().await;

        register(
            &instance,
            &acc.name,
            REGISTER_DURATION,
            &acc.identity(),
            &acc.identity(),
        )
        .await;
        register(
            &instance,
            &acc.name,
            REGISTER_DURATION,
            &acc.identity(),
            &acc.identity(),
        )
        .await;
    }

    #[tokio::test]
    #[should_panic]
    async fn cant_register_max_duration() {
        let (instance, acc, _wallet2) = setup().await;

        register(
            &instance,
            &acc.name,
            u64::MAX,
            &acc.identity(),
            &acc.identity(),
        )
        .await;
    }
}

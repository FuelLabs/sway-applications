use crate::utils::setup::{setup, REGISTER_DURATION};

mod success {
    use super::*;
    use crate::utils::{interface::core::register_with_time, setup::NameRegisteredEvent};

    #[tokio::test]
    #[ignore]
    async fn can_register() {
        let (instance, acc, _wallet2) = setup().await;

        // TODO: Breaking changes by SDK prevent retention of time
        let (response, latest_block_time) = register_with_time(
            &instance,
            &acc.name,
            REGISTER_DURATION,
            &acc.identity(),
            &acc.identity(),
        )
        .await;

        let log = response
            .decode_logs_with_type::<NameRegisteredEvent>()
            .unwrap();

        assert_eq!(
            log,
            vec![NameRegisteredEvent {
                expiry: latest_block_time + REGISTER_DURATION,
                name: acc.name,
                owner: acc.identity(),
                identity: acc.identity()
            }]
        )
    }
}

mod revert {
    use super::*;
    use crate::utils::interface::core::register;

    // TODO: missing test

    #[tokio::test]
    #[should_panic(expected = "NameNotExpired")]
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
    #[should_panic(expected = "InsufficientPayment")]
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

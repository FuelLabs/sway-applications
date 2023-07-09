use crate::utils::{
    interface::core::set_asset,
    setup::{setup, REGISTER_DURATION},
};
use fuels::prelude::ContractId;

mod success {
    use super::*;
    use crate::utils::{
        interface::core::register_with_time,
        setup::{string_to_ascii, NameRegisteredEvent},
    };

    #[tokio::test]
    #[ignore]
    async fn can_register() {
        let (instance, acc, _wallet2) = setup().await;
        set_asset(&instance, ContractId::zeroed(), Some(1)).await;

        // TODO: Breaking changes by SDK prevent retention of time
        let (response, latest_block_time) = register_with_time(
            &instance,
            &acc.name,
            REGISTER_DURATION,
            &acc.identity(),
            &acc.identity(),
            ContractId::zeroed(),
        )
        .await;

        let log = response
            .decode_logs_with_type::<NameRegisteredEvent>()
            .unwrap();

        assert_eq!(
            log,
            vec![NameRegisteredEvent {
                expiry: latest_block_time + REGISTER_DURATION,
                name: string_to_ascii(&acc.name),
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
        set_asset(&instance, ContractId::zeroed(), Some(1)).await;

        register(
            &instance,
            &acc.name,
            REGISTER_DURATION,
            &acc.identity(),
            &acc.identity(),
            ContractId::zeroed(),
        )
        .await;
        register(
            &instance,
            &acc.name,
            REGISTER_DURATION,
            &acc.identity(),
            &acc.identity(),
            ContractId::zeroed(),
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "InsufficientPayment")]
    async fn cant_register_max_duration() {
        let (instance, acc, _wallet2) = setup().await;
        set_asset(&instance, ContractId::zeroed(), Some(1)).await;

        register(
            &instance,
            &acc.name,
            u64::MAX,
            &acc.identity(),
            &acc.identity(),
            ContractId::zeroed(),
        )
        .await;
    }
}

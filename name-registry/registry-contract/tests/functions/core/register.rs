use crate::utils::{
    interface::core::set_asset,
    setup::{setup, REGISTER_DURATION},
};
use fuels::prelude::AssetId;

mod success {
    use super::*;
    use crate::utils::{interface::core::register_with_time, setup::NameRegisteredEvent};

    #[tokio::test]
    #[ignore]
    async fn can_register() {
        let (instance, acc, _wallet2) = setup().await;
        set_asset(&instance, AssetId::default(), Some(1)).await;

        // TODO: Breaking changes by SDK prevent retention of time
        let (response, latest_block_time) = register_with_time(
            &instance,
            acc.name.clone(),
            REGISTER_DURATION,
            &acc.identity(),
            &acc.identity(),
            AssetId::default(),
        )
        .await;

        let log = response
            .decode_logs_with_type::<NameRegisteredEvent>()
            .unwrap();

        assert_eq!(
            log,
            vec![NameRegisteredEvent {
                expiry: latest_block_time + REGISTER_DURATION,
                name: acc.name.clone(),
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
        register(
            &instance,
            acc.name.clone(),
            REGISTER_DURATION,
            &acc.identity(),
            &acc.identity(),
            AssetId::default(),
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "InsufficientPayment")]
    async fn cant_register_max_duration() {
        let (instance, acc, _wallet2) = setup().await;
        set_asset(&instance, AssetId::default(), Some(1)).await;

        register(
            &instance,
            acc.name.clone(),
            u64::MAX,
            &acc.identity(),
            &acc.identity(),
            AssetId::default(),
        )
        .await;
    }
}

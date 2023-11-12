use crate::utils::{
    interface::core::{register, set_asset, set_identity},
    setup::{setup, REGISTER_DURATION},
};
use fuels::{
    prelude::{Address, AssetId},
    types::Identity,
};

mod success {
    use super::*;
    use crate::utils::{
        interface::info::identity,
        setup::{string_to_ascii, IdentityChangedEvent},
    };

    #[tokio::test]
    async fn can_set_identity() {
        let (instance, acc1, wallet2) = setup().await;
        let wallet_identity2 = Identity::Address(Address::from(wallet2.address()));
        set_asset(&instance, AssetId::zeroed(), Some(1)).await;

        register(
            &instance,
            &acc1.name,
            REGISTER_DURATION,
            &acc1.identity(),
            &acc1.identity(),
            AssetId::zeroed(),
        )
        .await;

        let previous_identity = identity(&instance, &acc1.name).await;

        assert_eq!(previous_identity.value.unwrap(), acc1.identity(),);

        let response = set_identity(&instance, &acc1.name, wallet_identity2.clone()).await;

        let new_identity = identity(&instance, &acc1.name).await;

        assert_eq!(new_identity.value.unwrap(), wallet_identity2);

        let log = response
            .decode_logs_with_type::<IdentityChangedEvent>()
            .unwrap();
        assert_eq!(
            log,
            vec![IdentityChangedEvent {
                name: string_to_ascii(&acc1.name),
                new_identity: wallet_identity2,
                previous_identity: acc1.identity(),
            }]
        )
    }
}

mod revert {
    use super::*;
    use crate::utils::setup::{setup, REGISTER_DURATION};

    // TODO: missing tests

    #[tokio::test]
    #[should_panic(expected = "SenderNotOwner")]
    async fn cant_set_identity() {
        let (instance, acc1, wallet2) = setup().await;
        let wallet_identity2 = Identity::Address(Address::from(wallet2.address()));
        set_asset(&instance, AssetId::zeroed(), Some(1)).await;

        register(
            &instance,
            &acc1.name,
            REGISTER_DURATION,
            &acc1.identity(),
            &acc1.identity(),
            AssetId::zeroed(),
        )
        .await;

        set_identity(
            &instance.with_account(wallet2).unwrap(),
            &acc1.name,
            wallet_identity2.clone(),
        )
        .await;
    }
}

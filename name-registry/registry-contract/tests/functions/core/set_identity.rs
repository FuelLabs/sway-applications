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
    use crate::utils::interface::info::identity;

    #[tokio::test]
    async fn can_set_identity() {
        let (instance, acc1, wallet2) = setup().await;
        let wallet_identity2 = Identity::Address(Address::from(wallet2.address()));
        set_asset(&instance, AssetId::default(), Some(1)).await;

        register(
            &instance,
            acc1.name.clone(),
            REGISTER_DURATION,
            &acc1.identity(),
            &acc1.identity(),
            AssetId::default(),
        )
        .await;

        let previous_identity = identity(&instance, acc1.name.clone()).await;

        assert_eq!(previous_identity.value.unwrap(), acc1.identity(),);

        let _response = set_identity(&instance, acc1.name.clone(), wallet_identity2).await;

        let new_identity = identity(&instance, acc1.name.clone()).await;

        assert_eq!(new_identity.value.unwrap(), wallet_identity2);

        // TODO: Enable when nested heap types are supported.
        // let log = response
        //     .decode_logs_with_type::<IdentityChangedEvent>()
        //     .unwrap();
        // assert_eq!(
        //     log,
        //     vec![IdentityChangedEvent {
        //         name: acc1.name.clone(),
        //         new_identity: wallet_identity2,
        //         previous_identity: acc1.identity(),
        //     }]
        // )
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
        set_asset(&instance, AssetId::default(), Some(1)).await;

        register(
            &instance,
            acc1.name.clone(),
            REGISTER_DURATION,
            &acc1.identity(),
            &acc1.identity(),
            AssetId::default(),
        )
        .await;

        set_identity(
            &instance.with_account(wallet2),
            acc1.name.clone(),
            wallet_identity2,
        )
        .await;
    }
}

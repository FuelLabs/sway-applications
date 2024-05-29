use crate::utils::{
    interface::core::{register, set_asset, set_owner},
    setup::{setup, REGISTER_DURATION},
};
use fuels::{
    prelude::{Address, AssetId},
    types::Identity,
};

mod success {
    use super::*;
    use crate::utils::interface::info::owner;

    #[tokio::test]
    async fn can_set_owner() {
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

        let previous_owner = owner(&instance, acc1.name.clone()).await;

        assert_eq!(previous_owner.value.unwrap(), acc1.identity());

        let _response = set_owner(&instance, acc1.name.clone(), wallet_identity2).await;
        let new_owner = owner(&instance, acc1.name.clone()).await;

        assert_eq!(new_owner.value.unwrap(), wallet_identity2);

        // TODO: Enable when nested heap types are supported.
        // let log = response
        //     .decode_logs_with_type::<OwnerChangedEvent>()
        //     .unwrap();
        // assert_eq!(
        //     log,
        //     vec![OwnerChangedEvent {
        //         name: acc1.name.clone(),
        //         new_owner: wallet_identity2,
        //         previous_owner: acc1.identity()
        //     }]
        // )
    }
}

mod revert {
    use super::*;

    // TODO: missing tests

    #[tokio::test]
    #[should_panic(expected = "SenderNotOwner")]
    async fn cant_set_owner() {
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

        set_owner(
            &instance.with_account(wallet2),
            acc1.name.clone(),
            wallet_identity2,
        )
        .await;
    }
}

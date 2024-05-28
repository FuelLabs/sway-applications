use crate::utils::{interface::info::identity, setup::setup};

mod success {
    use super::*;
    use crate::utils::{
        interface::core::{register, set_asset, set_identity},
        setup::REGISTER_DURATION,
    };
    use fuels::{
        prelude::{Address, AssetId},
        types::Identity,
    };

    #[tokio::test]
    async fn can_get_identity() {
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

        set_identity(&instance, acc1.name.clone(), wallet_identity2).await;

        let new_identity = identity(&instance, acc1.name.clone()).await;

        assert_eq!(previous_identity.value.unwrap(), acc1.identity());
        assert_eq!(new_identity.value.unwrap(), wallet_identity2);
    }
}

mod revert {
    use super::*;

    // TODO: missing test

    #[tokio::test]
    #[should_panic(expected = "NameNotRegistered")]
    async fn cant_get_identity_when_not_registered() {
        let (instance, acc, _wallet2) = setup().await;

        let identity = identity(&instance, acc.name.clone()).await;
        identity.value.unwrap();
    }
}

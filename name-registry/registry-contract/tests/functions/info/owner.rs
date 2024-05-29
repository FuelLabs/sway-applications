use crate::utils::{interface::info::owner, setup::setup};

mod success {
    use super::*;
    use crate::utils::{
        interface::core::{register, set_asset, set_owner},
        setup::REGISTER_DURATION,
    };
    use fuels::{
        prelude::{Address, AssetId},
        types::Identity,
    };

    #[tokio::test]
    async fn can_get_owner() {
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

        set_owner(&instance, acc1.name.clone(), wallet_identity2).await;

        let new_owner = owner(&instance, acc1.name.clone()).await;

        assert_eq!(new_owner.value.unwrap(), wallet_identity2);
    }
}

mod revert {
    use super::*;

    // TODO: missing test

    #[tokio::test]
    #[should_panic(expected = "NameNotRegistered")]
    async fn cant_get_owner() {
        let (instance, acc, _wallet2) = setup().await;
        let owner = owner(&instance, acc.name.clone()).await;
        owner.value.unwrap();
    }
}

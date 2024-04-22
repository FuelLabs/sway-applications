use crate::utils::{
    interface::{constructor, name, set_name},
    setup::{defaults, setup},
};

mod success {

    use super::*;

    #[tokio::test]
    async fn sets_one_asset() {
        let (owner_wallet, other_wallet, id, instance_1, _instance_2) = setup().await;
        let (
            asset_id_1,
            _asset_id_2,
            _asset_id_3,
            _sub_id_1,
            _sub_id_2,
            _sub_id_3,
            owner_identity,
            _other_identity,
        ) = defaults(id, owner_wallet, other_wallet.clone());

        constructor(&instance_1, owner_identity.clone()).await;

        assert_eq!(name(&instance_1, asset_id_1).await, None);

        set_name(&instance_1, asset_id_1, String::from("Fuel NFT 1")).await;
        assert_eq!(
            name(&instance_1, asset_id_1).await,
            Some(String::from("Fuel NFT 1"))
        );
    }

    #[tokio::test]
    async fn sets_multiple_assets() {
        let (owner_wallet, other_wallet, id, instance_1, _instance_2) = setup().await;
        let (
            asset_id_1,
            asset_id_2,
            asset_id_3,
            _sub_id_1,
            _sub_id_2,
            _sub_id_3,
            owner_identity,
            _other_identity,
        ) = defaults(id, owner_wallet, other_wallet.clone());

        constructor(&instance_1, owner_identity.clone()).await;

        assert_eq!(name(&instance_1, asset_id_1).await, None);
        set_name(&instance_1, asset_id_1, String::from("Fuel NFT 1")).await;
        assert_eq!(
            name(&instance_1, asset_id_1).await,
            Some(String::from("Fuel NFT 1"))
        );

        assert_eq!(name(&instance_1, asset_id_2).await, None);
        set_name(&instance_1, asset_id_2, String::from("Fuel NFT 2")).await;
        assert_eq!(
            name(&instance_1, asset_id_2).await,
            Some(String::from("Fuel NFT 2"))
        );

        assert_eq!(name(&instance_1, asset_id_3).await, None);
        set_name(&instance_1, asset_id_3, String::from("Fuel NFT 3")).await;
        assert_eq!(
            name(&instance_1, asset_id_3).await,
            Some(String::from("Fuel NFT 3"))
        );
    }

    #[tokio::test]
    async fn does_not_overwrite_other_names() {
        let (owner_wallet, other_wallet, id, instance_1, _instance_2) = setup().await;
        let (
            asset_id_1,
            asset_id_2,
            asset_id_3,
            _sub_id_1,
            _sub_id_2,
            _sub_id_3,
            owner_identity,
            _other_identity,
        ) = defaults(id, owner_wallet, other_wallet.clone());

        constructor(&instance_1, owner_identity.clone()).await;

        assert_eq!(name(&instance_1, asset_id_1).await, None);
        set_name(&instance_1, asset_id_1, String::from("Fuel NFT 1")).await;
        assert_eq!(
            name(&instance_1, asset_id_1).await,
            Some(String::from("Fuel NFT 1"))
        );

        assert_eq!(name(&instance_1, asset_id_2).await, None);
        set_name(&instance_1, asset_id_2, String::from("Fuel NFT 2")).await;

        assert_eq!(
            name(&instance_1, asset_id_1).await,
            Some(String::from("Fuel NFT 1"))
        );
        assert_eq!(
            name(&instance_1, asset_id_2).await,
            Some(String::from("Fuel NFT 2"))
        );

        assert_eq!(name(&instance_1, asset_id_3).await, None);
        set_name(&instance_1, asset_id_3, String::from("Fuel NFT 3")).await;

        assert_eq!(
            name(&instance_1, asset_id_1).await,
            Some(String::from("Fuel NFT 1"))
        );
        assert_eq!(
            name(&instance_1, asset_id_2).await,
            Some(String::from("Fuel NFT 2"))
        );
        assert_eq!(
            name(&instance_1, asset_id_3).await,
            Some(String::from("Fuel NFT 3"))
        );
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "NotOwner")]
    async fn when_not_owner() {
        let (owner_wallet, other_wallet, id, instance_1, instance_2) = setup().await;
        let (
            asset_id_1,
            _asset_id_2,
            _asset_id_3,
            _sub_id_1,
            _sub_id_2,
            _sub_id_3,
            owner_identity,
            _other_identity,
        ) = defaults(id, owner_wallet, other_wallet.clone());

        constructor(&instance_1, owner_identity.clone()).await;

        set_name(&instance_2, asset_id_1, String::from("Fuel NFT 1")).await;
    }

    #[tokio::test]
    #[should_panic(expected = "ValueAlreadySet")]
    async fn when_a_name_has_already_been_set() {
        let (owner_wallet, other_wallet, id, instance_1, _instance_2) = setup().await;
        let (
            asset_id_1,
            _asset_id_2,
            _asset_id_3,
            _sub_id_1,
            _sub_id_2,
            _sub_id_3,
            owner_identity,
            _other_identity,
        ) = defaults(id, owner_wallet, other_wallet.clone());

        constructor(&instance_1, owner_identity.clone()).await;

        set_name(&instance_1, asset_id_1, String::from("Fuel NFT 1")).await;
        set_name(&instance_1, asset_id_1, String::from("Fuel NFT 1")).await;
    }
}

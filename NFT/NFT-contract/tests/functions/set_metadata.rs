use crate::utils::{
    interface::{constructor, metadata, set_metadata},
    setup::{defaults, setup, Metadata},
};
use fuels::types::Bytes;

mod success {

    use super::*;

    #[ignore]
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
        let metadata1 = Metadata::String(String::from("Fuel NFT Metadata"));
        let key = String::from("key1");

        constructor(&instance_1, owner_identity).await;

        assert_eq!(metadata(&instance_1, asset_id_1, key.clone()).await, None);

        set_metadata(&instance_1, asset_id_1, key.clone(), metadata1.clone()).await;
        assert_eq!(
            metadata(&instance_1, asset_id_1, key).await,
            Some(metadata1)
        );
    }

    #[ignore]
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
        let metadata1 = Metadata::String(String::from("Fuel NFT Metadata 1"));
        let metadata2 = Metadata::String(String::from("Fuel NFT Metadata 2"));
        let metadata3 = Metadata::String(String::from("Fuel NFT Metadata 3"));
        let key = String::from("key1");

        constructor(&instance_1, owner_identity).await;

        assert_eq!(metadata(&instance_1, asset_id_1, key.clone()).await, None);
        set_metadata(&instance_1, asset_id_1, key.clone(), metadata1.clone()).await;
        assert_eq!(
            metadata(&instance_1, asset_id_1, key.clone()).await,
            Some(metadata1)
        );

        assert_eq!(metadata(&instance_1, asset_id_2, key.clone()).await, None);
        set_metadata(&instance_1, asset_id_2, key.clone(), metadata2.clone()).await;
        assert_eq!(
            metadata(&instance_1, asset_id_2, key.clone()).await,
            Some(metadata2)
        );

        assert_eq!(metadata(&instance_1, asset_id_3, key.clone()).await, None);
        set_metadata(&instance_1, asset_id_3, key.clone(), metadata3.clone()).await;
        assert_eq!(
            metadata(&instance_1, asset_id_3, key).await,
            Some(metadata3)
        );
    }

    #[ignore]
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
        let metadata1 = Metadata::String(String::from("Fuel NFT Metadata 1"));
        let metadata2 = Metadata::String(String::from("Fuel NFT Metadata 2"));
        let metadata3 = Metadata::String(String::from("Fuel NFT Metadata 3"));
        let key = String::from("key1");

        constructor(&instance_1, owner_identity).await;

        assert_eq!(metadata(&instance_1, asset_id_1, key.clone()).await, None);
        set_metadata(&instance_1, asset_id_1, key.clone(), metadata1.clone()).await;
        assert_eq!(
            metadata(&instance_1, asset_id_1, key.clone()).await,
            Some(metadata1.clone())
        );

        assert_eq!(metadata(&instance_1, asset_id_2, key.clone()).await, None);
        set_metadata(&instance_1, asset_id_2, key.clone(), metadata2.clone()).await;

        assert_eq!(
            metadata(&instance_1, asset_id_1, key.clone()).await,
            Some(metadata1.clone())
        );
        assert_eq!(
            metadata(&instance_1, asset_id_2, key.clone()).await,
            Some(metadata2.clone())
        );

        assert_eq!(metadata(&instance_1, asset_id_3, key.clone()).await, None);
        set_metadata(&instance_1, asset_id_3, key.clone(), metadata3.clone()).await;

        assert_eq!(
            metadata(&instance_1, asset_id_1, key.clone()).await,
            Some(metadata1)
        );
        assert_eq!(
            metadata(&instance_1, asset_id_2, key.clone()).await,
            Some(metadata2)
        );
        assert_eq!(
            metadata(&instance_1, asset_id_3, key).await,
            Some(metadata3)
        );
    }

    #[ignore]
    #[tokio::test]
    async fn sets_multiple_types() {
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
        let metadata1 = Metadata::String(String::from("Fuel NFT Metadata 1"));
        let metadata2 = Metadata::Int(1);
        let metadata3 =
            Metadata::Bytes(Bytes::from_hex_str("bytes").expect("failed to conver to bytes"));
        let key1 = String::from("key1");
        let key2 = String::from("key2");
        let key3 = String::from("key3");

        constructor(&instance_1, owner_identity).await;

        assert_eq!(metadata(&instance_1, asset_id_1, key1.clone()).await, None);
        set_metadata(&instance_1, asset_id_1, key1.clone(), metadata1.clone()).await;
        assert_eq!(
            metadata(&instance_1, asset_id_1, key1.clone()).await,
            Some(metadata1.clone())
        );

        assert_eq!(metadata(&instance_1, asset_id_1, key2.clone()).await, None);
        set_metadata(&instance_1, asset_id_1, key2.clone(), metadata2.clone()).await;
        assert_eq!(
            metadata(&instance_1, asset_id_1, key2.clone()).await,
            Some(metadata2.clone())
        );
        assert_eq!(
            metadata(&instance_1, asset_id_1, key1.clone()).await,
            Some(metadata1.clone())
        );

        assert_eq!(metadata(&instance_1, asset_id_1, key3.clone()).await, None);
        set_metadata(&instance_1, asset_id_1, key3.clone(), metadata3.clone()).await;
        assert_eq!(
            metadata(&instance_1, asset_id_1, key3).await,
            Some(metadata3)
        );
        assert_eq!(
            metadata(&instance_1, asset_id_1, key2.clone()).await,
            Some(metadata2)
        );
        assert_eq!(
            metadata(&instance_1, asset_id_1, key1.clone()).await,
            Some(metadata1)
        );
    }
}

mod revert {

    use super::*;

    #[ignore]
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
        let metadata1 = Metadata::String(String::from("Fuel NFT Metadata 1"));
        let key = String::from("key1");

        constructor(&instance_1, owner_identity).await;

        set_metadata(&instance_2, asset_id_1, key, metadata1).await;
    }

    #[ignore]
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
        let metadata1 = Metadata::String(String::from("Fuel NFT Metadata 1"));
        let key = String::from("key1");

        constructor(&instance_1, owner_identity).await;

        set_metadata(&instance_1, asset_id_1, key.clone(), metadata1.clone()).await;
        set_metadata(&instance_1, asset_id_1, key, metadata1).await;
    }
}

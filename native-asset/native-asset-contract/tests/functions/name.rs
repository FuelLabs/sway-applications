use crate::utils::{
    interface::{constructor, name, set_name},
    setup::{defaults, get_asset_id, setup},
};
use fuels::types::Bytes32;

mod success {

    use super::*;

    #[tokio::test]
    async fn one_asset() {
        let (owner_wallet, other_wallet, id, instance_1, _instance_2) = setup().await;
        let (
            asset_id_1,
            _asset_id_2,
            _sub_id_1,
            _sub_id_2,
            _supply,
            owner_identity,
            _other_identity,
        ) = defaults(id, owner_wallet, other_wallet.clone());

        constructor(&instance_1, owner_identity).await;

        assert_eq!(name(&instance_1, asset_id_1).await, None);

        set_name(&instance_1, asset_id_1, String::from("Fuel Asset 1")).await;
        assert_eq!(
            name(&instance_1, asset_id_1).await,
            Some(String::from("Fuel Asset 1"))
        );
    }

    #[tokio::test]
    async fn multiple_assets() {
        let (owner_wallet, other_wallet, id, instance_1, _instance_2) = setup().await;
        let (
            asset_id_1,
            asset_id_2,
            _sub_id_1,
            _sub_id_2,
            _supply,
            owner_identity,
            _other_identity,
        ) = defaults(id, owner_wallet, other_wallet.clone());

        constructor(&instance_1, owner_identity).await;

        assert_eq!(name(&instance_1, asset_id_1).await, None);
        set_name(&instance_1, asset_id_1, String::from("Fuel Asset 1")).await;
        assert_eq!(
            name(&instance_1, asset_id_1).await,
            Some(String::from("Fuel Asset 1"))
        );

        assert_eq!(name(&instance_1, asset_id_2).await, None);
        set_name(&instance_1, asset_id_2, String::from("Fuel Asset 2")).await;
        assert_eq!(
            name(&instance_1, asset_id_2).await,
            Some(String::from("Fuel Asset 2"))
        );

        let asset_id_3 = get_asset_id(Bytes32::from([3u8; 32]), id);
        assert_eq!(name(&instance_1, asset_id_3).await, None);
        set_name(&instance_1, asset_id_3, String::from("Fuel Asset 3")).await;
        assert_eq!(
            name(&instance_1, asset_id_3).await,
            Some(String::from("Fuel Asset 3"))
        );

        let asset_id_4 = get_asset_id(Bytes32::from([4u8; 32]), id);
        assert_eq!(name(&instance_1, asset_id_4).await, None);
        set_name(&instance_1, asset_id_4, String::from("Fuel Asset 4")).await;
        assert_eq!(
            name(&instance_1, asset_id_4).await,
            Some(String::from("Fuel Asset 4"))
        );

        let asset_id_5 = get_asset_id(Bytes32::from([5u8; 32]), id);
        assert_eq!(name(&instance_1, asset_id_5).await, None);
        set_name(&instance_1, asset_id_5, String::from("Fuel Asset 5")).await;
        assert_eq!(
            name(&instance_1, asset_id_5).await,
            Some(String::from("Fuel Asset 5"))
        );
    }

    #[tokio::test]
    async fn does_not_overwrite_other_names() {
        let (owner_wallet, other_wallet, id, instance_1, _instance_2) = setup().await;
        let (
            asset_id_1,
            asset_id_2,
            _sub_id_1,
            _sub_id_2,
            _supply,
            owner_identity,
            _other_identity,
        ) = defaults(id, owner_wallet, other_wallet.clone());

        constructor(&instance_1, owner_identity).await;

        assert_eq!(name(&instance_1, asset_id_1).await, None);
        set_name(&instance_1, asset_id_1, String::from("Fuel Asset 1")).await;
        assert_eq!(
            name(&instance_1, asset_id_1).await,
            Some(String::from("Fuel Asset 1"))
        );

        assert_eq!(name(&instance_1, asset_id_2).await, None);
        set_name(&instance_1, asset_id_2, String::from("Fuel Asset 2")).await;

        assert_eq!(
            name(&instance_1, asset_id_1).await,
            Some(String::from("Fuel Asset 1"))
        );
        assert_eq!(
            name(&instance_1, asset_id_2).await,
            Some(String::from("Fuel Asset 2"))
        );
    }
}

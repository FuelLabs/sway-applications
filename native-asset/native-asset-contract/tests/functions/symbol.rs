use crate::utils::{
    interface::{constructor, set_symbol, symbol},
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

        assert_eq!(symbol(&instance_1, asset_id_1).await, None);

        set_symbol(&instance_1, asset_id_1, String::from("FA1")).await;
        assert_eq!(
            symbol(&instance_1, asset_id_1).await,
            Some(String::from("FA1"))
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

        assert_eq!(symbol(&instance_1, asset_id_1).await, None);
        set_symbol(&instance_1, asset_id_1, String::from("FA1")).await;
        assert_eq!(
            symbol(&instance_1, asset_id_1).await,
            Some(String::from("FA1"))
        );

        assert_eq!(symbol(&instance_1, asset_id_2).await, None);
        set_symbol(&instance_1, asset_id_2, String::from("FA2")).await;
        assert_eq!(
            symbol(&instance_1, asset_id_2).await,
            Some(String::from("FA2"))
        );

        let asset_id_3 = get_asset_id(Bytes32::from([3u8; 32]), id);
        assert_eq!(symbol(&instance_1, asset_id_3).await, None);
        set_symbol(&instance_1, asset_id_3, String::from("FA3")).await;
        assert_eq!(
            symbol(&instance_1, asset_id_3).await,
            Some(String::from("FA3"))
        );

        let asset_id_4 = get_asset_id(Bytes32::from([4u8; 32]), id);
        assert_eq!(symbol(&instance_1, asset_id_4).await, None);
        set_symbol(&instance_1, asset_id_4, String::from("FA4")).await;
        assert_eq!(
            symbol(&instance_1, asset_id_4).await,
            Some(String::from("FA4"))
        );

        let asset_id_5 = get_asset_id(Bytes32::from([5u8; 32]), id);
        assert_eq!(symbol(&instance_1, asset_id_5).await, None);
        set_symbol(&instance_1, asset_id_5, String::from("FA5")).await;
        assert_eq!(
            symbol(&instance_1, asset_id_5).await,
            Some(String::from("FA5"))
        );
    }

    #[tokio::test]
    async fn does_not_overwrite_other_symbols() {
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

        assert_eq!(symbol(&instance_1, asset_id_1).await, None);
        set_symbol(&instance_1, asset_id_1, String::from("FA1")).await;
        assert_eq!(
            symbol(&instance_1, asset_id_1).await,
            Some(String::from("FA1"))
        );

        assert_eq!(symbol(&instance_1, asset_id_2).await, None);
        set_symbol(&instance_1, asset_id_2, String::from("FA2")).await;

        assert_eq!(
            symbol(&instance_1, asset_id_1).await,
            Some(String::from("FA1"))
        );
        assert_eq!(
            symbol(&instance_1, asset_id_2).await,
            Some(String::from("FA2"))
        );
    }
}

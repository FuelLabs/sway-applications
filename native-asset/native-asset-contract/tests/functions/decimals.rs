use crate::utils::{
    interface::{constructor, decimals, set_decimals},
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

        assert_eq!(decimals(&instance_1, asset_id_1).await, None);

        set_decimals(&instance_1, asset_id_1, 9u8).await;
        assert_eq!(decimals(&instance_1, asset_id_1).await, Some(9u8));
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

        assert_eq!(decimals(&instance_1, asset_id_1).await, None);
        set_decimals(&instance_1, asset_id_1, 9u8).await;
        assert_eq!(decimals(&instance_1, asset_id_1).await, Some(9u8));

        assert_eq!(decimals(&instance_1, asset_id_2).await, None);
        set_decimals(&instance_1, asset_id_2, 8u8).await;
        assert_eq!(decimals(&instance_1, asset_id_2).await, Some(8u8));

        let asset_id_3 = get_asset_id(Bytes32::from([3u8; 32]), id);
        assert_eq!(decimals(&instance_1, asset_id_3).await, None);
        set_decimals(&instance_1, asset_id_3, 7u8).await;
        assert_eq!(decimals(&instance_1, asset_id_3).await, Some(7u8));

        let asset_id_4 = get_asset_id(Bytes32::from([4u8; 32]), id);
        assert_eq!(decimals(&instance_1, asset_id_4).await, None);
        set_decimals(&instance_1, asset_id_4, 6u8).await;
        assert_eq!(decimals(&instance_1, asset_id_4).await, Some(6u8));

        let asset_id_5 = get_asset_id(Bytes32::from([5u8; 32]), id);
        assert_eq!(decimals(&instance_1, asset_id_5).await, None);
        set_decimals(&instance_1, asset_id_5, 5u8).await;
        assert_eq!(decimals(&instance_1, asset_id_5).await, Some(5u8));
    }
}

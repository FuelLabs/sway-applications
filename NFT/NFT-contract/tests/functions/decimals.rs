use crate::utils::{
    interface::{constructor, decimals},
    setup::{defaults, setup},
};

mod success {

    use super::*;

    #[tokio::test]
    async fn one_asset() {
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

        assert_eq!(decimals(&instance_1, asset_id_1).await, Some(0u8));
    }

    #[tokio::test]
    async fn multiple_assets() {
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

        assert_eq!(decimals(&instance_1, asset_id_1).await, Some(0u8));
        assert_eq!(decimals(&instance_1, asset_id_2).await, Some(0u8));
        assert_eq!(decimals(&instance_1, asset_id_3).await, Some(0u8));
    }
}

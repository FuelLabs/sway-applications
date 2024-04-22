use crate::utils::{
    interface::{constructor, mint, total_assets},
    setup::{defaults, setup},
};

mod success {

    use super::*;

    #[tokio::test]
    async fn one_asset() {
        let (owner_wallet, other_wallet, id, instance_1, _instance_2) = setup().await;
        let (
            _asset_id_1,
            _asset_id_2,
            _asset_id_3,
            sub_id_1,
            _sub_id_2,
            _sub_id_3,
            owner_identity,
            other_identity,
        ) = defaults(id, owner_wallet, other_wallet.clone());

        constructor(&instance_1, owner_identity.clone()).await;

        assert_eq!(total_assets(&instance_1).await, 0);

        mint(&instance_1, other_identity, sub_id_1, 1).await;
        assert_eq!(total_assets(&instance_1).await, 1);
    }

    #[tokio::test]
    async fn multiple_assets() {
        let (owner_wallet, other_wallet, id, instance_1, _instance_2) = setup().await;
        let (
            _asset_id_1,
            _asset_id_2,
            _asset_id_3,
            sub_id_1,
            sub_id_2,
            sub_id_3,
            owner_identity,
            other_identity,
        ) = defaults(id, owner_wallet, other_wallet.clone());

        constructor(&instance_1, owner_identity.clone()).await;

        assert_eq!(total_assets(&instance_1).await, 0);

        mint(&instance_1, other_identity.clone(), sub_id_1, 1).await;
        assert_eq!(total_assets(&instance_1).await, 1);

        mint(&instance_1, other_identity.clone(), sub_id_2, 1).await;
        assert_eq!(total_assets(&instance_1).await, 2);

        mint(&instance_1, other_identity.clone(), sub_id_3, 1).await;
        assert_eq!(total_assets(&instance_1).await, 3);
    }
}

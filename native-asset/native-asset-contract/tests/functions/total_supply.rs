use crate::utils::{
    interface::{burn, constructor, mint, total_supply},
    setup::{defaults, get_asset_id, setup},
};
use fuels::types::{Bits256, Bytes32};

mod success {

    use super::*;

    #[tokio::test]
    async fn one_asset() {
        let (owner_wallet, other_wallet, id, instance_1, _instance_2) = setup().await;
        let (asset_id_1, _asset_id_2, sub_id_1, _sub_id_2, _supply, owner_identity, other_identity) =
            defaults(id, owner_wallet, other_wallet.clone());

        constructor(&instance_1, owner_identity).await;

        assert_eq!(total_supply(&instance_1, asset_id_1).await, None);
        mint(&instance_1, other_identity, sub_id_1, 100).await;
        assert_eq!(total_supply(&instance_1, asset_id_1).await, Some(100));
    }

    #[tokio::test]
    async fn multiple_assets() {
        let (owner_wallet, other_wallet, id, instance_1, _instance_2) = setup().await;
        let (asset_id_1, asset_id_2, sub_id_1, sub_id_2, _supply, owner_identity, other_identity) =
            defaults(id, owner_wallet, other_wallet.clone());

        constructor(&instance_1, owner_identity).await;

        assert_eq!(total_supply(&instance_1, asset_id_1).await, None);
        mint(&instance_1, other_identity, sub_id_1, 100).await;
        assert_eq!(total_supply(&instance_1, asset_id_1).await, Some(100));

        assert_eq!(total_supply(&instance_1, asset_id_2).await, None);
        mint(&instance_1, other_identity, sub_id_2, 200).await;
        assert_eq!(total_supply(&instance_1, asset_id_2).await, Some(200));

        let asset_id_3 = get_asset_id(Bytes32::from([3u8; 32]), id);
        assert_eq!(total_supply(&instance_1, asset_id_3).await, None);
        mint(&instance_1, other_identity, Bits256([3u8; 32]), 300).await;
        assert_eq!(total_supply(&instance_1, asset_id_3).await, Some(300));

        let asset_id_4 = get_asset_id(Bytes32::from([4u8; 32]), id);
        assert_eq!(total_supply(&instance_1, asset_id_4).await, None);
        mint(&instance_1, other_identity, Bits256([4u8; 32]), 400).await;
        assert_eq!(total_supply(&instance_1, asset_id_4).await, Some(400));

        let asset_id_5 = get_asset_id(Bytes32::from([5u8; 32]), id);
        assert_eq!(total_supply(&instance_1, asset_id_5).await, None);
        mint(&instance_1, other_identity, Bits256([5u8; 32]), 500).await;
        assert_eq!(total_supply(&instance_1, asset_id_5).await, Some(500));
    }

    #[tokio::test]
    async fn only_increments_on_one_asset() {
        let (owner_wallet, other_wallet, id, instance_1, _instance_2) = setup().await;
        let (asset_id_1, _asset_id_2, sub_id_1, sub_id_2, _supply, owner_identity, other_identity) =
            defaults(id, owner_wallet, other_wallet.clone());

        constructor(&instance_1, owner_identity).await;

        assert_eq!(total_supply(&instance_1, asset_id_1).await, None);
        mint(&instance_1, other_identity, sub_id_1, 100).await;
        assert_eq!(total_supply(&instance_1, asset_id_1).await, Some(100));

        mint(&instance_1, other_identity, sub_id_2, 200).await;
        assert_eq!(total_supply(&instance_1, asset_id_1).await, Some(100));

        mint(&instance_1, other_identity, sub_id_2, 300).await;
        assert_eq!(total_supply(&instance_1, asset_id_1).await, Some(100));
    }

    #[tokio::test]
    async fn decrements_on_burn() {
        let (owner_wallet, other_wallet, id, instance_1, instance_2) = setup().await;
        let (asset_id_1, _asset_id_2, sub_id_1, _sub_id_2, _supply, owner_identity, other_identity) =
            defaults(id, owner_wallet, other_wallet.clone());

        constructor(&instance_1, owner_identity).await;

        assert_eq!(total_supply(&instance_1, asset_id_1).await, None);
        mint(&instance_1, other_identity, sub_id_1, 100).await;
        assert_eq!(total_supply(&instance_1, asset_id_1).await, Some(100));

        burn(&instance_2, asset_id_1, sub_id_1, 50).await;
        assert_eq!(total_supply(&instance_1, asset_id_1).await, Some(50));
    }
}

use crate::utils::{
    interface::{burn, constructor, mint, total_assets, total_supply},
    setup::{defaults, get_wallet_balance, setup},
};

mod success {

    use super::*;

    #[tokio::test]
    async fn mints_assets() {
        let (owner_wallet, other_wallet, id, instance_1, _instance_2) = setup().await;
        let (asset_id_1, _asset_id_2, sub_id_1, _sub_id_2, _supply, owner_identity, other_identity) =
            defaults(id, owner_wallet, other_wallet.clone());

        constructor(&instance_1, owner_identity).await;

        assert_eq!(get_wallet_balance(&other_wallet, &asset_id_1).await, 0);
        assert_eq!(total_supply(&instance_1, asset_id_1).await, None);
        assert_eq!(total_assets(&instance_1).await, 0);

        mint(&instance_1, other_identity, sub_id_1, 100).await;

        assert_eq!(get_wallet_balance(&other_wallet, &asset_id_1).await, 100);
        assert_eq!(total_supply(&instance_1, asset_id_1).await, Some(100));
        assert_eq!(total_assets(&instance_1).await, 1);
    }

    #[tokio::test]
    async fn mints_multiple_assets() {
        let (owner_wallet, other_wallet, id, instance_1, _instance_2) = setup().await;
        let (asset_id_1, asset_id_2, sub_id_1, sub_id_2, _supply, owner_identity, other_identity) =
            defaults(id, owner_wallet, other_wallet.clone());

        constructor(&instance_1, owner_identity).await;
        mint(&instance_1, other_identity, sub_id_1, 100).await;

        assert_eq!(get_wallet_balance(&other_wallet, &asset_id_1).await, 100);
        assert_eq!(get_wallet_balance(&other_wallet, &asset_id_2).await, 0);
        assert_eq!(total_supply(&instance_1, asset_id_1).await, Some(100));
        assert_eq!(total_supply(&instance_1, asset_id_2).await, None);
        assert_eq!(total_assets(&instance_1).await, 1);

        mint(&instance_1, other_identity, sub_id_2, 200).await;

        assert_eq!(get_wallet_balance(&other_wallet, &asset_id_1).await, 100);
        assert_eq!(get_wallet_balance(&other_wallet, &asset_id_2).await, 200);
        assert_eq!(total_supply(&instance_1, asset_id_1).await, Some(100));
        assert_eq!(total_supply(&instance_1, asset_id_2).await, Some(200));
        assert_eq!(total_assets(&instance_1).await, 2);
    }

    #[tokio::test]
    async fn can_mint_max_supply() {
        let (owner_wallet, other_wallet, id, instance_1, _instance_2) = setup().await;
        let (asset_id_1, _asset_id_2, sub_id_1, _sub_id_2, supply, owner_identity, other_identity) =
            defaults(id, owner_wallet, other_wallet.clone());

        constructor(&instance_1, owner_identity).await;

        assert_eq!(get_wallet_balance(&other_wallet, &asset_id_1).await, 0);
        assert_eq!(total_supply(&instance_1, asset_id_1).await, None);
        assert_eq!(total_assets(&instance_1).await, 0);

        mint(&instance_1, other_identity, sub_id_1, supply).await;

        assert_eq!(get_wallet_balance(&other_wallet, &asset_id_1).await, supply);
        assert_eq!(total_supply(&instance_1, asset_id_1).await, Some(supply));
        assert_eq!(total_assets(&instance_1).await, 1);
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "NotOwner")]
    async fn when_not_owner() {
        let (owner_wallet, other_wallet, id, instance_1, instance_2) = setup().await;
        let (
            _asset_id_1,
            _asset_id_2,
            sub_id_1,
            _sub_id_2,
            _supply,
            owner_identity,
            other_identity,
        ) = defaults(id, owner_wallet, other_wallet);

        constructor(&instance_1, owner_identity).await;

        mint(&instance_2, other_identity, sub_id_1, 100).await;
    }

    #[tokio::test]
    #[should_panic(expected = "NotOwner")]
    async fn when_no_owner() {
        let (owner_wallet, other_wallet, id, instance_1, _instance_2) = setup().await;
        let (
            _asset_id_1,
            _asset_id_2,
            sub_id_1,
            _sub_id_2,
            _supply,
            _owner_identity,
            other_identity,
        ) = defaults(id, owner_wallet, other_wallet);

        mint(&instance_1, other_identity, sub_id_1, 100).await;
    }

    #[tokio::test]
    #[should_panic(expected = "MaxMinted")]
    async fn when_max_supply_minted() {
        let (owner_wallet, other_wallet, id, instance_1, _instance_2) = setup().await;
        let (_asset_id_1, _asset_id_2, sub_id_1, _sub_id_2, supply, owner_identity, other_identity) =
            defaults(id, owner_wallet, other_wallet);

        constructor(&instance_1, owner_identity).await;

        mint(&instance_1, other_identity, sub_id_1, supply + 1).await;
    }

    #[tokio::test]
    #[should_panic(expected = "MaxMinted")]
    async fn when_minting_max_supply_after_burn() {
        let (owner_wallet, other_wallet, id, instance_1, instance_2) = setup().await;
        let (asset_id_1, _asset_id_2, sub_id_1, _sub_id_2, supply, owner_identity, other_identity) =
            defaults(id, owner_wallet, other_wallet.clone());

        constructor(&instance_1, owner_identity).await;
        mint(&instance_1, other_identity, sub_id_1, supply).await;

        burn(&instance_2, asset_id_1, sub_id_1, 1).await;

        mint(&instance_1, other_identity, sub_id_1, 1).await;
    }
}

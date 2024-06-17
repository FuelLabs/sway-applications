use crate::utils::{
    interface::{constructor, mint, pause, total_assets, total_supply},
    setup::{defaults, get_wallet_balance, setup},
};

mod success {

    use super::*;

    #[tokio::test]
    async fn mints_assets() {
        let (owner_wallet, other_wallet, id, instance_1, _instance_2) = setup().await;
        let (
            asset_id_1,
            _asset_id_2,
            _asset_id_3,
            sub_id_1,
            _sub_id_2,
            _sub_id_3,
            owner_identity,
            other_identity,
        ) = defaults(id, owner_wallet, other_wallet.clone());

        constructor(&instance_1, owner_identity).await;

        assert_eq!(get_wallet_balance(&other_wallet, &asset_id_1).await, 0);
        assert_eq!(total_supply(&instance_1, asset_id_1).await, None);
        assert_eq!(total_assets(&instance_1).await, 0);

        mint(&instance_1, other_identity, sub_id_1, 1).await;

        assert_eq!(get_wallet_balance(&other_wallet, &asset_id_1).await, 1);
        assert_eq!(total_supply(&instance_1, asset_id_1).await, Some(1));
        assert_eq!(total_assets(&instance_1).await, 1);
    }

    #[tokio::test]
    async fn mints_multiple_assets() {
        let (owner_wallet, other_wallet, id, instance_1, _instance_2) = setup().await;
        let (
            asset_id_1,
            asset_id_2,
            asset_id_3,
            sub_id_1,
            sub_id_2,
            sub_id_3,
            owner_identity,
            other_identity,
        ) = defaults(id, owner_wallet, other_wallet.clone());

        constructor(&instance_1, owner_identity).await;
        mint(&instance_1, other_identity, sub_id_1, 1).await;

        assert_eq!(get_wallet_balance(&other_wallet, &asset_id_1).await, 1);
        assert_eq!(get_wallet_balance(&other_wallet, &asset_id_2).await, 0);
        assert_eq!(total_supply(&instance_1, asset_id_1).await, Some(1));
        assert_eq!(total_supply(&instance_1, asset_id_2).await, None);
        assert_eq!(total_assets(&instance_1).await, 1);

        mint(&instance_1, other_identity, sub_id_2, 1).await;

        assert_eq!(get_wallet_balance(&other_wallet, &asset_id_1).await, 1);
        assert_eq!(get_wallet_balance(&other_wallet, &asset_id_2).await, 1);
        assert_eq!(total_supply(&instance_1, asset_id_1).await, Some(1));
        assert_eq!(total_supply(&instance_1, asset_id_2).await, Some(1));
        assert_eq!(total_assets(&instance_1).await, 2);

        mint(&instance_1, other_identity, sub_id_3, 1).await;

        assert_eq!(get_wallet_balance(&other_wallet, &asset_id_1).await, 1);
        assert_eq!(get_wallet_balance(&other_wallet, &asset_id_2).await, 1);
        assert_eq!(get_wallet_balance(&other_wallet, &asset_id_3).await, 1);
        assert_eq!(total_supply(&instance_1, asset_id_1).await, Some(1));
        assert_eq!(total_supply(&instance_1, asset_id_2).await, Some(1));
        assert_eq!(total_supply(&instance_1, asset_id_3).await, Some(1));
        assert_eq!(total_assets(&instance_1).await, 3);
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Paused")]
    async fn when_paused() {
        let (owner_wallet, other_wallet, id, instance_1, instance_2) = setup().await;
        let (
            _asset_id_1,
            _asset_id_2,
            _asset_id_3,
            sub_id_1,
            _sub_id_2,
            _sub_id_3,
            owner_identity,
            other_identity,
        ) = defaults(id, owner_wallet, other_wallet);

        constructor(&instance_1, owner_identity).await;

        pause(&instance_1).await;

        mint(&instance_2, other_identity, sub_id_1, 1).await;
    }

    #[tokio::test]
    #[should_panic(expected = "CannotMintMoreThanOneNFTWithSubId")]
    async fn when_minting_more_than_one() {
        let (owner_wallet, other_wallet, id, instance_1, _instance_2) = setup().await;
        let (
            _asset_id_1,
            _asset_id_2,
            _asset_id_3,
            sub_id_1,
            _sub_id_2,
            _sub_id_3,
            _owner_identity,
            other_identity,
        ) = defaults(id, owner_wallet, other_wallet);

        mint(&instance_1, other_identity, sub_id_1, 2).await;
    }

    #[tokio::test]
    #[should_panic(expected = "NFTAlreadyMinted")]
    async fn when_nft_already_minted() {
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
        ) = defaults(id, owner_wallet, other_wallet);

        constructor(&instance_1, owner_identity).await;

        mint(&instance_1, other_identity, sub_id_1, 1).await;
        mint(&instance_1, other_identity, sub_id_1, 1).await;
    }
}

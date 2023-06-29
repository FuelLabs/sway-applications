mod success {

    use crate::utils::{
        interface::{core::create_escrow, info::assets},
        setup::{create_arbiter, create_asset, mint, setup},
    };

    #[tokio::test]
    async fn returns_none() {
        let (_arbiter, _buyer, seller, _defaults) = setup().await;
        assert!(matches!(assets(&seller, 0).await, None));
    }

    #[tokio::test]
    async fn returns_asset() {
        let (arbiter, buyer, seller, defaults) = setup().await;
        let arbiter_obj = create_arbiter(&arbiter, defaults.asset_id, defaults.asset_amount).await;
        let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

        mint(&seller, defaults.asset_amount, &defaults.asset).await;

        assert!(matches!(assets(&seller, 0).await, None));

        create_escrow(
            defaults.asset_amount,
            &arbiter_obj,
            &defaults.asset_id,
            vec![asset.clone()],
            &buyer,
            &seller,
            defaults.deadline,
        )
        .await;

        assert_eq!(assets(&seller, 0).await.unwrap(), asset);
    }
}

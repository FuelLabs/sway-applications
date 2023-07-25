mod success {

    use crate::utils::{
        interface::{core::create_escrow, info::escrow_count},
        setup::{create_arbiter, create_asset, setup},
    };

    #[tokio::test]
    async fn returns_zero() {
        let (_arbiter, _buyer, seller, _defaults) = setup().await;
        assert_eq!(0, escrow_count(&seller).await);
    }

    #[tokio::test]
    async fn returns_one() {
        let (arbiter, buyer, seller, defaults) = setup().await;
        let arbiter_obj = create_arbiter(&arbiter, defaults.asset_id, defaults.asset_amount).await;
        let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

        assert_eq!(0, escrow_count(&seller).await);

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

        assert_eq!(1, escrow_count(&seller).await);
    }
}

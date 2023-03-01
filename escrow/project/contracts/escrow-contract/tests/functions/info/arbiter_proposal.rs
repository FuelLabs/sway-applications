mod success {

    use crate::utils::{
        interface::{
            core::{create_escrow, propose_arbiter},
            info::arbiter_proposal,
        },
        setup::{create_arbiter, create_asset, mint, setup},
    };

    #[tokio::test]
    async fn returns_none() {
        let (_arbiter, _buyer, seller, _defaults) = setup().await;
        assert!(matches!(arbiter_proposal(&seller, 0).await, None));
    }

    #[tokio::test]
    async fn return_arbiter() {
        let (arbiter, buyer, seller, defaults) = setup().await;
        let arbiter_obj = create_arbiter(&arbiter, defaults.asset_id, defaults.asset_amount).await;
        let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

        mint(&seller, defaults.asset_amount * 2, &defaults.asset).await;
        create_escrow(
            defaults.asset_amount,
            &arbiter_obj,
            &defaults.asset_id,
            vec![asset.clone(), asset.clone()],
            &buyer,
            &seller,
            defaults.deadline,
        )
        .await;

        assert!(matches!(arbiter_proposal(&seller, 0).await, None));

        propose_arbiter(arbiter_obj.clone(), &seller, 0).await;

        assert_eq!(arbiter_proposal(&seller, 0).await.unwrap(), arbiter_obj);
    }
}

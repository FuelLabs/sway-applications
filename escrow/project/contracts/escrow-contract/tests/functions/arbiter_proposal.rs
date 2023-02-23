mod success {

    use crate::utils::{
        abi_calls::{arbiter_proposal, create_escrow, propose_arbiter},
        test_helpers::{create_arbiter, create_asset, mint, setup},
    };

    #[tokio::test]
    async fn returns_none() {
        let (_arbiter, _buyer, seller, _defaults) = setup().await;
        assert!(matches!(arbiter_proposal(&seller.contract, 0).await, None));
    }

    #[tokio::test]
    async fn return_arbiter() {
        let (arbiter, buyer, seller, defaults) = setup().await;
        let arbiter_obj = create_arbiter(
            arbiter.wallet.address(),
            defaults.asset_id,
            defaults.asset_amount,
        )
        .await;
        let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

        mint(
            seller.wallet.address(),
            defaults.asset_amount * 2,
            &defaults.asset,
        )
        .await;
        create_escrow(
            defaults.asset_amount,
            &arbiter_obj,
            &defaults.asset_id,
            vec![asset.clone(), asset.clone()],
            buyer.wallet.address(),
            &seller.contract,
            defaults.deadline,
        )
        .await;

        assert!(matches!(arbiter_proposal(&seller.contract, 0).await, None));

        propose_arbiter(arbiter_obj.clone(), &seller.contract, 0).await;

        assert_eq!(
            arbiter_proposal(&seller.contract, 0).await.unwrap(),
            arbiter_obj
        );
    }
}

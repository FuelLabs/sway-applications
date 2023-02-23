use crate::utils::{
    interface::core::{create_escrow, deposit, withdraw_collateral},
    setup::{create_arbiter, create_asset, mint, setup},
};

mod success {

    use super::*;
    use crate::utils::{
        interface::core::propose_arbiter,
        setup::{asset_amount, WithdrawnCollateralEvent},
    };

    #[tokio::test]
    #[ignore]
    async fn withdraws_collateral() {
        let (arbiter, buyer, seller, defaults) = setup().await;

        mint(
            seller.wallet.address(),
            defaults.asset_amount,
            &defaults.asset,
        )
        .await;

        let arbiter_obj = create_arbiter(
            arbiter.wallet.address(),
            defaults.asset_id,
            defaults.asset_amount,
        )
        .await;
        let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

        create_escrow(
            defaults.asset_amount,
            &arbiter_obj,
            &defaults.asset_id,
            vec![asset.clone(), asset.clone()],
            buyer.wallet.address(),
            &seller.contract,
            6,
        )
        .await;

        assert_eq!(0, asset_amount(&defaults.asset_id, &seller.wallet).await);

        // TODO: need to shift block by one, waiting on SDK
        let response = withdraw_collateral(&seller.contract, 0).await;
        let log = response
            .get_logs_with_type::<WithdrawnCollateralEvent>()
            .unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(*event, WithdrawnCollateralEvent { identifier: 0 });
        assert_eq!(
            defaults.asset_amount,
            asset_amount(&defaults.asset_id, &seller.wallet).await
        );
    }

    #[tokio::test]
    async fn withdraws_collateral_after_proposing_arbiter() {
        let (arbiter, buyer, seller, defaults) = setup().await;

        mint(
            seller.wallet.address(),
            defaults.asset_amount * 2,
            &defaults.asset,
        )
        .await;

        let arbiter_obj = create_arbiter(
            arbiter.wallet.address(),
            defaults.asset_id,
            defaults.asset_amount,
        )
        .await;
        let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

        create_escrow(
            defaults.asset_amount,
            &arbiter_obj,
            &defaults.asset_id,
            vec![asset.clone(), asset.clone()],
            buyer.wallet.address(),
            &seller.contract,
            5,
        )
        .await;

        propose_arbiter(arbiter_obj, &seller.contract, 0).await;

        assert_eq!(0, asset_amount(&defaults.asset_id, &seller.wallet).await);

        let response = withdraw_collateral(&seller.contract, 0).await;
        let log = response
            .get_logs_with_type::<WithdrawnCollateralEvent>()
            .unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(*event, WithdrawnCollateralEvent { identifier: 0 });
        assert_eq!(
            defaults.asset_amount * 2,
            asset_amount(&defaults.asset_id, &seller.wallet).await
        );
    }

    #[tokio::test]
    #[ignore]
    async fn withdraws_collateral_in_two_escrows() {
        // TODO: skipping similar to withdraws_collateral
    }
}

mod revert {

    use super::*;
    use crate::utils::interface::core::return_deposit;

    #[tokio::test]
    #[should_panic(expected = "StateNotPending")]
    async fn when_escrow_is_not_pending() {
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
            defaults.asset_amount,
            &defaults.asset,
        )
        .await;
        mint(
            buyer.wallet.address(),
            defaults.asset_amount,
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
        deposit(
            defaults.asset_amount,
            &defaults.asset_id,
            &buyer.contract,
            0,
        )
        .await;
        return_deposit(&seller.contract, 0).await;
        withdraw_collateral(&seller.contract, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "CannotWithdrawBeforeDeadline")]
    async fn when_deadline_is_not_in_the_past() {
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
            defaults.asset_amount,
            &defaults.asset,
        )
        .await;
        mint(
            buyer.wallet.address(),
            defaults.asset_amount,
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
        deposit(
            defaults.asset_amount,
            &defaults.asset_id,
            &buyer.contract,
            0,
        )
        .await;
        withdraw_collateral(&seller.contract, 0).await;
    }

    #[tokio::test]
    #[ignore]
    #[should_panic(expected = "Unauthorized")]
    async fn when_caller_is_not_seller() {
        // Test passes when deadline requirement is met. Ignored till SDK manipulation to prevent failure
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
            defaults.asset_amount,
            &defaults.asset,
        )
        .await;
        mint(
            buyer.wallet.address(),
            defaults.asset_amount,
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
        deposit(
            defaults.asset_amount,
            &defaults.asset_id,
            &buyer.contract,
            0,
        )
        .await;
        withdraw_collateral(&seller.contract, 0).await;
    }

    #[tokio::test]
    #[ignore]
    #[should_panic(expected = "CannotWithdrawAfterDesposit")]
    async fn when_buyer_has_deposited() {
        // Test passes when deadline requirement is met. Ignored till SDK manipulation to prevent failure
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
            defaults.asset_amount,
            &defaults.asset,
        )
        .await;
        mint(
            buyer.wallet.address(),
            defaults.asset_amount,
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
        deposit(
            defaults.asset_amount,
            &defaults.asset_id,
            &buyer.contract,
            0,
        )
        .await;
        withdraw_collateral(&seller.contract, 0).await;
    }
}

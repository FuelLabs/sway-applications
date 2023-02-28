use crate::utils::{
    interface::core::{create_escrow, deposit, dispute, take_payment},
    setup::{create_arbiter, create_asset, mint, setup},
};

mod success {

    use super::*;
    use crate::utils::{
        interface::{
            core::propose_arbiter,
            info::{arbiter_proposal, escrows},
        },
        setup::{asset_amount, escrow_info, PaymentTakenEvent},
    };

    #[tokio::test]
    #[ignore]
    async fn takes_payment() {
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
            6,
        )
        .await;
        deposit(
            defaults.asset_amount,
            &defaults.asset_id,
            &buyer.contract,
            0,
        )
        .await;

        // TODO: need to shift block by one, waiting on SDK then uncomment below

        assert_eq!(0, asset_amount(&defaults.asset_id, &buyer.wallet).await);
        assert_eq!(0, asset_amount(&defaults.asset_id, &seller.wallet).await);

        assert_eq!(
            escrows(&seller.contract, 0).await.unwrap(),
            escrow_info(
                arbiter_obj.clone(),
                2,
                buyer.wallet.address(),
                Some(defaults.asset_id),
                defaults.asset_amount,
                6,
                false,
                0,
                seller.wallet.address(),
                false
            )
            .await
        );

        // let response = take_payment(&seller.contract, 0).await;

        // assert_eq!(defaults.asset_amount, asset_amount(&defaults.asset_id, &seller.wallet).await);
        assert_eq!(0, asset_amount(&defaults.asset_id, &buyer.wallet).await);

        assert_eq!(
            escrows(&seller.contract, 0).await.unwrap(),
            escrow_info(
                arbiter_obj.clone(),
                2,
                buyer.wallet.address(),
                Some(defaults.asset_id),
                defaults.asset_amount,
                6,
                false,
                0,
                seller.wallet.address(),
                true
            )
            .await
        );

        // let log = response
        //     .get_logs_with_type::<PaymentTakenEvent>()
        //     .unwrap();
        // let event = log.get(0).unwrap();

        // assert_eq!(*event, PaymentTakenEvent { identifier: 0 });
    }

    #[tokio::test]
    async fn takes_payment_after_proposing_arbiter() {
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
            7,
        )
        .await;
        deposit(
            defaults.asset_amount,
            &defaults.asset_id,
            &buyer.contract,
            0,
        )
        .await;

        // This should really be above `deposit` but given SDK limitations for block manipulation
        // we put this here
        propose_arbiter(arbiter_obj.clone(), &seller.contract, 0).await;

        assert_eq!(0, asset_amount(&defaults.asset_id, &buyer.wallet).await);
        assert_eq!(0, asset_amount(&defaults.asset_id, &seller.wallet).await);
        assert_eq!(
            escrows(&seller.contract, 0).await.unwrap(),
            escrow_info(
                arbiter_obj.clone(),
                2,
                buyer.wallet.address(),
                Some(defaults.asset_id),
                defaults.asset_amount,
                7,
                false,
                0,
                seller.wallet.address(),
                false
            )
            .await
        );
        assert_eq!(
            arbiter_proposal(&seller.contract, 0).await.unwrap(),
            arbiter_obj.clone()
        );

        let response = take_payment(&seller.contract, 0).await;

        assert_eq!(
            defaults.asset_amount * 3,
            asset_amount(&defaults.asset_id, &seller.wallet).await
        );
        assert_eq!(0, asset_amount(&defaults.asset_id, &buyer.wallet).await);
        assert_eq!(
            escrows(&seller.contract, 0).await.unwrap(),
            escrow_info(
                arbiter_obj.clone(),
                2,
                buyer.wallet.address(),
                Some(defaults.asset_id),
                defaults.asset_amount,
                7,
                false,
                0,
                seller.wallet.address(),
                true
            )
            .await
        );
        assert!(matches!(arbiter_proposal(&seller.contract, 0).await, None));

        let log = response.get_logs_with_type::<PaymentTakenEvent>().unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(*event, PaymentTakenEvent { identifier: 0 });
    }

    #[tokio::test]
    #[ignore]
    async fn takes_payment_in_two_escrows() {
        // TODO: skipping similar to takes_payment
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
        take_payment(&seller.contract, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "CannotTakePaymentBeforeDeadline")]
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
        take_payment(&seller.contract, 0).await;
    }

    #[tokio::test]
    #[ignore]
    #[should_panic(expected = "CannotTakePaymentDuringDispute")]
    async fn when_disputed() {
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
        dispute(&buyer.contract, 0).await;
        take_payment(&seller.contract, 0).await;
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
        take_payment(&buyer.contract, 0).await;
    }

    #[tokio::test]
    #[ignore]
    #[should_panic(expected = "CannotTransferBeforeDesposit")]
    async fn when_buyer_has_not_deposited() {
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
        take_payment(&buyer.contract, 0).await;
    }
}

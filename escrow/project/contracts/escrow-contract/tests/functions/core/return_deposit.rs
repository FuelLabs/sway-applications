use crate::utils::{
    interface::core::{create_escrow, deposit, return_deposit},
    setup::{create_arbiter, create_asset, mint, setup},
};

mod success {

    use super::*;
    use crate::utils::{
        interface::{core::propose_arbiter, info::escrows},
        setup::{asset_amount, escrow_info, ReturnedDepositEvent},
    };

    #[tokio::test]
    async fn returns_deposit() {
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

        assert_eq!(0, asset_amount(&defaults.asset_id, &buyer.wallet).await);

        assert_eq!(
            escrows(&seller.contract, 0).await.unwrap(),
            escrow_info(
                arbiter_obj.clone(),
                2,
                buyer.wallet.address(),
                Some(defaults.asset_id),
                defaults.asset_amount,
                defaults.deadline,
                false,
                0,
                seller.wallet.address(),
                false
            )
            .await
        );

        let response = return_deposit(&seller.contract, 0).await;

        assert_eq!(
            defaults.asset_amount,
            asset_amount(&defaults.asset_id, &buyer.wallet).await
        );
        assert_eq!(
            escrows(&seller.contract, 0).await.unwrap(),
            escrow_info(
                arbiter_obj.clone(),
                2,
                buyer.wallet.address(),
                Some(defaults.asset_id),
                defaults.asset_amount,
                defaults.deadline,
                false,
                0,
                seller.wallet.address(),
                true
            )
            .await
        );

        let log = response
            .get_logs_with_type::<ReturnedDepositEvent>()
            .unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(*event, ReturnedDepositEvent { identifier: 0 });
    }

    #[tokio::test]
    async fn returns_deposit_after_proposing_arbiter() {
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
            defaults.deadline,
        )
        .await;
        propose_arbiter(arbiter_obj.clone(), &seller.contract, 0).await;
        deposit(
            defaults.asset_amount,
            &defaults.asset_id,
            &buyer.contract,
            0,
        )
        .await;

        assert_eq!(0, asset_amount(&defaults.asset_id, &seller.wallet).await);
        assert_eq!(0, asset_amount(&defaults.asset_id, &buyer.wallet).await);

        assert_eq!(
            escrows(&seller.contract, 0).await.unwrap(),
            escrow_info(
                arbiter_obj.clone(),
                2,
                buyer.wallet.address(),
                Some(defaults.asset_id),
                defaults.asset_amount,
                defaults.deadline,
                false,
                0,
                seller.wallet.address(),
                false
            )
            .await
        );

        let response = return_deposit(&seller.contract, 0).await;

        assert_eq!(
            defaults.asset_amount,
            asset_amount(&defaults.asset_id, &buyer.wallet).await
        );
        assert_eq!(
            defaults.asset_amount * 2,
            asset_amount(&defaults.asset_id, &seller.wallet).await
        );

        assert_eq!(
            escrows(&seller.contract, 0).await.unwrap(),
            escrow_info(
                arbiter_obj.clone(),
                2,
                buyer.wallet.address(),
                Some(defaults.asset_id),
                defaults.asset_amount,
                defaults.deadline,
                false,
                0,
                seller.wallet.address(),
                true
            )
            .await
        );

        let log = response
            .get_logs_with_type::<ReturnedDepositEvent>()
            .unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(*event, ReturnedDepositEvent { identifier: 0 });
    }

    #[tokio::test]
    async fn returns_deposit_in_two_escrows() {
        let (arbiter, buyer, seller, defaults) = setup().await;

        mint(
            seller.wallet.address(),
            defaults.asset_amount * 2,
            &defaults.asset,
        )
        .await;
        mint(
            buyer.wallet.address(),
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
            defaults.deadline,
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
        deposit(
            defaults.asset_amount,
            &defaults.asset_id,
            &buyer.contract,
            1,
        )
        .await;

        assert_eq!(0, asset_amount(&defaults.asset_id, &buyer.wallet).await);

        assert_eq!(
            escrows(&seller.contract, 0).await.unwrap(),
            escrow_info(
                arbiter_obj.clone(),
                2,
                buyer.wallet.address(),
                Some(defaults.asset_id),
                defaults.asset_amount,
                defaults.deadline,
                false,
                0,
                seller.wallet.address(),
                false
            )
            .await
        );

        let response1 = return_deposit(&seller.contract, 0).await;

        assert_eq!(
            defaults.asset_amount,
            asset_amount(&defaults.asset_id, &buyer.wallet).await
        );

        assert_eq!(
            escrows(&seller.contract, 0).await.unwrap(),
            escrow_info(
                arbiter_obj.clone(),
                2,
                buyer.wallet.address(),
                Some(defaults.asset_id),
                defaults.asset_amount,
                defaults.deadline,
                false,
                0,
                seller.wallet.address(),
                true
            )
            .await
        );

        assert_eq!(
            escrows(&seller.contract, 1).await.unwrap(),
            escrow_info(
                arbiter_obj.clone(),
                2,
                buyer.wallet.address(),
                Some(defaults.asset_id),
                defaults.asset_amount,
                defaults.deadline,
                false,
                2,
                seller.wallet.address(),
                false
            )
            .await
        );

        let response2 = return_deposit(&seller.contract, 1).await;

        assert_eq!(
            defaults.asset_amount * 2,
            asset_amount(&defaults.asset_id, &buyer.wallet).await
        );

        assert_eq!(
            escrows(&seller.contract, 1).await.unwrap(),
            escrow_info(
                arbiter_obj.clone(),
                2,
                buyer.wallet.address(),
                Some(defaults.asset_id),
                defaults.asset_amount,
                defaults.deadline,
                false,
                2,
                seller.wallet.address(),
                true
            )
            .await
        );

        let log1 = response1
            .get_logs_with_type::<ReturnedDepositEvent>()
            .unwrap();
        let log2 = response2
            .get_logs_with_type::<ReturnedDepositEvent>()
            .unwrap();
        let event1 = log1.get(0).unwrap();
        let event2 = log2.get(0).unwrap();

        assert_eq!(*event1, ReturnedDepositEvent { identifier: 0 });
        assert_eq!(*event2, ReturnedDepositEvent { identifier: 1 });
    }
}

mod revert {

    use super::*;

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
        return_deposit(&seller.contract, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Unauthorized")]
    async fn when_caller_is_not_seller() {
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
        return_deposit(&buyer.contract, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "CannotTransferBeforeDesposit")]
    async fn when_buyer_has_not_deposited() {
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
        return_deposit(&seller.contract, 0).await;
    }
}

use crate::utils::{
    interface::core::{create_escrow, deposit, transfer_to_seller},
    setup::{create_arbiter, create_asset, mint, setup},
};

mod success {

    use super::*;
    use crate::utils::{
        interface::{
            core::propose_arbiter,
            info::{arbiter_proposal, escrows},
        },
        setup::{asset_amount, escrow_info, TransferredToSellerEvent},
    };

    #[tokio::test]
    async fn transfers_to_seller() {
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
        assert_eq!(0, asset_amount(&defaults.asset_id, &seller.wallet).await);

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

        let response = transfer_to_seller(&buyer.contract, 0).await;

        assert_eq!(
            defaults.asset_amount * 2,
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
                defaults.deadline,
                false,
                0,
                seller.wallet.address(),
                true
            )
            .await
        );

        let log = response
            .get_logs_with_type::<TransferredToSellerEvent>()
            .unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(*event, TransferredToSellerEvent { identifier: 0 });
    }

    #[tokio::test]
    async fn transfers_to_seller_after_proposing_arbiter() {
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
        deposit(
            defaults.asset_amount,
            &defaults.asset_id,
            &buyer.contract,
            0,
        )
        .await;
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
                defaults.deadline,
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

        let response = transfer_to_seller(&buyer.contract, 0).await;

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
                defaults.deadline,
                false,
                0,
                seller.wallet.address(),
                true
            )
            .await
        );
        assert!(matches!(arbiter_proposal(&seller.contract, 0).await, None));

        let log = response
            .get_logs_with_type::<TransferredToSellerEvent>()
            .unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(*event, TransferredToSellerEvent { identifier: 0 });
    }

    #[tokio::test]
    async fn transfers_to_seller_in_two_escrows() {
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
        assert_eq!(0, asset_amount(&defaults.asset_id, &seller.wallet).await);
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

        let response1 = transfer_to_seller(&buyer.contract, 0).await;

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
            defaults.asset_amount * 2,
            asset_amount(&defaults.asset_id, &seller.wallet).await
        );
        assert_eq!(0, asset_amount(&defaults.asset_id, &buyer.wallet).await);
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

        let response2 = transfer_to_seller(&buyer.contract, 1).await;

        assert_eq!(
            defaults.asset_amount * 4,
            asset_amount(&defaults.asset_id, &seller.wallet).await
        );
        assert_eq!(0, asset_amount(&defaults.asset_id, &buyer.wallet).await);
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
            .get_logs_with_type::<TransferredToSellerEvent>()
            .unwrap();
        let log2 = response2
            .get_logs_with_type::<TransferredToSellerEvent>()
            .unwrap();
        let event1 = log1.get(0).unwrap();
        let event2 = log2.get(0).unwrap();

        assert_eq!(*event1, TransferredToSellerEvent { identifier: 0 });
        assert_eq!(*event2, TransferredToSellerEvent { identifier: 1 });
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
        transfer_to_seller(&buyer.contract, 0).await;
        transfer_to_seller(&buyer.contract, 0).await;
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
        transfer_to_seller(&buyer.contract, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Unauthorized")]
    async fn when_caller_is_not_buyer() {
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
        transfer_to_seller(&seller.contract, 0).await;
    }
}

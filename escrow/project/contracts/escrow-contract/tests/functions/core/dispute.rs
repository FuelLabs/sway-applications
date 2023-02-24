use crate::utils::{
    interface::core::{create_escrow, deposit, dispute},
    setup::{create_arbiter, create_asset, mint, setup},
};

mod success {

    use super::*;
    use crate::utils::{
        interface::info::escrows,
        setup::{escrow_info, DisputeEvent},
    };

    #[tokio::test]
    async fn disputes() {
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

        let response = dispute(&buyer.contract, 0).await;

        assert_eq!(
            escrows(&seller.contract, 0).await.unwrap(),
            escrow_info(
                arbiter_obj.clone(),
                2,
                buyer.wallet.address(),
                Some(defaults.asset_id),
                defaults.asset_amount,
                defaults.deadline,
                true,
                0,
                seller.wallet.address(),
                false
            )
            .await
        );

        let log = response.get_logs_with_type::<DisputeEvent>().unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(*event, DisputeEvent { identifier: 0 });
    }

    #[tokio::test]
    async fn disputes_in_two_escrows() {
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

        let response1 = dispute(&buyer.contract, 0).await;
        let response2 = dispute(&buyer.contract, 1).await;

        assert_eq!(
            escrows(&seller.contract, 0).await.unwrap(),
            escrow_info(
                arbiter_obj.clone(),
                2,
                buyer.wallet.address(),
                Some(defaults.asset_id),
                defaults.asset_amount,
                defaults.deadline,
                true,
                0,
                seller.wallet.address(),
                false
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
                true,
                2,
                seller.wallet.address(),
                false
            )
            .await
        );

        let log1 = response1.get_logs_with_type::<DisputeEvent>().unwrap();
        let log2 = response2.get_logs_with_type::<DisputeEvent>().unwrap();
        let event1 = log1.get(0).unwrap();
        let event2 = log2.get(0).unwrap();

        assert_eq!(*event1, DisputeEvent { identifier: 0 });
        assert_eq!(*event2, DisputeEvent { identifier: 1 });
    }
}

mod revert {

    use super::*;
    use crate::utils::interface::core::transfer_to_seller;

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
        dispute(&buyer.contract, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "AlreadyDisputed")]
    async fn when_disputing_more_than_once() {
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
        dispute(&buyer.contract, 0).await;
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
        dispute(&seller.contract, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "CannotDisputeBeforeDesposit")]
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
        dispute(&buyer.contract, 0).await;
    }
}

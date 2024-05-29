use crate::utils::{
    interface::core::{create_escrow, deposit},
    setup::{create_arbiter, create_asset, setup},
};

mod success {

    use super::*;
    use crate::utils::{
        interface::info::escrows,
        setup::{asset_amount, DepositEvent},
    };

    #[tokio::test]
    async fn deposits() {
        let (arbiter, buyer, seller, defaults) = setup().await;
        let arbiter_obj = create_arbiter(&arbiter, defaults.asset_id, defaults.asset_amount).await;
        let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

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

        assert_eq!(
            defaults.initial_wallet_amount,
            asset_amount(&defaults.asset_id, &buyer).await
        );

        let escrow = escrows(&seller, 0).await.unwrap();
        assert!(escrow.buyer.asset.is_none());
        assert_eq!(0, escrow.buyer.deposited_amount);

        let response = deposit(defaults.asset_amount, &defaults.asset_id, &buyer, 0).await;

        assert_eq!(
            defaults.initial_wallet_amount - defaults.asset_amount,
            asset_amount(&defaults.asset_id, &buyer).await
        );

        let escrow = escrows(&seller, 0).await.unwrap().buyer;
        assert_eq!(escrow.asset, Some(defaults.asset_id));
        assert_eq!(defaults.asset_amount, escrow.deposited_amount);

        let log = response.decode_logs_with_type::<DepositEvent>().unwrap();
        let event = log.first().unwrap();

        assert_eq!(
            *event,
            DepositEvent {
                asset: defaults.asset_id,
                identifier: 0
            }
        );
    }

    #[tokio::test]
    async fn deposits_to_two_escrows() {
        let (arbiter, buyer, seller, defaults) = setup().await;
        let arbiter_obj = create_arbiter(&arbiter, defaults.asset_id, defaults.asset_amount).await;
        let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

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
        create_escrow(
            defaults.asset_amount,
            &arbiter_obj,
            &defaults.asset_id,
            vec![asset.clone(), asset.clone(), asset.clone()],
            &buyer,
            &seller,
            defaults.deadline,
        )
        .await;

        assert_eq!(
            defaults.initial_wallet_amount,
            asset_amount(&defaults.asset_id, &buyer).await
        );

        let escrow1 = escrows(&seller, 0).await.unwrap();
        let escrow2 = escrows(&seller, 0).await.unwrap();

        assert!(escrow1.buyer.asset.is_none());
        assert!(escrow2.buyer.asset.is_none());
        assert_eq!(0, escrow1.buyer.deposited_amount);
        assert_eq!(0, escrow2.buyer.deposited_amount);

        let response1 = deposit(defaults.asset_amount, &defaults.asset_id, &buyer, 0).await;

        let asset_amount1 = asset_amount(&defaults.asset_id, &buyer).await;

        let response2 = deposit(defaults.asset_amount, &defaults.asset_id, &buyer, 1).await;

        assert_eq!(
            defaults.initial_wallet_amount - defaults.asset_amount,
            asset_amount1
        );
        assert_eq!(
            defaults.initial_wallet_amount - (2 * defaults.asset_amount),
            asset_amount(&defaults.asset_id, &buyer).await
        );

        let escrow3 = escrows(&seller, 0).await.unwrap().buyer;
        let escrow4 = escrows(&seller, 0).await.unwrap().buyer;

        assert_eq!(escrow3.asset, Some(defaults.asset_id));
        assert_eq!(escrow4.asset, Some(defaults.asset_id));
        assert_eq!(defaults.asset_amount, escrow3.deposited_amount);
        assert_eq!(defaults.asset_amount, escrow4.deposited_amount);

        let log1 = response1.decode_logs_with_type::<DepositEvent>().unwrap();
        let log2 = response2.decode_logs_with_type::<DepositEvent>().unwrap();
        let event1 = log1.first().unwrap();
        let event2 = log2.first().unwrap();

        assert_eq!(
            *event1,
            DepositEvent {
                asset: defaults.asset_id,
                identifier: 0
            }
        );
        assert_eq!(
            *event2,
            DepositEvent {
                asset: defaults.asset_id,
                identifier: 1
            }
        );
    }
}

mod revert {

    use super::*;
    use crate::utils::interface::core::transfer_to_seller;

    #[tokio::test]
    #[should_panic(expected = "EscrowExpired")]
    async fn when_deadline_is_reached() {
        let (arbiter, buyer, seller, defaults) = setup().await;
        let arbiter_obj = create_arbiter(&arbiter, defaults.asset_id, defaults.asset_amount).await;
        let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

        create_escrow(
            defaults.asset_amount,
            &arbiter_obj,
            &defaults.asset_id,
            vec![asset.clone(), asset.clone()],
            &buyer,
            &seller,
            3,
        )
        .await;
        deposit(defaults.asset_amount, &defaults.asset_id, &buyer, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "StateNotPending")]
    async fn when_escrow_is_not_pending() {
        let (arbiter, buyer, seller, defaults) = setup().await;
        let arbiter_obj = create_arbiter(&arbiter, defaults.asset_id, defaults.asset_amount).await;
        let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

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
        deposit(defaults.asset_amount, &defaults.asset_id, &buyer, 0).await;
        transfer_to_seller(&buyer, 0).await;
        deposit(defaults.asset_amount, &defaults.asset_id, &buyer, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Unauthorized")]
    async fn when_caller_is_not_buyer() {
        let (arbiter, buyer, seller, defaults) = setup().await;
        let arbiter_obj = create_arbiter(&arbiter, defaults.asset_id, defaults.asset_amount).await;
        let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

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
        deposit(defaults.asset_amount, &defaults.asset_id, &seller, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "AlreadyDeposited")]
    async fn when_depositing_more_than_once() {
        let (arbiter, buyer, seller, defaults) = setup().await;
        let arbiter_obj = create_arbiter(&arbiter, defaults.asset_id, defaults.asset_amount).await;
        let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

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
        deposit(defaults.asset_amount, &defaults.asset_id, &buyer, 0).await;
        deposit(defaults.asset_amount, &defaults.asset_id, &buyer, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "IncorrectAssetAmount")]
    async fn when_incorrect_asset_amount_is_sent() {
        let (arbiter, buyer, seller, defaults) = setup().await;
        let arbiter_obj = create_arbiter(&arbiter, defaults.asset_id, defaults.asset_amount).await;
        let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

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
        deposit(defaults.asset_amount - 1, &defaults.asset_id, &buyer, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "IncorrectAssetSent")]
    async fn when_incorrect_asset_is_sent() {
        let (arbiter, buyer, seller, defaults) = setup().await;
        let arbiter_obj = create_arbiter(&arbiter, defaults.asset_id, defaults.asset_amount).await;
        let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

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
        deposit(defaults.asset_amount, &defaults.other_asset_id, &buyer, 0).await;
    }
}

use crate::utils::{
    interface::core::{create_escrow, deposit, transfer_to_seller},
    setup::{create_arbiter, create_asset, setup},
};

mod success {

    use super::*;
    use crate::utils::{
        interface::{
            core::propose_arbiter,
            info::{arbiter_proposal, escrows},
        },
        setup::{asset_amount, State, TransferredToSellerEvent},
    };

    #[tokio::test]
    async fn transfers_to_seller() {
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

        assert_eq!(
            defaults.initial_wallet_amount - defaults.asset_amount,
            asset_amount(&defaults.asset_id, &buyer).await
        );
        assert_eq!(
            defaults.initial_wallet_amount - defaults.asset_amount,
            asset_amount(&defaults.asset_id, &seller).await
        );
        assert!(matches!(
            escrows(&seller, 0).await.unwrap().state,
            State::Pending
        ));

        let response = transfer_to_seller(&buyer, 0).await;

        assert_eq!(
            defaults.initial_wallet_amount + defaults.asset_amount,
            asset_amount(&defaults.asset_id, &seller).await
        );
        assert_eq!(
            defaults.initial_wallet_amount - defaults.asset_amount,
            asset_amount(&defaults.asset_id, &buyer).await
        );
        assert!(matches!(
            escrows(&seller, 0).await.unwrap().state,
            State::Completed
        ));

        let log = response
            .decode_logs_with_type::<TransferredToSellerEvent>()
            .unwrap();
        let event = log.first().unwrap();

        assert_eq!(*event, TransferredToSellerEvent { identifier: 0 });
    }

    #[tokio::test]
    async fn transfers_to_seller_after_proposing_arbiter() {
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
        propose_arbiter(arbiter_obj.clone(), &seller, 0).await;

        assert_eq!(
            defaults.initial_wallet_amount - defaults.asset_amount,
            asset_amount(&defaults.asset_id, &buyer).await
        );
        assert_eq!(
            defaults.initial_wallet_amount - (2 * defaults.asset_amount),
            asset_amount(&defaults.asset_id, &seller).await
        );
        assert!(matches!(
            escrows(&seller, 0).await.unwrap().state,
            State::Pending
        ));
        assert_eq!(
            arbiter_obj.clone(),
            arbiter_proposal(&seller, 0).await.unwrap()
        );

        let response = transfer_to_seller(&buyer, 0).await;

        assert_eq!(
            defaults.initial_wallet_amount + defaults.asset_amount,
            asset_amount(&defaults.asset_id, &seller).await
        );
        assert_eq!(
            defaults.initial_wallet_amount - defaults.asset_amount,
            asset_amount(&defaults.asset_id, &buyer).await
        );
        assert!(matches!(
            escrows(&seller, 0).await.unwrap().state,
            State::Completed
        ));
        assert!(arbiter_proposal(&seller, 0).await.is_none());

        let log = response
            .decode_logs_with_type::<TransferredToSellerEvent>()
            .unwrap();
        let event = log.first().unwrap();

        assert_eq!(*event, TransferredToSellerEvent { identifier: 0 });
    }

    #[tokio::test]
    async fn transfers_to_seller_in_two_escrows() {
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
        deposit(defaults.asset_amount, &defaults.asset_id, &buyer, 1).await;

        assert_eq!(
            defaults.initial_wallet_amount - (2 * defaults.asset_amount),
            asset_amount(&defaults.asset_id, &buyer).await
        );
        assert_eq!(
            defaults.initial_wallet_amount - (2 * defaults.asset_amount),
            asset_amount(&defaults.asset_id, &seller).await
        );
        assert!(matches!(
            escrows(&seller, 0).await.unwrap().state,
            State::Pending
        ));
        assert!(matches!(
            escrows(&seller, 1).await.unwrap().state,
            State::Pending
        ));

        let response1 = transfer_to_seller(&buyer, 0).await;

        assert_eq!(
            defaults.initial_wallet_amount,
            asset_amount(&defaults.asset_id, &seller).await
        );
        assert_eq!(
            defaults.initial_wallet_amount - (2 * defaults.asset_amount),
            asset_amount(&defaults.asset_id, &buyer).await
        );
        assert!(matches!(
            escrows(&seller, 0).await.unwrap().state,
            State::Completed
        ));

        let response2 = transfer_to_seller(&buyer, 1).await;

        assert_eq!(
            defaults.initial_wallet_amount + (2 * defaults.asset_amount),
            asset_amount(&defaults.asset_id, &seller).await
        );
        assert_eq!(
            defaults.initial_wallet_amount - (2 * defaults.asset_amount),
            asset_amount(&defaults.asset_id, &buyer).await
        );
        assert!(matches!(
            escrows(&seller, 1).await.unwrap().state,
            State::Completed
        ));

        let log1 = response1
            .decode_logs_with_type::<TransferredToSellerEvent>()
            .unwrap();
        let log2 = response2
            .decode_logs_with_type::<TransferredToSellerEvent>()
            .unwrap();
        let event1 = log1.first().unwrap();
        let event2 = log2.first().unwrap();

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
        transfer_to_seller(&buyer, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "CannotTransferBeforeDeposit")]
    async fn when_buyer_has_not_deposited() {
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
        transfer_to_seller(&buyer, 0).await;
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
        deposit(defaults.asset_amount, &defaults.asset_id, &buyer, 0).await;
        transfer_to_seller(&seller, 0).await;
    }
}

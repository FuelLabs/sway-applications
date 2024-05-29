use crate::utils::{
    interface::core::{create_escrow, deposit, dispute, resolve_dispute},
    setup::{create_arbiter, create_asset, setup},
};

mod success {

    use super::*;
    use crate::utils::{
        interface::{
            core::propose_arbiter,
            info::{arbiter_proposal, escrows},
        },
        setup::{asset_amount, ResolvedDisputeEvent, State},
    };
    use fuels::{prelude::Address, types::Identity};

    #[tokio::test]
    async fn resolves_in_buyers_favour_full_payment_taken() {
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
        assert_eq!(
            defaults.initial_wallet_amount,
            asset_amount(&defaults.asset_id, &arbiter).await
        );

        dispute(&buyer, 0).await;

        assert!(matches!(
            escrows(&seller, 0).await.unwrap().state,
            State::Pending
        ));

        let response = resolve_dispute(&arbiter, 0, arbiter_obj.fee_amount, &buyer).await;

        assert_eq!(
            defaults.initial_wallet_amount - defaults.asset_amount,
            asset_amount(&defaults.asset_id, &seller).await
        );
        assert_eq!(
            defaults.initial_wallet_amount,
            asset_amount(&defaults.asset_id, &buyer).await
        );
        assert_eq!(
            defaults.initial_wallet_amount + defaults.asset_amount,
            asset_amount(&defaults.asset_id, &arbiter).await
        );
        assert!(matches!(
            escrows(&seller, 0).await.unwrap().state,
            State::Completed
        ));

        let log = response
            .decode_logs_with_type::<ResolvedDisputeEvent>()
            .unwrap();
        let event = log.first().unwrap();

        assert_eq!(
            *event,
            ResolvedDisputeEvent {
                identifier: 0,
                user: Identity::Address(Address::from(buyer.wallet.address()))
            }
        );
    }

    #[tokio::test]
    async fn resolves_in_buyers_favour_partial_payment_taken() {
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

        dispute(&buyer, 0).await;

        assert!(matches!(
            escrows(&seller, 0).await.unwrap().state,
            State::Pending
        ));

        let fee_amount = arbiter_obj.fee_amount - 1;
        let response = resolve_dispute(&arbiter, 0, fee_amount, &buyer).await;

        assert_eq!(
            defaults.initial_wallet_amount - fee_amount,
            asset_amount(&defaults.asset_id, &seller).await
        );
        assert_eq!(
            defaults.initial_wallet_amount,
            asset_amount(&defaults.asset_id, &buyer).await
        );
        assert_eq!(
            defaults.initial_wallet_amount + fee_amount,
            asset_amount(&defaults.asset_id, &arbiter).await
        );
        assert!(matches!(
            escrows(&seller, 0).await.unwrap().state,
            State::Completed
        ));

        let log = response
            .decode_logs_with_type::<ResolvedDisputeEvent>()
            .unwrap();
        let event = log.first().unwrap();

        assert_eq!(
            *event,
            ResolvedDisputeEvent {
                identifier: 0,
                user: Identity::Address(Address::from(buyer.wallet.address()))
            }
        );
    }

    #[tokio::test]
    async fn resolves_in_sellers_favour_full_payment_taken() {
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

        dispute(&buyer, 0).await;

        assert!(matches!(
            escrows(&seller, 0).await.unwrap().state,
            State::Pending
        ));

        let response = resolve_dispute(&arbiter, 0, arbiter_obj.fee_amount, &seller).await;

        assert_eq!(
            defaults.initial_wallet_amount,
            asset_amount(&defaults.asset_id, &seller).await
        );
        assert_eq!(
            defaults.initial_wallet_amount - defaults.asset_amount,
            asset_amount(&defaults.asset_id, &buyer).await
        );
        assert_eq!(
            defaults.initial_wallet_amount + defaults.asset_amount,
            asset_amount(&defaults.asset_id, &arbiter).await
        );
        assert!(matches!(
            escrows(&seller, 0).await.unwrap().state,
            State::Completed
        ));

        let log = response
            .decode_logs_with_type::<ResolvedDisputeEvent>()
            .unwrap();
        let event = log.first().unwrap();

        assert_eq!(
            *event,
            ResolvedDisputeEvent {
                identifier: 0,
                user: Identity::Address(Address::from(seller.wallet.address()))
            }
        );
    }

    #[tokio::test]
    async fn resolves_in_sellers_favour_partial_payment_taken() {
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

        dispute(&buyer, 0).await;

        assert!(matches!(
            escrows(&seller, 0).await.unwrap().state,
            State::Pending
        ));

        let fee_amount = arbiter_obj.fee_amount - 1;
        let response = resolve_dispute(&arbiter, 0, fee_amount, &seller).await;

        assert_eq!(
            defaults.initial_wallet_amount + 1,
            asset_amount(&defaults.asset_id, &seller).await
        );
        assert_eq!(
            defaults.initial_wallet_amount - defaults.asset_amount,
            asset_amount(&defaults.asset_id, &buyer).await
        );
        assert_eq!(
            defaults.initial_wallet_amount + fee_amount,
            asset_amount(&defaults.asset_id, &arbiter).await
        );
        assert!(matches!(
            escrows(&seller, 0).await.unwrap().state,
            State::Completed
        ));

        let log = response
            .decode_logs_with_type::<ResolvedDisputeEvent>()
            .unwrap();
        let event = log.first().unwrap();

        assert_eq!(
            *event,
            ResolvedDisputeEvent {
                identifier: 0,
                user: Identity::Address(Address::from(seller.wallet.address()))
            }
        );
    }

    #[tokio::test]
    async fn resolves_after_proposing_arbiter() {
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

        dispute(&buyer, 0).await;

        assert!(matches!(
            escrows(&seller, 0).await.unwrap().state,
            State::Pending
        ));
        assert_eq!(
            arbiter_obj.clone(),
            arbiter_proposal(&seller, 0).await.unwrap()
        );

        let response = resolve_dispute(&arbiter, 0, arbiter_obj.fee_amount, &buyer).await;

        assert_eq!(
            defaults.initial_wallet_amount - defaults.asset_amount,
            asset_amount(&defaults.asset_id, &seller).await
        );
        assert_eq!(
            defaults.initial_wallet_amount,
            asset_amount(&defaults.asset_id, &buyer).await
        );
        assert_eq!(
            defaults.initial_wallet_amount + defaults.asset_amount,
            asset_amount(&defaults.asset_id, &arbiter).await
        );
        assert!(arbiter_proposal(&seller, 0).await.is_none());
        assert!(matches!(
            escrows(&seller, 0).await.unwrap().state,
            State::Completed
        ));

        let log = response
            .decode_logs_with_type::<ResolvedDisputeEvent>()
            .unwrap();
        let event = log.first().unwrap();

        assert_eq!(
            *event,
            ResolvedDisputeEvent {
                identifier: 0,
                user: Identity::Address(Address::from(buyer.wallet.address()))
            }
        );
    }

    #[tokio::test]
    async fn resolves_in_two_escrows() {
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
        assert_eq!(
            defaults.initial_wallet_amount,
            asset_amount(&defaults.asset_id, &arbiter).await
        );

        dispute(&buyer, 0).await;

        assert!(matches!(
            escrows(&seller, 0).await.unwrap().state,
            State::Pending
        ));
        assert!(matches!(
            escrows(&seller, 1).await.unwrap().state,
            State::Pending
        ));

        let response1 = resolve_dispute(&arbiter, 0, arbiter_obj.fee_amount, &buyer).await;

        assert_eq!(
            defaults.initial_wallet_amount - (2 * defaults.asset_amount),
            asset_amount(&defaults.asset_id, &seller).await
        );
        assert_eq!(
            defaults.initial_wallet_amount - defaults.asset_amount,
            asset_amount(&defaults.asset_id, &buyer).await
        );
        assert_eq!(
            defaults.initial_wallet_amount + defaults.asset_amount,
            asset_amount(&defaults.asset_id, &arbiter).await
        );
        assert!(matches!(
            escrows(&seller, 0).await.unwrap().state,
            State::Completed
        ));

        dispute(&buyer, 1).await;

        let response2 = resolve_dispute(&arbiter, 1, arbiter_obj.fee_amount, &seller).await;

        assert_eq!(
            defaults.initial_wallet_amount - defaults.asset_amount,
            asset_amount(&defaults.asset_id, &seller).await
        );
        assert_eq!(
            defaults.initial_wallet_amount - defaults.asset_amount,
            asset_amount(&defaults.asset_id, &buyer).await
        );
        assert_eq!(
            defaults.initial_wallet_amount + (2 * defaults.asset_amount),
            asset_amount(&defaults.asset_id, &arbiter).await
        );
        assert!(matches!(
            escrows(&seller, 1).await.unwrap().state,
            State::Completed
        ));

        let log1 = response1
            .decode_logs_with_type::<ResolvedDisputeEvent>()
            .unwrap();
        let log2 = response2
            .decode_logs_with_type::<ResolvedDisputeEvent>()
            .unwrap();
        let event1 = log1.first().unwrap();
        let event2 = log2.first().unwrap();

        assert_eq!(
            *event1,
            ResolvedDisputeEvent {
                identifier: 0,
                user: Identity::Address(Address::from(buyer.wallet.address()))
            }
        );
        assert_eq!(
            *event2,
            ResolvedDisputeEvent {
                identifier: 1,
                user: Identity::Address(Address::from(seller.wallet.address()))
            }
        );
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
        dispute(&buyer, 0).await;
        resolve_dispute(&arbiter, 0, arbiter_obj.fee_amount, &buyer).await;
        resolve_dispute(&arbiter, 0, arbiter_obj.fee_amount, &buyer).await;
    }

    #[tokio::test]
    #[should_panic(expected = "NotDisputed")]
    async fn when_not_disputed() {
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
        resolve_dispute(&arbiter, 0, arbiter_obj.fee_amount, &buyer).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Unauthorized")]
    async fn when_caller_is_not_arbiter() {
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
        dispute(&buyer, 0).await;
        resolve_dispute(&buyer, 0, arbiter_obj.fee_amount, &buyer).await;
    }

    #[tokio::test]
    #[should_panic(expected = "InvalidRecipient")]
    async fn when_user_is_not_buyer_or_seller() {
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
        dispute(&buyer, 0).await;
        resolve_dispute(&arbiter, 0, arbiter_obj.fee_amount, &arbiter).await;
    }

    #[tokio::test]
    #[ignore]
    #[should_panic(expected = "CannotResolveBeforeDeposit")]
    async fn when_buyer_has_not_deposited() {
        // Note: Buyer can only dispute after they deposit and we cannot get past the require
        //       checks in resolve_dispute unless there is a dispute therefore this cannot
        //       actually be tested however for clarity & completeness this has been left in
    }

    #[tokio::test]
    #[should_panic(expected = "PaymentTooLarge")]
    async fn when_payment_amount_is_too_large() {
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
        dispute(&buyer, 0).await;
        resolve_dispute(&arbiter, 0, arbiter_obj.fee_amount + 1, &buyer).await;
    }
}

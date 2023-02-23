use crate::utils::{
    abi_calls::{create_escrow, deposit, dispute, propose_arbiter, resolve_dispute},
    test_helpers::{asset_amount, create_arbiter, create_asset, mint, setup},
};

mod success {

    use super::*;
    use crate::utils::ResolvedDisputeEvent;
    use fuels::{prelude::Address, types::Identity};

    #[tokio::test]
    async fn resolves_in_buyers_favour_full_payment_taken() {
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
        assert_eq!(0, asset_amount(&defaults.asset_id, &arbiter.wallet).await);

        dispute(&buyer.contract, 0).await;
        let response = resolve_dispute(
            &arbiter.contract,
            0,
            arbiter_obj.fee_amount,
            buyer.wallet.address(),
        )
        .await;
        let log = response
            .get_logs_with_type::<ResolvedDisputeEvent>()
            .unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(
            *event,
            ResolvedDisputeEvent {
                identifier: 0,
                user: Identity::Address(Address::from(buyer.wallet.address()))
            }
        );
        assert_eq!(0, asset_amount(&defaults.asset_id, &seller.wallet).await);
        assert_eq!(
            defaults.asset_amount,
            asset_amount(&defaults.asset_id, &buyer.wallet).await
        );
        assert_eq!(
            defaults.asset_amount,
            asset_amount(&defaults.asset_id, &arbiter.wallet).await
        );
    }

    #[tokio::test]
    async fn resolves_in_buyers_favour_partial_payment_taken() {
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

        dispute(&buyer.contract, 0).await;
        let response = resolve_dispute(
            &arbiter.contract,
            0,
            arbiter_obj.fee_amount - 1,
            buyer.wallet.address(),
        )
        .await;
        let log = response
            .get_logs_with_type::<ResolvedDisputeEvent>()
            .unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(
            *event,
            ResolvedDisputeEvent {
                identifier: 0,
                user: Identity::Address(Address::from(buyer.wallet.address()))
            }
        );

        assert_eq!(1, asset_amount(&defaults.asset_id, &seller.wallet).await);
        assert_eq!(
            defaults.asset_amount,
            asset_amount(&defaults.asset_id, &buyer.wallet).await
        );
        assert_eq!(
            defaults.asset_amount - 1,
            asset_amount(&defaults.asset_id, &arbiter.wallet).await
        );
    }

    #[tokio::test]
    async fn resolves_in_sellers_favour_full_payment_taken() {
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

        dispute(&buyer.contract, 0).await;
        let response = resolve_dispute(
            &arbiter.contract,
            0,
            arbiter_obj.fee_amount,
            seller.wallet.address(),
        )
        .await;
        let log = response
            .get_logs_with_type::<ResolvedDisputeEvent>()
            .unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(
            *event,
            ResolvedDisputeEvent {
                identifier: 0,
                user: Identity::Address(Address::from(seller.wallet.address()))
            }
        );
        assert_eq!(
            defaults.asset_amount,
            asset_amount(&defaults.asset_id, &seller.wallet).await
        );
        assert_eq!(0, asset_amount(&defaults.asset_id, &buyer.wallet).await);
        assert_eq!(
            defaults.asset_amount,
            asset_amount(&defaults.asset_id, &arbiter.wallet).await
        );
    }

    #[tokio::test]
    async fn resolves_in_sellers_favour_partial_payment_taken() {
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

        dispute(&buyer.contract, 0).await;
        let response = resolve_dispute(
            &arbiter.contract,
            0,
            arbiter_obj.fee_amount - 1,
            seller.wallet.address(),
        )
        .await;
        let log = response
            .get_logs_with_type::<ResolvedDisputeEvent>()
            .unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(
            *event,
            ResolvedDisputeEvent {
                identifier: 0,
                user: Identity::Address(Address::from(seller.wallet.address()))
            }
        );
        assert_eq!(
            defaults.asset_amount + 1,
            asset_amount(&defaults.asset_id, &seller.wallet).await
        );
        assert_eq!(0, asset_amount(&defaults.asset_id, &buyer.wallet).await);
        assert_eq!(
            defaults.asset_amount - 1,
            asset_amount(&defaults.asset_id, &arbiter.wallet).await
        );
    }

    #[tokio::test]
    async fn resolves_after_proposing_arbiter() {
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

        dispute(&buyer.contract, 0).await;
        let response = resolve_dispute(
            &arbiter.contract,
            0,
            arbiter_obj.fee_amount,
            buyer.wallet.address(),
        )
        .await;
        let log = response
            .get_logs_with_type::<ResolvedDisputeEvent>()
            .unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(
            *event,
            ResolvedDisputeEvent {
                identifier: 0,
                user: Identity::Address(Address::from(buyer.wallet.address()))
            }
        );
        assert_eq!(
            defaults.asset_amount,
            asset_amount(&defaults.asset_id, &seller.wallet).await
        );
        assert_eq!(
            defaults.asset_amount,
            asset_amount(&defaults.asset_id, &buyer.wallet).await
        );
        assert_eq!(
            defaults.asset_amount,
            asset_amount(&defaults.asset_id, &arbiter.wallet).await
        );
    }

    #[tokio::test]
    async fn resolves_in_two_escrows() {
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
        assert_eq!(0, asset_amount(&defaults.asset_id, &arbiter.wallet).await);

        dispute(&buyer.contract, 0).await;
        let response = resolve_dispute(
            &arbiter.contract,
            0,
            arbiter_obj.fee_amount,
            buyer.wallet.address(),
        )
        .await;
        let log = response
            .get_logs_with_type::<ResolvedDisputeEvent>()
            .unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(
            *event,
            ResolvedDisputeEvent {
                identifier: 0,
                user: Identity::Address(Address::from(buyer.wallet.address()))
            }
        );
        assert_eq!(0, asset_amount(&defaults.asset_id, &seller.wallet).await);
        assert_eq!(
            defaults.asset_amount,
            asset_amount(&defaults.asset_id, &buyer.wallet).await
        );
        assert_eq!(
            defaults.asset_amount,
            asset_amount(&defaults.asset_id, &arbiter.wallet).await
        );

        dispute(&buyer.contract, 1).await;
        let response = resolve_dispute(
            &arbiter.contract,
            1,
            arbiter_obj.fee_amount,
            seller.wallet.address(),
        )
        .await;
        let log = response
            .get_logs_with_type::<ResolvedDisputeEvent>()
            .unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(
            *event,
            ResolvedDisputeEvent {
                identifier: 1,
                user: Identity::Address(Address::from(seller.wallet.address()))
            }
        );
        assert_eq!(
            defaults.asset_amount,
            asset_amount(&defaults.asset_id, &seller.wallet).await
        );
        assert_eq!(
            defaults.asset_amount,
            asset_amount(&defaults.asset_id, &buyer.wallet).await
        );
        assert_eq!(
            defaults.asset_amount * 2,
            asset_amount(&defaults.asset_id, &arbiter.wallet).await
        );
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

        dispute(&buyer.contract, 0).await;

        resolve_dispute(
            &arbiter.contract,
            0,
            arbiter_obj.fee_amount,
            buyer.wallet.address(),
        )
        .await;
        resolve_dispute(
            &arbiter.contract,
            0,
            arbiter_obj.fee_amount,
            buyer.wallet.address(),
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "NotDisputed")]
    async fn when_not_disputed() {
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
        resolve_dispute(
            &arbiter.contract,
            0,
            arbiter_obj.fee_amount,
            buyer.wallet.address(),
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Unauthorized")]
    async fn when_caller_is_not_arbiter() {
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
        resolve_dispute(
            &buyer.contract,
            0,
            arbiter_obj.fee_amount,
            buyer.wallet.address(),
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "InvalidRecipient")]
    async fn when_user_is_not_buyer_or_seller() {
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
        resolve_dispute(
            &arbiter.contract,
            0,
            arbiter_obj.fee_amount,
            arbiter.wallet.address(),
        )
        .await;
    }

    #[tokio::test]
    #[ignore]
    #[should_panic(expected = "CannotResolveBeforeDesposit")]
    async fn when_buyer_has_not_deposited() {
        // Note: Buyer can only dispute after they deposit and we cannot get past the require
        //       checks in resolve_dispute unless there is a dispute therefore this cannot
        //       actually be tested however for clarity & completeness this has been left in
    }

    #[tokio::test]
    #[should_panic(expected = "PaymentTooLarge")]
    async fn when_payment_amount_is_too_large() {
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
        resolve_dispute(
            &arbiter.contract,
            0,
            arbiter_obj.fee_amount + 1,
            buyer.wallet.address(),
        )
        .await;
    }
}

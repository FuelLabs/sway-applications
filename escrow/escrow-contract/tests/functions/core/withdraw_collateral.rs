use crate::utils::{
    interface::core::{create_escrow, deposit, withdraw_collateral},
    setup::{create_arbiter, create_asset, setup},
};

mod success {

    use super::*;
    use crate::utils::{
        interface::{
            core::propose_arbiter,
            info::{arbiter_proposal, escrows},
        },
        setup::{asset_amount, State, WithdrawnCollateralEvent},
    };

    #[tokio::test]
    async fn withdraws_collateral() {
        let (arbiter, buyer, seller, defaults) = setup().await;
        let arbiter_obj = create_arbiter(&arbiter, defaults.asset_id, defaults.asset_amount).await;
        let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

        let provider = buyer.wallet.provider().unwrap();
        let origin_block = provider.latest_block_height().await.unwrap();

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
            defaults.initial_wallet_amount - defaults.asset_amount,
            asset_amount(&defaults.asset_id, &seller).await
        );
        assert!(matches!(
            escrows(&seller, 0).await.unwrap().state,
            State::Pending
        ));

        provider
            .produce_blocks(
                ((origin_block as u64) + defaults.deadline)
                    .try_into()
                    .unwrap(),
                None,
            )
            .await
            .unwrap();

        let response = withdraw_collateral(&seller, 0).await;

        assert_eq!(
            defaults.initial_wallet_amount,
            asset_amount(&defaults.asset_id, &seller).await
        );
        assert!(matches!(
            escrows(&seller, 0).await.unwrap().state,
            State::Completed
        ));

        let log = response
            .decode_logs_with_type::<WithdrawnCollateralEvent>()
            .unwrap();
        let event = log.first().unwrap();

        assert_eq!(*event, WithdrawnCollateralEvent { identifier: 0 });
    }

    #[tokio::test]
    async fn withdraws_collateral_after_proposing_arbiter() {
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
            5,
        )
        .await;
        propose_arbiter(arbiter_obj.clone(), &seller, 0).await;

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

        let response = withdraw_collateral(&seller, 0).await;

        assert_eq!(
            defaults.initial_wallet_amount,
            asset_amount(&defaults.asset_id, &seller).await
        );
        assert!(matches!(
            escrows(&seller, 0).await.unwrap().state,
            State::Completed
        ));
        assert!(arbiter_proposal(&seller, 0).await.is_none());

        let log = response
            .decode_logs_with_type::<WithdrawnCollateralEvent>()
            .unwrap();
        let event = log.first().unwrap();

        assert_eq!(*event, WithdrawnCollateralEvent { identifier: 0 });
    }

    #[tokio::test]
    async fn withdraws_collateral_in_two_escrows() {
        let (arbiter, buyer, seller, defaults) = setup().await;
        let arbiter_obj = create_arbiter(&arbiter, defaults.asset_id, defaults.asset_amount).await;
        let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

        let provider = buyer.wallet.provider().unwrap();
        let origin_block = provider.latest_block_height().await.unwrap();

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
        let escrow_id_0 = 0;
        assert!(matches!(
            escrows(&seller, escrow_id_0).await.unwrap().state,
            State::Pending
        ));

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
        let escrow_id_1 = 1;
        assert!(matches!(
            escrows(&seller, escrow_id_1).await.unwrap().state,
            State::Pending
        ));

        assert_eq!(
            defaults.initial_wallet_amount - (2 * defaults.asset_amount),
            asset_amount(&defaults.asset_id, &seller).await
        );

        provider
            .produce_blocks(
                ((origin_block as u64) + defaults.deadline)
                    .try_into()
                    .unwrap(),
                None,
            )
            .await
            .unwrap();

        let response0 = withdraw_collateral(&seller, escrow_id_0).await;
        let response1 = withdraw_collateral(&seller, escrow_id_1).await;

        assert_eq!(
            defaults.initial_wallet_amount,
            asset_amount(&defaults.asset_id, &seller).await
        );
        assert!(matches!(
            escrows(&seller, escrow_id_0).await.unwrap().state,
            State::Completed
        ));
        assert!(matches!(
            escrows(&seller, escrow_id_1).await.unwrap().state,
            State::Completed
        ));

        let log0 = response0
            .decode_logs_with_type::<WithdrawnCollateralEvent>()
            .unwrap();
        let event0 = log0.first().unwrap();
        assert_eq!(
            *event0,
            WithdrawnCollateralEvent {
                identifier: escrow_id_0
            }
        );

        let log1 = response1
            .decode_logs_with_type::<WithdrawnCollateralEvent>()
            .unwrap();
        let event1 = log1.first().unwrap();
        assert_eq!(
            *event1,
            WithdrawnCollateralEvent {
                identifier: escrow_id_1
            }
        );
    }
}

mod revert {

    use super::*;
    use crate::utils::interface::core::return_deposit;

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
        return_deposit(&seller, 0).await;
        withdraw_collateral(&seller, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "CannotWithdrawBeforeDeadline")]
    async fn when_deadline_is_not_in_the_past() {
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
        withdraw_collateral(&seller, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Unauthorized")]
    async fn when_caller_is_not_seller() {
        let (arbiter, buyer, seller, defaults) = setup().await;
        let arbiter_obj = create_arbiter(&arbiter, defaults.asset_id, defaults.asset_amount).await;
        let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

        let provider = buyer.wallet.provider().unwrap();
        let origin_block = provider.latest_block_height().await.unwrap();

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

        provider
            .produce_blocks(
                ((origin_block as u64) + defaults.deadline)
                    .try_into()
                    .unwrap(),
                None,
            )
            .await
            .unwrap();

        withdraw_collateral(&buyer, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "CannotWithdrawAfterDeposit")]
    async fn when_buyer_has_deposited() {
        let (arbiter, buyer, seller, defaults) = setup().await;
        let arbiter_obj = create_arbiter(&arbiter, defaults.asset_id, defaults.asset_amount).await;
        let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

        let provider = buyer.wallet.provider().unwrap();
        let origin_block = provider.latest_block_height().await.unwrap();

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
        provider
            .produce_blocks(
                ((origin_block as u64) + defaults.deadline)
                    .try_into()
                    .unwrap(),
                None,
            )
            .await
            .unwrap();

        withdraw_collateral(&seller, 0).await;
    }
}

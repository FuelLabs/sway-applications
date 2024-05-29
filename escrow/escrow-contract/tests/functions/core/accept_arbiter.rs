use crate::utils::{
    interface::{
        core::{accept_arbiter, create_escrow, deposit},
        info::arbiter_proposal,
    },
    setup::{create_arbiter, create_asset, setup},
};

mod success {
    use super::*;
    use crate::utils::{
        interface::{core::propose_arbiter, info::escrows},
        setup::{asset_amount, AcceptedArbiterEvent},
    };

    #[tokio::test]
    async fn accepts_proposal() {
        let (arbiter, buyer, seller, defaults) = setup().await;

        let arbiter_obj = create_arbiter(&arbiter, defaults.asset_id, defaults.asset_amount).await;
        let arbiter_obj2 =
            create_arbiter(&arbiter, defaults.asset_id, defaults.asset_amount - 1).await;
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
        propose_arbiter(arbiter_obj2.clone(), &seller, 0).await;

        let initial_amount = asset_amount(&defaults.asset_id, &seller).await;
        let initial_escrow = escrows(&seller, 0).await.unwrap();
        let initial_proposal = arbiter_proposal(&seller, 0).await.unwrap();

        let response = accept_arbiter(&buyer, 0).await;

        assert_eq!(
            defaults.initial_wallet_amount - (defaults.asset_amount + arbiter_obj2.fee_amount),
            initial_amount
        );
        assert_eq!(arbiter_obj, initial_escrow.arbiter);
        assert_eq!(arbiter_obj2.clone(), initial_proposal);
        assert_eq!(
            defaults.initial_wallet_amount - arbiter_obj2.fee_amount,
            asset_amount(&defaults.asset_id, &seller).await
        );
        assert_eq!(arbiter_obj2, escrows(&seller, 0).await.unwrap().arbiter);
        assert!(arbiter_proposal(&seller, 0).await.is_none());

        let log = response
            .decode_logs_with_type::<AcceptedArbiterEvent>()
            .unwrap();
        let event = log.first().unwrap();

        assert_eq!(*event, AcceptedArbiterEvent { identifier: 0 });
    }

    #[tokio::test]
    async fn accepts_proposal_in_two_escrows() {
        let (arbiter, buyer, seller, defaults) = setup().await;

        let arbiter_obj = create_arbiter(&arbiter, defaults.asset_id, defaults.asset_amount).await;
        let arbiter_obj2 =
            create_arbiter(&arbiter, defaults.asset_id, defaults.asset_amount - 1).await;
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

        propose_arbiter(arbiter_obj2.clone(), &seller, 0).await;
        propose_arbiter(arbiter_obj2.clone(), &seller, 1).await;

        assert_eq!(
            defaults.initial_wallet_amount
                - (2 * (defaults.asset_amount + arbiter_obj2.fee_amount)),
            asset_amount(&defaults.asset_id, &seller).await
        );
        assert_eq!(arbiter_obj2, arbiter_proposal(&seller, 0).await.unwrap());
        assert_eq!(arbiter_obj2, arbiter_proposal(&seller, 1).await.unwrap());
        assert_eq!(
            arbiter_obj.clone(),
            escrows(&seller, 0).await.unwrap().arbiter
        );
        assert_eq!(
            arbiter_obj.clone(),
            escrows(&seller, 1).await.unwrap().arbiter
        );

        let response1 = accept_arbiter(&buyer, 0).await;
        let asset_amount1 = asset_amount(&defaults.asset_id, &seller).await;
        let response2 = accept_arbiter(&buyer, 1).await;

        assert_eq!(
            defaults.initial_wallet_amount
                - ((2 * arbiter_obj2.fee_amount) + defaults.asset_amount),
            asset_amount1
        );
        assert_eq!(
            defaults.initial_wallet_amount - (2 * arbiter_obj2.fee_amount),
            asset_amount(&defaults.asset_id, &seller).await
        );
        assert_eq!(arbiter_proposal(&seller, 0).await, None);
        assert_eq!(arbiter_proposal(&seller, 1).await, None);
        assert_eq!(
            arbiter_obj2.clone(),
            escrows(&seller, 0).await.unwrap().arbiter,
        );
        assert_eq!(arbiter_obj2, escrows(&seller, 1).await.unwrap().arbiter,);

        let log1 = response1
            .decode_logs_with_type::<AcceptedArbiterEvent>()
            .unwrap();
        let log2 = response2
            .decode_logs_with_type::<AcceptedArbiterEvent>()
            .unwrap();

        let event1 = log1.first().unwrap();
        let event2 = log2.first().unwrap();

        assert_eq!(*event1, AcceptedArbiterEvent { identifier: 0 });
        assert_eq!(*event2, AcceptedArbiterEvent { identifier: 1 });
    }
}

mod revert {

    use super::*;
    use crate::utils::interface::core::transfer_to_seller;

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
        accept_arbiter(&buyer, 0).await;
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
        accept_arbiter(&seller, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "ArbiterHasNotBeenProposed")]
    async fn when_arbiter_proposal_is_not_set() {
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
        accept_arbiter(&buyer, 0).await;
    }
}

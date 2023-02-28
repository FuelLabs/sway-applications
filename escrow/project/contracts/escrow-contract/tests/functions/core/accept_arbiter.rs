use crate::utils::{
    interface::{
        core::{accept_arbiter, create_escrow, deposit},
        info::arbiter_proposal,
    },
    setup::{create_arbiter, create_asset, mint, setup},
};

mod success {

    use super::*;
    use crate::utils::{
        interface::{core::propose_arbiter, info::escrows},
        setup::{asset_amount, escrow_info, AcceptedArbiterEvent},
    };

    #[tokio::test]
    async fn accepts_proposal() {
        let (arbiter, buyer, seller, defaults) = setup().await;
        let arbiter_obj = create_arbiter(
            arbiter.wallet.address(),
            defaults.asset_id,
            defaults.asset_amount,
        )
        .await;
        let arbiter_obj2 = create_arbiter(
            arbiter.wallet.address(),
            defaults.asset_id,
            defaults.asset_amount - 1,
        )
        .await;
        let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

        mint(
            seller.wallet.address(),
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

        propose_arbiter(arbiter_obj2.clone(), &seller.contract, 0).await;

        assert_eq!(1, asset_amount(&defaults.asset_id, &seller.wallet).await);
        assert_eq!(
            escrows(&seller.contract, 0).await.unwrap(),
            escrow_info(
                arbiter_obj.clone(),
                2,
                buyer.wallet.address(),
                None,
                0,
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
            arbiter_obj2.clone()
        );

        let response = accept_arbiter(&buyer.contract, 0).await;

        assert_eq!(
            defaults.asset_amount + 1,
            asset_amount(&defaults.asset_id, &seller.wallet).await
        );
        assert_eq!(
            escrows(&seller.contract, 0).await.unwrap(),
            escrow_info(
                arbiter_obj2,
                2,
                buyer.wallet.address(),
                None,
                0,
                defaults.deadline,
                false,
                0,
                seller.wallet.address(),
                false
            )
            .await
        );
        assert!(matches!(arbiter_proposal(&seller.contract, 0).await, None));

        let log = response
            .get_logs_with_type::<AcceptedArbiterEvent>()
            .unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(*event, AcceptedArbiterEvent { identifier: 0 });
    }

    #[tokio::test]
    async fn accepts_proposal_in_two_escrows() {
        let (arbiter, buyer, seller, defaults) = setup().await;
        let arbiter_obj = create_arbiter(
            arbiter.wallet.address(),
            defaults.asset_id,
            defaults.asset_amount,
        )
        .await;
        let arbiter_obj2 = create_arbiter(
            arbiter.wallet.address(),
            defaults.asset_id,
            defaults.asset_amount - 1,
        )
        .await;
        let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

        mint(
            seller.wallet.address(),
            defaults.asset_amount * 4,
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

        propose_arbiter(arbiter_obj2.clone(), &seller.contract, 0).await;
        propose_arbiter(arbiter_obj2.clone(), &seller.contract, 1).await;

        assert_eq!(2, asset_amount(&defaults.asset_id, &seller.wallet).await);

        assert_eq!(
            arbiter_proposal(&seller.contract, 0).await.unwrap(),
            arbiter_obj2
        );
        assert_eq!(
            arbiter_proposal(&seller.contract, 1).await.unwrap(),
            arbiter_obj2
        );

        assert_eq!(
            escrows(&seller.contract, 0).await.unwrap(),
            escrow_info(
                arbiter_obj.clone(),
                2,
                buyer.wallet.address(),
                None,
                0,
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
                None,
                0,
                defaults.deadline,
                false,
                2,
                seller.wallet.address(),
                false
            )
            .await
        );

        let response1 = accept_arbiter(&buyer.contract, 0).await;
        let asset_amount1 = asset_amount(&defaults.asset_id, &seller.wallet).await;
        let response2 = accept_arbiter(&buyer.contract, 1).await;

        assert_eq!(defaults.asset_amount + 2, asset_amount1);
        assert_eq!(
            defaults.asset_amount * 2 + 2,
            asset_amount(&defaults.asset_id, &seller.wallet).await
        );

        assert_eq!(arbiter_proposal(&seller.contract, 0).await, None);
        assert_eq!(arbiter_proposal(&seller.contract, 1).await, None);

        assert_eq!(
            escrows(&seller.contract, 0).await.unwrap(),
            escrow_info(
                arbiter_obj2.clone(),
                2,
                buyer.wallet.address(),
                None,
                0,
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
                arbiter_obj2,
                2,
                buyer.wallet.address(),
                None,
                0,
                defaults.deadline,
                false,
                2,
                seller.wallet.address(),
                false
            )
            .await
        );

        let log1 = response1
            .get_logs_with_type::<AcceptedArbiterEvent>()
            .unwrap();
        let log2 = response2
            .get_logs_with_type::<AcceptedArbiterEvent>()
            .unwrap();

        let event1 = log1.get(0).unwrap();
        let event2 = log2.get(0).unwrap();

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
        transfer_to_seller(&buyer.contract, 0).await;
        accept_arbiter(&buyer.contract, 0).await;
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
        accept_arbiter(&seller.contract, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "ArbiterHasNotBeenProposed")]
    async fn when_arbiter_proposal_is_not_set() {
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
        accept_arbiter(&buyer.contract, 0).await;
    }
}

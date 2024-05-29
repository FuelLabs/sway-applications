use crate::utils::{
    interface::core::{create_escrow, deposit, propose_arbiter},
    setup::{create_arbiter, create_asset, setup},
};
use fuels::{
    prelude::{AssetId, CallParameters, TxPolicies},
    programs::call_utils::TxDependencyExtension,
};

mod success {

    use super::*;
    use crate::utils::{
        interface::info::arbiter_proposal,
        setup::{asset_amount, ProposedArbiterEvent},
    };

    #[tokio::test]
    async fn proposes_arbiter() {
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
            defaults.initial_wallet_amount - defaults.asset_amount,
            asset_amount(&defaults.asset_id, &seller).await
        );
        assert!(arbiter_proposal(&seller, 0).await.is_none());

        let response = propose_arbiter(arbiter_obj.clone(), &seller, 0).await;

        assert_eq!(
            defaults.initial_wallet_amount - (2 * defaults.asset_amount),
            asset_amount(&defaults.asset_id, &seller).await
        );
        assert_eq!(arbiter_proposal(&seller, 0).await.unwrap(), arbiter_obj);

        let log = response
            .decode_logs_with_type::<ProposedArbiterEvent>()
            .unwrap();
        let event = log.first().unwrap();

        assert_eq!(
            *event,
            ProposedArbiterEvent {
                arbiter: arbiter_obj,
                identifier: 0
            }
        );
    }

    #[tokio::test]
    async fn proposes_arbiter_twice() {
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

        assert_eq!(
            defaults.initial_wallet_amount - defaults.asset_amount,
            asset_amount(&defaults.asset_id, &seller).await
        );
        assert!(arbiter_proposal(&seller, 0).await.is_none());

        let response1 = propose_arbiter(arbiter_obj.clone(), &seller, 0).await;

        assert_eq!(
            defaults.initial_wallet_amount - (2 * defaults.asset_amount),
            asset_amount(&defaults.asset_id, &seller).await
        );
        assert_eq!(arbiter_proposal(&seller, 0).await.unwrap(), arbiter_obj);

        let response2 = propose_arbiter(arbiter_obj2.clone(), &seller, 0).await;

        assert_eq!(
            defaults.initial_wallet_amount - defaults.asset_amount - arbiter_obj2.fee_amount,
            asset_amount(&defaults.asset_id, &seller).await
        );
        assert_eq!(arbiter_proposal(&seller, 0).await.unwrap(), arbiter_obj2);

        let log1 = response1
            .decode_logs_with_type::<ProposedArbiterEvent>()
            .unwrap();
        let log2 = response2
            .decode_logs_with_type::<ProposedArbiterEvent>()
            .unwrap();
        let event1 = log1.first().unwrap();
        let event2 = log2.first().unwrap();

        assert_eq!(
            *event1,
            ProposedArbiterEvent {
                arbiter: arbiter_obj.clone(),
                identifier: 0
            }
        );
        assert_eq!(
            *event2,
            ProposedArbiterEvent {
                arbiter: arbiter_obj2,
                identifier: 0
            }
        );
    }

    #[tokio::test]
    async fn proposes_arbiter_in_two_escrows() {
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

        assert_eq!(
            defaults.initial_wallet_amount - (2 * defaults.asset_amount),
            asset_amount(&defaults.asset_id, &seller).await
        );
        assert!(arbiter_proposal(&seller, 0).await.is_none());
        assert!(arbiter_proposal(&seller, 1).await.is_none());

        let response1 = propose_arbiter(arbiter_obj.clone(), &seller, 0).await;

        assert_eq!(
            defaults.initial_wallet_amount - (3 * defaults.asset_amount),
            asset_amount(&defaults.asset_id, &seller).await
        );

        let response2 = propose_arbiter(arbiter_obj.clone(), &seller, 1).await;

        assert_eq!(
            defaults.initial_wallet_amount - (4 * defaults.asset_amount),
            asset_amount(&defaults.asset_id, &seller).await
        );
        assert_eq!(
            arbiter_obj.clone(),
            arbiter_proposal(&seller, 0).await.unwrap()
        );
        assert_eq!(
            arbiter_obj.clone(),
            arbiter_proposal(&seller, 1).await.unwrap()
        );

        let log1 = response1
            .decode_logs_with_type::<ProposedArbiterEvent>()
            .unwrap();
        let log2 = response2
            .decode_logs_with_type::<ProposedArbiterEvent>()
            .unwrap();
        let event1 = log1.first().unwrap();
        let event2 = log2.first().unwrap();

        assert_eq!(
            *event1,
            ProposedArbiterEvent {
                arbiter: arbiter_obj.clone(),
                identifier: 0
            }
        );
        assert_eq!(
            *event2,
            ProposedArbiterEvent {
                arbiter: arbiter_obj,
                identifier: 1
            }
        );
    }

    #[tokio::test]
    async fn proposes_arbiter_in_two_escrows_twice() {
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

        assert_eq!(
            defaults.initial_wallet_amount - (2 * defaults.asset_amount),
            asset_amount(&defaults.asset_id, &seller).await
        );
        assert!(arbiter_proposal(&seller, 0).await.is_none());
        assert!(arbiter_proposal(&seller, 1).await.is_none());

        let response1 = propose_arbiter(arbiter_obj.clone(), &seller, 0).await;

        assert_eq!(
            defaults.initial_wallet_amount - (3 * defaults.asset_amount),
            asset_amount(&defaults.asset_id, &seller).await
        );
        assert_eq!(
            arbiter_proposal(&seller, 0).await.unwrap(),
            arbiter_obj.clone()
        );

        let response2 = propose_arbiter(arbiter_obj.clone(), &seller, 1).await;

        assert_eq!(
            defaults.initial_wallet_amount - (4 * defaults.asset_amount),
            asset_amount(&defaults.asset_id, &seller).await
        );
        assert_eq!(
            arbiter_proposal(&seller, 1).await.unwrap(),
            arbiter_obj.clone()
        );

        let response3 = propose_arbiter(arbiter_obj2.clone(), &seller, 0).await;

        assert_eq!(
            defaults.initial_wallet_amount - (3 * defaults.asset_amount) - arbiter_obj2.fee_amount,
            asset_amount(&defaults.asset_id, &seller).await
        );
        assert_eq!(
            arbiter_proposal(&seller, 0).await.unwrap(),
            arbiter_obj2.clone()
        );

        let response4 = propose_arbiter(arbiter_obj2.clone(), &seller, 1).await;

        assert_eq!(
            defaults.initial_wallet_amount
                - (2 * defaults.asset_amount)
                - (2 * arbiter_obj2.fee_amount),
            asset_amount(&defaults.asset_id, &seller).await
        );
        assert_eq!(
            arbiter_proposal(&seller, 1).await.unwrap(),
            arbiter_obj2.clone()
        );

        let log1 = response1
            .decode_logs_with_type::<ProposedArbiterEvent>()
            .unwrap();
        let log2 = response2
            .decode_logs_with_type::<ProposedArbiterEvent>()
            .unwrap();
        let log3 = response3
            .decode_logs_with_type::<ProposedArbiterEvent>()
            .unwrap();
        let log4 = response4
            .decode_logs_with_type::<ProposedArbiterEvent>()
            .unwrap();
        let event1 = log1.first().unwrap();
        let event2 = log2.first().unwrap();
        let event3 = log3.first().unwrap();
        let event4 = log4.first().unwrap();

        assert_eq!(
            *event1,
            ProposedArbiterEvent {
                arbiter: arbiter_obj.clone(),
                identifier: 0
            }
        );
        assert_eq!(
            *event2,
            ProposedArbiterEvent {
                arbiter: arbiter_obj.clone(),
                identifier: 1
            }
        );
        assert_eq!(
            *event3,
            ProposedArbiterEvent {
                arbiter: arbiter_obj2.clone(),
                identifier: 0
            }
        );
        assert_eq!(
            *event4,
            ProposedArbiterEvent {
                arbiter: arbiter_obj2,
                identifier: 1
            }
        );
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
        propose_arbiter(arbiter_obj, &seller, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Unauthorized")]
    async fn when_caller_is_not_seller() {
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
        propose_arbiter(arbiter_obj, &buyer, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "CannotBeBuyer")]
    async fn when_arbiter_address_is_set_to_buyer() {
        let (arbiter, buyer, seller, defaults) = setup().await;
        let arbiter_obj = create_arbiter(&arbiter, defaults.asset_id, defaults.asset_amount).await;
        let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;
        let arbiter_obj_buyer =
            create_arbiter(&buyer, defaults.asset_id, defaults.asset_amount).await;

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
        propose_arbiter(arbiter_obj_buyer, &seller, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "CannotBeSeller")]
    async fn when_arbiter_address_is_set_to_seller() {
        let (arbiter, buyer, seller, defaults) = setup().await;
        let arbiter_obj = create_arbiter(&arbiter, defaults.asset_id, defaults.asset_amount).await;
        let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;
        let arbiter_obj_seller =
            create_arbiter(&seller, defaults.asset_id, defaults.asset_amount).await;

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
        propose_arbiter(arbiter_obj_seller, &seller, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "FeeCannotBeZero")]
    async fn when_arbiter_fee_is_zero() {
        let (arbiter, buyer, seller, defaults) = setup().await;
        let arbiter_obj = create_arbiter(&arbiter, defaults.asset_id, defaults.asset_amount).await;
        let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;
        let arbiter_obj_zero = create_arbiter(&arbiter, defaults.asset_id, 0).await;

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
        propose_arbiter(arbiter_obj_zero, &seller, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "FeeDoesNotMatchAmountSent")]
    async fn when_deposit_for_arbiter_fee_is_unequal() {
        let (arbiter, buyer, seller, defaults) = setup().await;
        let arbiter_obj = create_arbiter(&arbiter, defaults.asset_id, defaults.asset_amount).await;
        let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;
        let tx_params = TxPolicies::new(Some(0), Some(10_000_000), None, None, None);
        let call_params = CallParameters::new(
            arbiter_obj.fee_amount - 1,
            AssetId::from(*arbiter_obj.asset),
            10_000_000,
        );

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

        seller
            .contract
            .methods()
            .propose_arbiter(arbiter_obj, 0)
            .with_tx_policies(tx_params)
            .call_params(call_params)
            .unwrap()
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "AssetDoesNotMatch")]
    async fn when_asset_used_for_arbiter_fee_is_unequal() {
        let (arbiter, buyer, seller, defaults) = setup().await;
        let arbiter_obj = create_arbiter(&arbiter, defaults.asset_id, defaults.asset_amount).await;
        let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

        let arbiter_obj_unequal =
            create_arbiter(&arbiter, defaults.asset_id, defaults.asset_amount).await;
        let tx_params = TxPolicies::new(Some(0), Some(1_000_000), None, None, None);
        let call_params = CallParameters::new(
            arbiter_obj_unequal.fee_amount,
            AssetId::from(*defaults.other_asset_id),
            1_000_000,
        );

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

        seller
            .contract
            .methods()
            .propose_arbiter(arbiter_obj_unequal, 0)
            .with_tx_policies(tx_params)
            .call_params(call_params)
            .unwrap()
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap();
    }
}

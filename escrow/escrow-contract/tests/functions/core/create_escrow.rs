use crate::utils::{
    interface::core::create_escrow,
    setup::{create_arbiter, create_asset, setup},
};

mod success {

    use super::*;
    use crate::utils::{
        interface::info::{assets, escrow_count, escrows},
        setup::{asset_amount, escrow_info, CreatedEscrowEvent},
    };

    #[tokio::test]
    async fn creates_escrow_single_asset() {
        let (arbiter, buyer, seller, defaults) = setup().await;
        let arbiter_obj = create_arbiter(&arbiter, defaults.asset_id, defaults.asset_amount).await;
        let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

        assert_eq!(
            defaults.initial_wallet_amount,
            asset_amount(&defaults.asset_id, &seller).await
        );
        assert_eq!(0, escrow_count(&seller).await);
        assert!(assets(&seller, 0).await.is_none());
        assert!(escrows(&seller, 0).await.is_none());

        let response = create_escrow(
            defaults.asset_amount,
            &arbiter_obj,
            &defaults.asset_id,
            vec![asset.clone()],
            &buyer,
            &seller,
            defaults.deadline,
        )
        .await;

        assert_eq!(
            defaults.initial_wallet_amount - defaults.asset_amount,
            asset_amount(&defaults.asset_id, &seller).await
        );
        assert_eq!(1, escrow_count(&seller).await);
        assert_eq!(assets(&seller, 0).await.unwrap(), asset);
        assert_eq!(
            escrows(&seller, 0).await.unwrap(),
            escrow_info(
                arbiter_obj.clone(),
                1,
                &buyer,
                None,
                0,
                defaults.deadline,
                false,
                0,
                &seller,
                false
            )
            .await
        );

        let log = response
            .decode_logs_with_type::<CreatedEscrowEvent>()
            .unwrap();
        let event = log.first().unwrap();

        assert_eq!(
            *event,
            CreatedEscrowEvent {
                escrow: escrow_info(
                    arbiter_obj.clone(),
                    1,
                    &buyer,
                    None,
                    0,
                    defaults.deadline,
                    false,
                    0,
                    &seller,
                    false
                )
                .await,
                identifier: 0
            }
        );
    }

    #[tokio::test]
    async fn creates_escrow_multiple_assets() {
        let (arbiter, buyer, seller, defaults) = setup().await;
        let arbiter_obj = create_arbiter(&arbiter, defaults.asset_id, defaults.asset_amount).await;
        let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

        assert_eq!(
            defaults.initial_wallet_amount,
            asset_amount(&defaults.asset_id, &seller).await
        );
        assert_eq!(0, escrow_count(&seller).await);
        assert!(assets(&seller, 0).await.is_none());
        assert!(escrows(&seller, 0).await.is_none());

        let response = create_escrow(
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
            defaults.initial_wallet_amount - defaults.asset_amount,
            asset_amount(&defaults.asset_id, &seller).await
        );
        assert_eq!(1, escrow_count(&seller).await);
        assert_eq!(assets(&seller, 0).await.unwrap(), asset);
        assert_eq!(assets(&seller, 1).await.unwrap(), asset);
        assert_eq!(assets(&seller, 2).await.unwrap(), asset);
        assert_eq!(
            escrows(&seller, 0).await.unwrap(),
            escrow_info(
                arbiter_obj.clone(),
                3,
                &buyer,
                None,
                0,
                defaults.deadline,
                false,
                0,
                &seller,
                false
            )
            .await
        );

        let log = response
            .decode_logs_with_type::<CreatedEscrowEvent>()
            .unwrap();
        let event = log.first().unwrap();

        assert_eq!(
            *event,
            CreatedEscrowEvent {
                escrow: escrow_info(
                    arbiter_obj.clone(),
                    3,
                    &buyer,
                    None,
                    0,
                    defaults.deadline,
                    false,
                    0,
                    &seller,
                    false
                )
                .await,
                identifier: 0
            }
        );
    }

    #[tokio::test]
    async fn creates_two_escrow() {
        let (arbiter, buyer, seller, defaults) = setup().await;
        let arbiter_obj = create_arbiter(&arbiter, defaults.asset_id, defaults.asset_amount).await;
        let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

        assert_eq!(
            defaults.initial_wallet_amount,
            asset_amount(&defaults.asset_id, &seller).await
        );
        assert_eq!(0, escrow_count(&seller).await);
        assert!(assets(&seller, 0).await.is_none());
        assert!(escrows(&seller, 0).await.is_none());

        let response1 = create_escrow(
            defaults.asset_amount,
            &arbiter_obj,
            &defaults.asset_id,
            vec![asset.clone()],
            &buyer,
            &seller,
            defaults.deadline,
        )
        .await;

        assert_eq!(
            defaults.initial_wallet_amount - defaults.asset_amount,
            asset_amount(&defaults.asset_id, &seller).await
        );
        assert_eq!(1, escrow_count(&seller).await);
        assert_eq!(assets(&seller, 0).await.unwrap(), asset);
        assert_eq!(
            escrows(&seller, 0).await.unwrap(),
            escrow_info(
                arbiter_obj.clone(),
                1,
                &buyer,
                None,
                0,
                defaults.deadline,
                false,
                0,
                &seller,
                false
            )
            .await
        );

        let response2 = create_escrow(
            defaults.asset_amount,
            &arbiter_obj,
            &defaults.asset_id,
            vec![asset.clone()],
            &buyer,
            &seller,
            defaults.deadline,
        )
        .await;

        assert_eq!(
            defaults.initial_wallet_amount - (2 * (defaults.asset_amount)),
            asset_amount(&defaults.asset_id, &seller).await
        );
        assert_eq!(2, escrow_count(&seller).await);
        assert_eq!(assets(&seller, 1).await.unwrap(), asset);
        assert_eq!(
            escrows(&seller, 1).await.unwrap(),
            escrow_info(
                arbiter_obj.clone(),
                1,
                &buyer,
                None,
                0,
                defaults.deadline,
                false,
                1,
                &seller,
                false
            )
            .await
        );

        let log1 = response1
            .decode_logs_with_type::<CreatedEscrowEvent>()
            .unwrap();
        let log2 = response2
            .decode_logs_with_type::<CreatedEscrowEvent>()
            .unwrap();
        let event1 = log1.first().unwrap();
        let event2 = log2.first().unwrap();

        assert_eq!(
            *event1,
            CreatedEscrowEvent {
                escrow: escrow_info(
                    arbiter_obj.clone(),
                    1,
                    &buyer,
                    None,
                    0,
                    defaults.deadline,
                    false,
                    0,
                    &seller,
                    false
                )
                .await,
                identifier: 0
            }
        );
        assert_eq!(
            *event2,
            CreatedEscrowEvent {
                escrow: escrow_info(
                    arbiter_obj.clone(),
                    1,
                    &buyer,
                    None,
                    0,
                    defaults.deadline,
                    false,
                    1,
                    &seller,
                    false
                )
                .await,
                identifier: 1
            }
        );
    }
}

mod revert {

    use fuels::types::AssetId;

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "UnspecifiedAssets")]
    async fn when_assets_are_not_specified() {
        let (arbiter, buyer, seller, defaults) = setup().await;
        let arbiter_obj = create_arbiter(&arbiter, defaults.asset_id, defaults.asset_amount).await;

        create_escrow(
            defaults.asset_amount,
            &arbiter_obj,
            &defaults.asset_id,
            vec![],
            &buyer,
            &seller,
            defaults.deadline,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "MustBeInTheFuture")]
    async fn when_deadline_is_not_in_the_future() {
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
            0,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "FeeCannotBeZero")]
    async fn when_arbiter_fee_is_zero() {
        let (arbiter, buyer, seller, defaults) = setup().await;
        let arbiter_obj = create_arbiter(&arbiter, defaults.asset_id, 0).await;
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
    }

    #[tokio::test]
    #[should_panic(expected = "FeeDoesNotMatchAmountSent")]
    async fn when_deposit_for_arbiter_fee_is_unequal() {
        let (arbiter, buyer, seller, defaults) = setup().await;
        let arbiter_obj = create_arbiter(&arbiter, defaults.asset_id, defaults.asset_amount).await;
        let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

        create_escrow(
            defaults.asset_amount - 1,
            &arbiter_obj,
            &defaults.asset_id,
            vec![asset.clone(), asset.clone()],
            &buyer,
            &seller,
            defaults.deadline,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "AssetDoesNotMatch")]
    async fn when_asset_used_for_arbiter_fee_is_unequal() {
        let (arbiter, buyer, seller, defaults) = setup().await;
        let arbiter_obj =
            create_arbiter(&arbiter, AssetId::from([2u8; 32]), defaults.asset_amount).await;
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
    }

    #[tokio::test]
    #[should_panic(expected = "CannotBeBuyer")]
    async fn when_arbiter_address_is_set_to_buyer() {
        let (_, buyer, seller, defaults) = setup().await;
        let arbiter_obj = create_arbiter(&buyer, defaults.asset_id, defaults.asset_amount).await;
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
    }

    #[tokio::test]
    #[should_panic(expected = "CannotBeSeller")]
    async fn when_arbiter_address_is_set_to_seller() {
        let (_, buyer, seller, defaults) = setup().await;
        let arbiter_obj = create_arbiter(&seller, defaults.asset_id, defaults.asset_amount).await;
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
    }

    #[tokio::test]
    #[should_panic(expected = "AssetAmountCannotBeZero")]
    async fn when_asset_amount_is_zero() {
        let (arbiter, buyer, seller, defaults) = setup().await;
        let arbiter_obj = create_arbiter(&arbiter, defaults.asset_id, defaults.asset_amount).await;
        let asset = create_asset(0, defaults.asset_id).await;

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
    }
}

use crate::utils::{
    interface::core::{create_escrow, deposit},
    setup::{create_arbiter, create_asset, mint, setup},
};
use fuels::tx::ContractId;

mod success {

    use super::*;
    use crate::utils::{
        interface::info::escrows,
        setup::{asset_amount, escrow_info, DepositEvent},
    };

    #[tokio::test]
    async fn deposits() {
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

        assert_eq!(
            defaults.asset_amount,
            asset_amount(&defaults.asset_id, &buyer.wallet).await
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

        let response = deposit(
            defaults.asset_amount,
            &defaults.asset_id,
            &buyer.contract,
            0,
        )
        .await;

        assert_eq!(0, asset_amount(&defaults.asset_id, &buyer.wallet).await);
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

        let log = response.get_logs_with_type::<DepositEvent>().unwrap();
        let event = log.get(0).unwrap();

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
            vec![asset.clone()],
            buyer.wallet.address(),
            &seller.contract,
            defaults.deadline,
        )
        .await;
        create_escrow(
            defaults.asset_amount,
            &arbiter_obj,
            &defaults.asset_id,
            vec![asset.clone(), asset.clone(), asset.clone()],
            buyer.wallet.address(),
            &seller.contract,
            defaults.deadline,
        )
        .await;

        assert_eq!(
            defaults.asset_amount * 2,
            asset_amount(&defaults.asset_id, &buyer.wallet).await
        );
        assert_eq!(
            escrows(&seller.contract, 0).await.unwrap(),
            escrow_info(
                arbiter_obj.clone(),
                1,
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
                3,
                buyer.wallet.address(),
                None,
                0,
                defaults.deadline,
                false,
                1,
                seller.wallet.address(),
                false
            )
            .await
        );

        let response1 = deposit(
            defaults.asset_amount,
            &defaults.asset_id,
            &buyer.contract,
            0,
        )
        .await;

        let asset_amount1 = asset_amount(&defaults.asset_id, &buyer.wallet).await;
        let escrow_info1 = escrows(&seller.contract, 0).await.unwrap();

        let response2 = deposit(
            defaults.asset_amount,
            &defaults.asset_id,
            &buyer.contract,
            1,
        )
        .await;

        assert_eq!(defaults.asset_amount, asset_amount1);
        assert_eq!(0, asset_amount(&defaults.asset_id, &buyer.wallet).await);
        assert_eq!(
            escrow_info1,
            escrow_info(
                arbiter_obj.clone(),
                1,
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
                3,
                buyer.wallet.address(),
                Some(defaults.asset_id),
                defaults.asset_amount,
                defaults.deadline,
                false,
                1,
                seller.wallet.address(),
                false
            )
            .await
        );

        let log1 = response1.get_logs_with_type::<DepositEvent>().unwrap();
        let log2 = response2.get_logs_with_type::<DepositEvent>().unwrap();

        let event1 = log1.get(0).unwrap();
        let event2 = log2.get(0).unwrap();

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
    use crate::utils::{interface::core::transfer_to_seller, setup::create_asset_with_salt};

    #[tokio::test]
    #[should_panic(expected = "EscrowExpired")]
    async fn when_deadline_is_reached() {
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
            6,
        )
        .await;
        deposit(
            defaults.asset_amount,
            &defaults.asset_id,
            &buyer.contract,
            0,
        )
        .await;
    }

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
        deposit(
            defaults.asset_amount,
            &defaults.asset_id,
            &buyer.contract,
            0,
        )
        .await;
        transfer_to_seller(&buyer.contract, 0).await;
        deposit(
            defaults.asset_amount,
            &defaults.asset_id,
            &buyer.contract,
            0,
        )
        .await;
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
            &seller.contract,
            0,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "AlreadyDeposited")]
    async fn when_depositing_more_than_once() {
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
            0,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "IncorrectAssetAmount")]
    async fn when_incorrect_asset_amount_is_sent() {
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
        deposit(
            defaults.asset_amount - 1,
            &defaults.asset_id,
            &buyer.contract,
            0,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "IncorrectAssetSent")]
    async fn when_incorrect_asset_is_sent() {
        let (arbiter, buyer, seller, defaults) = setup().await;
        let arbiter_obj = create_arbiter(
            arbiter.wallet.address(),
            defaults.asset_id,
            defaults.asset_amount,
        )
        .await;
        let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;
        let (id, salted_asset) = create_asset_with_salt([1u8; 32], buyer.wallet.clone()).await;

        mint(
            seller.wallet.address(),
            defaults.asset_amount,
            &defaults.asset,
        )
        .await;
        mint(
            buyer.wallet.address(),
            defaults.asset_amount * 2,
            &salted_asset,
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
            &ContractId::from(*id),
            &buyer.contract,
            0,
        )
        .await;
    }
}

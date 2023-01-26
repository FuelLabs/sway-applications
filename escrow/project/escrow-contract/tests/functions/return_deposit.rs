use crate::utils::{
    abi_calls::{create_escrow, deposit, propose_arbiter, return_deposit},
    test_helpers::{asset_amount, create_arbiter, create_asset, mint, setup},
};

mod success {

    use super::*;

    #[tokio::test]
    async fn returns_deposit() {
        let (arbiter, buyer, seller, defaults) = setup().await;

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

        let arbiter_obj = create_arbiter(
            arbiter.wallet.address(),
            defaults.asset_id,
            defaults.asset_amount,
        )
        .await;
        let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

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

        return_deposit(&seller.contract, 0).await;
        assert_eq!(
            defaults.asset_amount,
            asset_amount(&defaults.asset_id, &buyer.wallet).await
        );
    }

    #[tokio::test]
    async fn returns_deposit_after_proposing_arbiter() {
        let (arbiter, buyer, seller, defaults) = setup().await;

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

        let arbiter_obj = create_arbiter(
            arbiter.wallet.address(),
            defaults.asset_id,
            defaults.asset_amount,
        )
        .await;
        let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

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
        propose_arbiter(arbiter_obj, &seller.contract, 0).await;
        deposit(
            defaults.asset_amount,
            &defaults.asset_id,
            &buyer.contract,
            0,
        )
        .await;

        assert_eq!(0, asset_amount(&defaults.asset_id, &seller.wallet).await);
        assert_eq!(0, asset_amount(&defaults.asset_id, &buyer.wallet).await);

        return_deposit(&seller.contract, 0).await;
        assert_eq!(
            defaults.asset_amount,
            asset_amount(&defaults.asset_id, &buyer.wallet).await
        );
        assert_eq!(
            defaults.asset_amount * 2,
            asset_amount(&defaults.asset_id, &seller.wallet).await
        );
    }

    #[tokio::test]
    async fn returns_deposit_in_two_escrows() {
        let (arbiter, buyer, seller, defaults) = setup().await;

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

        let arbiter_obj = create_arbiter(
            arbiter.wallet.address(),
            defaults.asset_id,
            defaults.asset_amount,
        )
        .await;
        let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

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

        return_deposit(&seller.contract, 0).await;
        assert_eq!(
            defaults.asset_amount,
            asset_amount(&defaults.asset_id, &buyer.wallet).await
        );

        return_deposit(&seller.contract, 1).await;
        assert_eq!(
            defaults.asset_amount * 2,
            asset_amount(&defaults.asset_id, &buyer.wallet).await
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
        return_deposit(&seller.contract, 0).await;
        return_deposit(&seller.contract, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Unauthorized")]
    async fn when_caller_is_not_seller() {
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
        return_deposit(&buyer.contract, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "CannotTransferBeforeDesposit")]
    async fn when_buyer_has_not_deposited() {
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
        return_deposit(&seller.contract, 0).await;
    }
}

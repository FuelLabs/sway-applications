use crate::utils::{
    abi_calls::create_escrow,
    test_helpers::{asset_amount, create_arbiter, create_asset, mint, setup},
};
use fuels::tx::ContractId;

mod success {

    use super::*;

    #[tokio::test]
    async fn creates_escrow() {
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
        assert_eq!(
            defaults.asset_amount,
            asset_amount(&defaults.asset_id, &seller.wallet).await
        );

        create_escrow(
            defaults.asset_amount,
            &arbiter_obj,
            &defaults.asset_id,
            [asset.clone(), asset.clone()],
            buyer.wallet.address(),
            &seller.contract,
            defaults.deadline,
        )
        .await;
        assert_eq!(0, asset_amount(&defaults.asset_id, &seller.wallet).await);
    }

    #[tokio::test]
    async fn creates_two_escrow() {
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
        assert_eq!(
            defaults.asset_amount * 2,
            asset_amount(&defaults.asset_id, &seller.wallet).await
        );

        create_escrow(
            defaults.asset_amount,
            &arbiter_obj,
            &defaults.asset_id,
            [asset.clone(), asset.clone()],
            buyer.wallet.address(),
            &seller.contract,
            defaults.deadline,
        )
        .await;
        assert_eq!(
            defaults.asset_amount,
            asset_amount(&defaults.asset_id, &seller.wallet).await
        );

        create_escrow(
            defaults.asset_amount,
            &arbiter_obj,
            &defaults.asset_id,
            [asset.clone(), asset.clone()],
            buyer.wallet.address(),
            &seller.contract,
            defaults.deadline,
        )
        .await;
        assert_eq!(0, asset_amount(&defaults.asset_id, &seller.wallet).await);
    }
}

mod revert {

    use super::*;

    // TODO marked as ignore until we use vec in the contract and can properly test this case
    #[tokio::test]
    #[ignore]
    #[should_panic(expected = "Revert(42)")]
    async fn when_assets_are_not_specified() {
        let (arbiter, buyer, seller, defaults) = setup().await;
        let arbiter_obj = create_arbiter(
            arbiter.wallet.address(),
            defaults.asset_id,
            defaults.asset_amount,
        )
        .await;

        mint(
            seller.wallet.address(),
            defaults.asset_amount,
            &defaults.asset,
        )
        .await;

        // TODO: this test likely fails because the param expects an ARRAY of 2 and we provide 0
        //       args. This is likely a panic because of the SDK rather than the test itself
        // create_escrow(
        //     defaults.asset_amount,
        //     &arbiter_obj,
        //     &defaults.asset_id,
        //     vec![],
        //     buyer.wallet.address(),
        //     &seller.contract,
        //     defaults.deadline,
        // )
        // .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_deadline_is_not_in_the_future() {
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

        create_escrow(
            defaults.asset_amount,
            &arbiter_obj,
            &defaults.asset_id,
            [asset.clone(), asset.clone()],
            buyer.wallet.address(),
            &seller.contract,
            0,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_arbiter_fee_is_zero() {
        let (arbiter, buyer, seller, defaults) = setup().await;
        let arbiter_obj = create_arbiter(arbiter.wallet.address(), defaults.asset_id, 0).await;
        let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

        mint(
            seller.wallet.address(),
            defaults.asset_amount,
            &defaults.asset,
        )
        .await;

        create_escrow(
            defaults.asset_amount,
            &arbiter_obj,
            &defaults.asset_id,
            [asset.clone(), asset.clone()],
            buyer.wallet.address(),
            &seller.contract,
            defaults.deadline,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_deposit_for_arbiter_fee_is_unequal() {
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

        create_escrow(
            defaults.asset_amount - 1,
            &arbiter_obj,
            &defaults.asset_id,
            [asset.clone(), asset.clone()],
            buyer.wallet.address(),
            &seller.contract,
            defaults.deadline,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_asset_used_for_arbiter_fee_is_unequal() {
        let (arbiter, buyer, seller, defaults) = setup().await;
        let arbiter_obj = create_arbiter(
            arbiter.wallet.address(),
            ContractId::from([2u8; 32]),
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

        create_escrow(
            defaults.asset_amount,
            &arbiter_obj,
            &defaults.asset_id,
            [asset.clone(), asset.clone()],
            buyer.wallet.address(),
            &seller.contract,
            defaults.deadline,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_arbiter_address_is_set_to_buyer() {
        let (_, buyer, seller, defaults) = setup().await;
        let arbiter_obj = create_arbiter(
            buyer.wallet.address(),
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

        create_escrow(
            defaults.asset_amount,
            &arbiter_obj,
            &defaults.asset_id,
            [asset.clone(), asset.clone()],
            buyer.wallet.address(),
            &seller.contract,
            defaults.deadline,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_arbiter_address_is_set_to_seller() {
        let (_, buyer, seller, defaults) = setup().await;
        let arbiter_obj = create_arbiter(
            seller.wallet.address(),
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

        create_escrow(
            defaults.asset_amount,
            &arbiter_obj,
            &defaults.asset_id,
            [asset.clone(), asset.clone()],
            buyer.wallet.address(),
            &seller.contract,
            defaults.deadline,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_asset_amount_is_zero() {
        let (arbiter, buyer, seller, defaults) = setup().await;
        let arbiter_obj = create_arbiter(
            arbiter.wallet.address(),
            defaults.asset_id,
            defaults.asset_amount,
        )
        .await;
        let asset = create_asset(0, defaults.asset_id).await;

        mint(
            seller.wallet.address(),
            defaults.asset_amount,
            &defaults.asset,
        )
        .await;

        create_escrow(
            defaults.asset_amount,
            &arbiter_obj,
            &defaults.asset_id,
            [asset.clone(), asset.clone()],
            buyer.wallet.address(),
            &seller.contract,
            defaults.deadline,
        )
        .await;
    }
}

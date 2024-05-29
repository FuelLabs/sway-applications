use crate::utils::{
    interface::core::auction::create,
    setup::{defaults, setup},
};
use fuels::types::Identity;

mod success {

    use super::*;
    use crate::utils::{
        interface::info::auction_info,
        setup::{create_auction_copy, State},
    };

    #[tokio::test]
    async fn creates_new_auction() {
        let (deployer, seller, _, _, _, sell_asset, buy_asset) = setup().await;
        let (sell_amount, initial_price, reserve_price, duration, _initial_wallet_amount) =
            defaults().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let provider = deployer.wallet.provider().unwrap();

        let auction = auction_info(0, &seller.auction).await;
        assert!(auction.is_none());

        let auction_id = create(
            buy_asset,
            &seller.auction,
            duration,
            initial_price,
            Some(reserve_price),
            seller_identity,
            sell_asset,
            sell_amount,
        )
        .await;

        let total_duration = provider.latest_block_height().await.unwrap() + duration;
        let auction = auction_info(auction_id, &seller.auction).await;
        assert!(auction.is_some());

        let auction_copy = create_auction_copy(
            buy_asset,
            0,
            None,
            total_duration,
            initial_price,
            Some(reserve_price),
            sell_asset,
            sell_amount,
            seller_identity,
            State::Open,
        )
        .await;
        assert_eq!(auction.unwrap(), auction_copy);
    }

    #[tokio::test]
    async fn creates_multiple_auctions() {
        let (deployer, seller, _, _, _, sell_asset, buy_asset) = setup().await;
        let (sell_amount, initial_price, reserve_price, duration, _initial_wallet_amount) =
            defaults().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let provider = deployer.wallet.provider().unwrap();

        let auction = auction_info(0, &seller.auction).await;
        assert!(auction.is_none());

        let auction_id1 = create(
            buy_asset,
            &seller.auction,
            duration,
            initial_price,
            Some(reserve_price),
            seller_identity,
            sell_asset,
            sell_amount,
        )
        .await;

        let total_duration = provider.latest_block_height().await.unwrap() + duration;
        let auction1 = auction_info(auction_id1, &seller.auction).await;
        assert!(auction1.is_some());

        let auction1_copy = create_auction_copy(
            buy_asset,
            0,
            None,
            total_duration,
            initial_price,
            Some(reserve_price),
            sell_asset,
            sell_amount,
            seller_identity,
            State::Open,
        )
        .await;
        assert_eq!(auction1.unwrap(), auction1_copy);

        let auction_id2 = create(
            buy_asset,
            &seller.auction,
            duration,
            initial_price,
            Some(reserve_price),
            seller_identity,
            sell_asset,
            sell_amount,
        )
        .await;

        let total_duration = provider.latest_block_height().await.unwrap() + duration;
        let auction2 = auction_info(auction_id2, &seller.auction).await;
        assert!(auction2.is_some());

        let auction2_copy = create_auction_copy(
            buy_asset,
            0,
            None,
            total_duration,
            initial_price,
            Some(reserve_price),
            sell_asset,
            sell_amount,
            seller_identity,
            State::Open,
        )
        .await;
        assert_eq!(auction2.unwrap(), auction2_copy);
    }

    #[tokio::test]
    async fn creates_new_auction_without_reserve() {
        let (deployer, seller, _, _, _, sell_asset, buy_asset) = setup().await;
        let (sell_amount, initial_price, _, duration, _) = defaults().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let provider = deployer.wallet.provider().unwrap();

        let auction = auction_info(0, &seller.auction).await;
        assert!(auction.is_none());

        let auction_id = create(
            buy_asset,
            &seller.auction,
            duration,
            initial_price,
            None,
            seller_identity,
            sell_asset,
            sell_amount,
        )
        .await;

        let total_duration = provider.latest_block_height().await.unwrap() + duration;
        let auction = auction_info(auction_id, &seller.auction).await;
        assert!(auction.is_some());

        let auction_copy = create_auction_copy(
            buy_asset,
            0,
            None,
            total_duration,
            initial_price,
            None,
            sell_asset,
            sell_amount,
            seller_identity,
            State::Open,
        )
        .await;
        assert_eq!(auction.unwrap(), auction_copy);
    }

    #[tokio::test]
    async fn creates_new_auction_with_reserve_equal_to_initial_price() {
        let (deployer, seller, _, _, _, sell_asset, buy_asset) = setup().await;
        let (sell_amount, initial_price, _, duration, _) = defaults().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let provider = deployer.wallet.provider().unwrap();

        let auction = auction_info(0, &seller.auction).await;
        assert!(auction.is_none());

        let auction_id = create(
            buy_asset,
            &seller.auction,
            duration,
            initial_price,
            Some(initial_price),
            seller_identity,
            sell_asset,
            sell_amount,
        )
        .await;

        let total_duration = provider.latest_block_height().await.unwrap() + duration;
        let auction = auction_info(auction_id, &seller.auction).await;
        assert!(auction.is_some());

        let auction_copy = create_auction_copy(
            buy_asset,
            0,
            None,
            total_duration,
            initial_price,
            Some(initial_price),
            sell_asset,
            sell_amount,
            seller_identity,
            State::Open,
        )
        .await;
        assert_eq!(auction.unwrap(), auction_copy);
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "ReserveLessThanInitialPrice")]
    async fn when_reserve_is_less_than_initial_price() {
        let (_, seller, _, _, _, sell_asset, buy_asset) = setup().await;
        let (sell_amount, initial_price, _, duration, _) = defaults().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());

        create(
            buy_asset,
            &seller.auction,
            duration,
            initial_price,
            Some(initial_price - 1),
            seller_identity,
            sell_asset,
            sell_amount,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "ReserveLessThanInitialPrice")]
    async fn when_reserve_is_zero() {
        let (_, seller, _, _, _, sell_asset, buy_asset) = setup().await;
        let (sell_amount, initial_price, _, duration, _) = defaults().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());

        create(
            buy_asset,
            &seller.auction,
            duration,
            initial_price,
            Some(0),
            seller_identity,
            sell_asset,
            sell_amount,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "AuctionDurationNotProvided")]
    async fn when_duration_is_zero() {
        let (_, seller, _, _, _, sell_asset, buy_asset) = setup().await;
        let (sell_amount, initial_price, reserve_price, _, _) = defaults().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());

        create(
            buy_asset,
            &seller.auction,
            0,
            initial_price,
            Some(reserve_price),
            seller_identity,
            sell_asset,
            sell_amount,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "InitialPriceCannotBeZero")]
    async fn when_initial_price_is_zero() {
        let (_, seller, _, _, _, sell_asset, buy_asset) = setup().await;
        let (sell_amount, _, reserve_price, duration, _) = defaults().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());

        create(
            buy_asset,
            &seller.auction,
            duration,
            0,
            Some(reserve_price),
            seller_identity,
            sell_asset,
            sell_amount,
        )
        .await;
    }
}

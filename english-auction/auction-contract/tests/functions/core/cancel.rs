use crate::utils::{
    interface::core::auction::{bid, cancel, create},
    setup::{defaults, setup},
};
use fuels::types::Identity;

mod success {

    use super::*;
    use crate::utils::{interface::info::auction_info, setup::State};

    #[tokio::test]
    async fn cancels_auction_after_bid() {
        let (_, seller, buyer1, _, _, sell_asset, buy_asset) = setup().await;
        let (sell_amount, initial_price, reserve_price, duration, _initial_wallet_amount) =
            defaults().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());

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

        bid(auction_id, buy_asset, initial_price, &buyer1.auction).await;

        cancel(auction_id, &seller.auction).await;

        let auction = auction_info(auction_id, &seller.auction).await.unwrap();
        assert_eq!(auction.highest_bidder, None);
        assert_eq!(auction.state, State::Closed);
    }

    #[tokio::test]
    async fn cancels_multiple_auctions() {
        let (_, seller, _, _, _, sell_asset, buy_asset) = setup().await;
        let (sell_amount, initial_price, reserve_price, duration, _initial_wallet_amount) =
            defaults().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());

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

        cancel(auction_id1, &seller.auction).await;

        let auction1 = auction_info(auction_id1, &seller.auction).await.unwrap();
        let auction2 = auction_info(auction_id2, &seller.auction).await.unwrap();
        assert_eq!(auction1.highest_bidder, None);
        assert_eq!(auction1.state, State::Closed);
        assert_eq!(auction2.highest_bidder, None);
        assert_eq!(auction2.state, State::Open);

        cancel(auction_id2, &seller.auction).await;

        let auction1 = auction_info(auction_id1, &seller.auction).await.unwrap();
        let auction2 = auction_info(auction_id2, &seller.auction).await.unwrap();
        assert_eq!(auction1.highest_bidder, None);
        assert_eq!(auction1.state, State::Closed);
        assert_eq!(auction2.highest_bidder, None);
        assert_eq!(auction2.state, State::Closed);
    }

    #[tokio::test]
    async fn cancels_asset_auction() {
        let (_, seller, _, _, _, sell_asset, buy_asset) = setup().await;
        let (sell_amount, initial_price, reserve_price, duration, _initial_wallet_amount) =
            defaults().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());

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

        cancel(auction_id, &seller.auction).await;

        let auction = auction_info(auction_id, &seller.auction).await.unwrap();
        assert_eq!(auction.highest_bidder, None);
        assert_eq!(auction.state, State::Closed);
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "AuctionDoesNotExist")]
    async fn when_auction_does_not_exist() {
        let (_, seller, _, _, _, _, _) = setup().await;

        cancel(0, &seller.auction).await;
    }

    #[tokio::test]
    #[should_panic(expected = "AuctionIsNotOpen")]
    async fn when_auction_bid_period_has_ended() {
        let (deployer, seller, _, _, _, sell_asset, buy_asset) = setup().await;
        let (sell_amount, initial_price, reserve_price, duration, _initial_wallet_amount) =
            defaults().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let provider = deployer.wallet.provider().unwrap();

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

        let _result = provider.produce_blocks(duration + 1, Option::None).await;

        cancel(auction_id, &seller.auction).await;
    }

    #[tokio::test]
    #[should_panic(expected = "AuctionIsNotOpen")]
    async fn when_auction_has_closed() {
        let (_, seller, buyer1, _, _, sell_asset, buy_asset) = setup().await;
        let (sell_amount, initial_price, reserve_price, duration, _initial_wallet_amount) =
            defaults().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());

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

        bid(auction_id, buy_asset, reserve_price, &buyer1.auction).await;

        cancel(auction_id, &seller.auction).await;
    }

    #[tokio::test]
    #[should_panic(expected = "AuctionIsNotOpen")]
    async fn when_auction_already_canceled() {
        let (_, seller, _, _, _, sell_asset, buy_asset) = setup().await;
        let (sell_amount, initial_price, reserve_price, duration, _initial_wallet_amount) =
            defaults().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());

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

        cancel(auction_id, &seller.auction).await;
        cancel(auction_id, &seller.auction).await;
    }

    #[tokio::test]
    #[should_panic(expected = "SenderIsNotSeller")]
    async fn when_sender_is_not_seller() {
        let (_, seller, buyer1, _, _, sell_asset, buy_asset) = setup().await;
        let (sell_amount, initial_price, reserve_price, duration, _initial_wallet_amount) =
            defaults().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());

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

        cancel(auction_id, &buyer1.auction).await;
    }
}

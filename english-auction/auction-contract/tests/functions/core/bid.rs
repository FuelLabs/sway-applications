use crate::utils::{
    interface::core::auction::{bid, create},
    setup::{defaults, setup},
};
use fuels::types::Identity;

mod success {

    use super::*;
    use crate::utils::{
        interface::info::{auction_info, deposit_balance},
        setup::{Auction, State},
    };

    #[tokio::test]
    async fn overrides_bid() {
        let (_, seller, buyer1, buyer2, _, sell_asset, buy_asset) = setup().await;
        let (sell_amount, initial_price, reserve_price, duration, _initial_wallet_amount) =
            defaults().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let buyer1_identity = Identity::Address(buyer1.wallet.address().into());
        let buyer2_identity = Identity::Address(buyer2.wallet.address().into());

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

        let buyer1_deposit = deposit_balance(auction_id, &buyer1.auction, buyer1_identity).await;
        assert!(buyer1_deposit.is_none());

        bid(auction_id, buy_asset, initial_price, &buyer1.auction).await;

        let buyer1_deposit = deposit_balance(auction_id, &buyer1.auction, buyer1_identity)
            .await
            .unwrap();
        let auction: Auction = auction_info(auction_id, &seller.auction).await.unwrap();
        assert_eq!(buyer1_deposit, initial_price);
        assert_eq!(auction.highest_bidder.unwrap(), buyer1_identity);
        assert_eq!(auction.state, State::Open);

        let buyer2_deposit = deposit_balance(auction_id, &buyer2.auction, buyer2_identity).await;
        assert!(buyer2_deposit.is_none());

        bid(auction_id, buy_asset, initial_price + 1, &buyer2.auction).await;

        let buyer2_deposit = deposit_balance(auction_id, &buyer2.auction, buyer2_identity)
            .await
            .unwrap();
        let auction: Auction = auction_info(auction_id, &seller.auction).await.unwrap();
        assert_eq!(buyer2_deposit, initial_price + 1);
        assert_eq!(auction.highest_bidder.unwrap(), buyer2_identity);
        assert_eq!(auction.state, State::Open);
    }

    #[tokio::test]
    async fn overrides_bid_to_reserve() {
        let (_, seller, buyer1, buyer2, _, sell_asset, buy_asset) = setup().await;
        let (sell_amount, initial_price, reserve_price, duration, _initial_wallet_amount) =
            defaults().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let buyer1_identity = Identity::Address(buyer1.wallet.address().into());
        let buyer2_identity = Identity::Address(buyer2.wallet.address().into());

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

        let buyer1_deposit = deposit_balance(auction_id, &buyer1.auction, buyer1_identity).await;
        assert!(buyer1_deposit.is_none());

        bid(auction_id, buy_asset, initial_price, &buyer1.auction).await;

        let buyer1_deposit = deposit_balance(auction_id, &buyer1.auction, buyer1_identity)
            .await
            .unwrap();
        let auction: Auction = auction_info(auction_id, &seller.auction).await.unwrap();
        assert_eq!(buyer1_deposit, initial_price);
        assert_eq!(auction.highest_bidder.unwrap(), buyer1_identity);
        assert_eq!(auction.state, State::Open);

        let buyer2_deposit = deposit_balance(auction_id, &buyer2.auction, buyer2_identity).await;
        assert!(buyer2_deposit.is_none());

        bid(auction_id, buy_asset, reserve_price, &buyer2.auction).await;

        let buyer2_deposit = deposit_balance(auction_id, &buyer2.auction, buyer2_identity)
            .await
            .unwrap();
        let auction: Auction = auction_info(auction_id, &seller.auction).await.unwrap();
        assert_eq!(buyer2_deposit, reserve_price);
        assert_eq!(auction.highest_bidder.unwrap(), buyer2_identity);
        assert_eq!(auction.state, State::Closed);
    }

    #[tokio::test]
    async fn places_bid_at_reserve() {
        let (_, seller, buyer1, _, _, sell_asset, buy_asset) = setup().await;
        let (sell_amount, initial_price, reserve_price, duration, _initial_wallet_amount) =
            defaults().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let buyer1_identity = Identity::Address(buyer1.wallet.address().into());

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

        let buyer1_deposit = deposit_balance(auction_id, &buyer1.auction, buyer1_identity).await;
        assert!(buyer1_deposit.is_none());

        bid(auction_id, buy_asset, reserve_price, &buyer1.auction).await;

        let buyer1_deposit = deposit_balance(auction_id, &buyer1.auction, buyer1_identity)
            .await
            .unwrap();
        let auction: Auction = auction_info(auction_id, &seller.auction).await.unwrap();
        assert_eq!(buyer1_deposit, reserve_price);
        assert_eq!(auction.highest_bidder.unwrap(), buyer1_identity);
        assert_eq!(auction.state, State::Closed);
    }

    #[tokio::test]
    async fn places_multiple_bids() {
        let (_, seller, buyer1, _, _, sell_asset, buy_asset) = setup().await;
        let (sell_amount, initial_price, reserve_price, duration, _initial_wallet_amount) =
            defaults().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let buyer1_identity = Identity::Address(buyer1.wallet.address().into());

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

        let buyer1_deposit = deposit_balance(auction_id, &buyer1.auction, buyer1_identity).await;
        assert!(buyer1_deposit.is_none());

        bid(auction_id, buy_asset, initial_price, &buyer1.auction).await;

        let buyer1_deposit = deposit_balance(auction_id, &buyer1.auction, buyer1_identity)
            .await
            .unwrap();
        let auction: Auction = auction_info(auction_id, &seller.auction).await.unwrap();
        assert_eq!(buyer1_deposit, initial_price);
        assert_eq!(auction.highest_bidder.unwrap(), buyer1_identity);
        assert_eq!(auction.state, State::Open);

        bid(auction_id, buy_asset, 1, &buyer1.auction).await;

        let buyer1_deposit = deposit_balance(auction_id, &buyer1.auction, buyer1_identity)
            .await
            .unwrap();
        let auction: Auction = auction_info(auction_id, &seller.auction).await.unwrap();
        assert_eq!(buyer1_deposit, initial_price + 1);
        assert_eq!(auction.highest_bidder.unwrap(), buyer1_identity);
        assert_eq!(auction.state, State::Open);
    }

    #[tokio::test]
    async fn places_bid() {
        let (_, seller, buyer1, _, _, sell_asset, buy_asset) = setup().await;
        let (sell_amount, initial_price, reserve_price, duration, _initial_wallet_amount) =
            defaults().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let buyer1_identity = Identity::Address(buyer1.wallet.address().into());

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

        let buyer1_deposit = deposit_balance(auction_id, &buyer1.auction, buyer1_identity).await;
        assert!(buyer1_deposit.is_none());

        bid(auction_id, buy_asset, initial_price, &buyer1.auction).await;

        let buyer1_deposit = deposit_balance(auction_id, &buyer1.auction, buyer1_identity)
            .await
            .unwrap();
        let auction: Auction = auction_info(auction_id, &seller.auction).await.unwrap();
        assert_eq!(buyer1_deposit, initial_price);
        assert_eq!(auction.bid_asset, buy_asset);
        assert_eq!(auction.highest_bidder.unwrap(), buyer1_identity);
        assert_eq!(auction.state, State::Open);
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "AuctionDoesNotExist")]
    async fn when_auction_id_does_not_map_to_existing_auction() {
        let (_, _, buyer1, _, _, _, buy_asset) = setup().await;
        let (_, initial_price, _reserve_price, _, _) = defaults().await;

        bid(0, buy_asset, initial_price, &buyer1.auction).await;
    }

    #[tokio::test]
    #[should_panic(expected = "BidderIsSeller")]
    async fn when_sender_is_the_seller() {
        let (_, seller, _buyer1, _, _, sell_asset, buy_asset) = setup().await;
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

        bid(auction_id, buy_asset, initial_price, &seller.auction).await;
    }

    #[tokio::test]
    #[should_panic(expected = "AuctionIsNotOpen")]
    async fn when_auction_has_closed() {
        let (_, seller, buyer1, buyer2, _, sell_asset, buy_asset) = setup().await;
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
        bid(auction_id, buy_asset, reserve_price, &buyer2.auction).await;
    }

    #[tokio::test]
    #[should_panic(expected = "AuctionIsNotOpen")]
    async fn when_bidding_period_has_ended() {
        let (deployer, seller, buyer1, _, _, sell_asset, buy_asset) = setup().await;
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

        bid(auction_id, buy_asset, initial_price, &buyer1.auction).await;
    }

    #[tokio::test]
    #[should_panic(expected = "IncorrectAssetProvided")]
    async fn when_asset_provided_not_accepted() {
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

        bid(auction_id, sell_asset, initial_price, &buyer1.auction).await;
    }

    #[tokio::test]
    #[should_panic(expected = "InitialPriceNotMet")]
    async fn when_bid_is_less_than_initial_price() {
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

        bid(auction_id, buy_asset, initial_price - 1, &buyer1.auction).await;
    }

    #[tokio::test]
    #[should_panic(expected = "IncorrectAmountProvided")]
    async fn when_bid_is_less_than_last_bid() {
        let (_, seller, buyer1, buyer2, _, sell_asset, buy_asset) = setup().await;
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
        bid(auction_id, buy_asset, initial_price + 2, &buyer2.auction).await;
        bid(auction_id, buy_asset, 1, &buyer1.auction).await;
    }

    #[tokio::test]
    #[should_panic(expected = "IncorrectAmountProvided")]
    async fn when_bid_is_greater_than_reserve_price() {
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

        bid(auction_id, buy_asset, reserve_price + 1, &buyer1.auction).await;
    }
}

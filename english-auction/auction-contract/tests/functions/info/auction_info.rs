mod success {
    use crate::utils::{
        interface::{
            core::auction::{bid, create},
            info::auction_info,
        },
        setup::{create_auction_copy, defaults, setup, State},
    };
    use fuels::types::Identity;

    #[tokio::test]
    async fn returns_auction_info() {
        let (deployer, seller, buyer1, _buyer2, _, sell_asset, buy_asset) = setup().await;
        let (sell_amount, initial_price, reserve_price, duration, _initial_wallet_amount) =
            defaults().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let buyer1_identity = Identity::Address(buyer1.wallet.address().into());
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

        bid(auction_id, buy_asset, initial_price, &buyer1.auction).await;

        let auction = auction_info(auction_id, &seller.auction).await.unwrap();
        assert_eq!(auction.highest_bidder.unwrap(), buyer1_identity);
        assert_eq!(auction.bid_asset, buy_asset);
        assert_eq!(auction.highest_bid, initial_price);
    }

    #[tokio::test]
    async fn returns_multiple_auction_info() {
        let (deployer, seller, _buyer1, _buyer2, _, sell_asset, buy_asset) = setup().await;
        let (sell_amount, initial_price, reserve_price, duration, _initial_wallet_amount) =
            defaults().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let provider = deployer.wallet.provider().unwrap();

        let auction1 = auction_info(0, &seller.auction).await;
        let auction2 = auction_info(1, &seller.auction).await;
        assert!(auction1.is_none());
        assert!(auction2.is_none());

        let auction1_id = create(
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

        let total_duration1 = provider.latest_block_height().await.unwrap() + duration;
        let auction1 = auction_info(auction1_id, &seller.auction).await;
        let auction2 = auction_info(1, &seller.auction).await;
        assert!(auction1.is_some());
        assert!(auction2.is_none());

        let auction1_copy = create_auction_copy(
            buy_asset,
            0,
            None,
            total_duration1,
            initial_price,
            Some(reserve_price),
            sell_asset,
            sell_amount,
            seller_identity,
            State::Open,
        )
        .await;
        assert_eq!(auction1.unwrap(), auction1_copy);

        let auction2_id = create(
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

        let total_duration2 = provider.latest_block_height().await.unwrap() + duration;
        let auction1 = auction_info(auction1_id, &seller.auction).await;
        let auction2 = auction_info(auction2_id, &seller.auction).await;
        assert!(auction1.is_some());
        assert!(auction2.is_some());
        assert_eq!(auction1.unwrap(), auction1_copy);

        let auction2_copy = create_auction_copy(
            buy_asset,
            0,
            None,
            total_duration2,
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
    async fn returns_none() {
        let (_, seller, _, _, _, _, _) = setup().await;

        assert!(auction_info(0, &seller.auction).await.is_none());
    }
}

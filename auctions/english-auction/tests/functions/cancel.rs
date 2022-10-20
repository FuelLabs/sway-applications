use crate::utils::{
    asset_abi_calls::mint_and_send_to_address,
    english_auction_abi_calls::{auction_info, bid, cancel, create},
    englishauction_mod::State,
    test_helpers::{defaults_token, setup, token_asset},
};
use fuels::prelude::Identity;

mod success {

    use super::*;

    #[tokio::test]
    async fn cancels_token_auction() {
        let (_, seller, _, _, _, sell_token_contract_id, _, buy_token_contract_id, _) = 
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration) = defaults_token().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = token_asset(sell_token_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_token_contract_id, 0).await;

        mint_and_send_to_address(sell_amount, &seller.asset, seller.wallet.address().into()).await;

        let auction_id = create(
            buy_asset.clone(),
            &seller.auction,
            duration,
            initial_price,
            Some(reserve_price),
            seller_identity.clone(),
            sell_asset.clone(),
        )
        .await;

        cancel(auction_id, &seller.auction).await;

        let auction = auction_info(auction_id, &seller.auction).await.unwrap();

        assert_eq!(auction.highest_bidder, None);
        assert_eq!(auction.state, State::Closed());
    }

    #[tokio::test]
    async fn cancels_auction_after_bid() {
        let (_, seller, buyer1, _, _, sell_token_contract_id, _, buy_token_contract_id, _) = 
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration) = defaults_token().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = token_asset(sell_token_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_token_contract_id, 0).await;
        let bid_asset = token_asset(buy_token_contract_id, initial_price).await;

        mint_and_send_to_address(sell_amount, &seller.asset, seller.wallet.address().into()).await;
        mint_and_send_to_address(reserve_price, &buyer1.asset, buyer1.wallet.address().into()).await;

        let auction_id = create(
            buy_asset.clone(),
            &seller.auction,
            duration,
            initial_price,
            Some(reserve_price),
            seller_identity.clone(),
            sell_asset.clone(),
        )
        .await;

        bid(auction_id, bid_asset.clone(), &buyer1.auction).await;

        cancel(auction_id, &seller.auction).await;

        let auction = auction_info(auction_id, &seller.auction).await.unwrap();

        assert_eq!(auction.highest_bidder, None);
        assert_eq!(auction.state, State::Closed());
    }

    #[tokio::test]
    async fn cancels_multiple_auctions() {
        let (_, seller, _, _, _, sell_token_contract_id, _, buy_token_contract_id, _) = 
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration) = defaults_token().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = token_asset(sell_token_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_token_contract_id, 0).await;

        mint_and_send_to_address(sell_amount * 2, &seller.asset, seller.wallet.address().into()).await;

        let auction_id1 = create(
            buy_asset.clone(),
            &seller.auction,
            duration,
            initial_price,
            Some(reserve_price),
            seller_identity.clone(),
            sell_asset.clone(),
        )
        .await;

        let auction_id2 = create(
            buy_asset.clone(),
            &seller.auction,
            duration,
            initial_price,
            Some(reserve_price),
            seller_identity.clone(),
            sell_asset.clone(),
        )
        .await;

        cancel(auction_id1, &seller.auction).await;

        let auction1 = auction_info(auction_id1, &seller.auction).await.unwrap();
        let auction2 = auction_info(auction_id2, &seller.auction).await.unwrap();

        assert_eq!(auction1.highest_bidder, None);
        assert_eq!(auction1.state, State::Closed());
        assert_eq!(auction2.highest_bidder, None);
        assert_eq!(auction2.state, State::Open());

        cancel(auction_id2, &seller.auction).await;

        let auction1 = auction_info(auction_id1, &seller.auction).await.unwrap();
        let auction2 = auction_info(auction_id2, &seller.auction).await.unwrap();

        assert_eq!(auction1.highest_bidder, None);
        assert_eq!(auction1.state, State::Closed());
        assert_eq!(auction2.highest_bidder, None);
        assert_eq!(auction2.state, State::Closed());
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_auction_does_not_exist() {
        let (_, seller, _, _, _, _, _, _, _) = setup().await;

        cancel(0, &seller.auction).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_auction_bid_period_has_ended() {
        let (deployer, seller, _, _, _, sell_token_contract_id, _, buy_token_contract_id, _) = 
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration) = defaults_token().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = token_asset(sell_token_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_token_contract_id, 0).await;
        let provider = deployer.wallet.get_provider().unwrap();

        mint_and_send_to_address(sell_amount, &seller.asset, seller.wallet.address().into()).await;

        let auction_id = create(
            buy_asset.clone(),
            &seller.auction,
            duration,
            initial_price,
            Some(reserve_price),
            seller_identity.clone(),
            sell_asset.clone(),
        )
        .await;

        provider.produce_blocks(duration + 1).await;

        cancel(auction_id, &seller.auction).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_auction_has_closed() {
        let (_, seller, buyer1, _, _, sell_token_contract_id, _, buy_token_contract_id, _) = 
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration) = defaults_token().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = token_asset(sell_token_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_token_contract_id, 0).await;
        let bid_asset = token_asset(buy_token_contract_id, reserve_price).await;

        mint_and_send_to_address(sell_amount, &seller.asset, seller.wallet.address().into()).await;
        mint_and_send_to_address(reserve_price, &buyer1.asset, buyer1.wallet.address().into()).await;

        let auction_id = create(
            buy_asset.clone(),
            &seller.auction,
            duration,
            initial_price,
            Some(reserve_price),
            seller_identity.clone(),
            sell_asset.clone(),
        )
        .await;

        bid(auction_id, bid_asset.clone(), &buyer1.auction).await;

        cancel(auction_id, &seller.auction).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_auction_already_canceled() {
        let (_, seller, _, _, _, sell_token_contract_id, _, buy_token_contract_id, _) = 
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration) = defaults_token().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = token_asset(sell_token_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_token_contract_id, 0).await;

        mint_and_send_to_address(sell_amount, &seller.asset, seller.wallet.address().into()).await;

        let auction_id = create(
            buy_asset.clone(),
            &seller.auction,
            duration,
            initial_price,
            Some(reserve_price),
            seller_identity.clone(),
            sell_asset.clone(),
        )
        .await;

        cancel(auction_id, &seller.auction).await;
        cancel(auction_id, &seller.auction).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_sender_is_not_seller() {
        let (_, seller, buyer1, _, _, sell_token_contract_id, _, buy_token_contract_id, _) = 
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration) = defaults_token().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = token_asset(sell_token_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_token_contract_id, 0).await;

        mint_and_send_to_address(sell_amount, &seller.asset, seller.wallet.address().into()).await;

        let auction_id = create(
            buy_asset.clone(),
            &seller.auction,
            duration,
            initial_price,
            Some(reserve_price),
            seller_identity.clone(),
            sell_asset.clone(),
        )
        .await;

        cancel(auction_id, &buyer1.auction).await;
    }
}

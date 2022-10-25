use crate::utils::{
    asset_abi_calls::mint_and_send_to_address,
    english_auction_abi_calls::{auction_info, bid, create},
    englishauction_mod::State,
    test_helpers::{create_auction_copy, defaults_token, setup, token_asset},
};
use fuels::prelude::Identity;

mod success {

    use super::*;

    #[tokio::test]
    async fn returns_auction_info() {
        let (deployer, seller, buyer1, _, _, sell_asset_contract_id, _, buy_asset_contract_id, _) =
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration) = defaults_token().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let buyer1_identity = Identity::Address(buyer1.wallet.address().into());
        let sell_asset = token_asset(sell_asset_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_asset_contract_id, 0).await;
        let bid_asset = token_asset(buy_asset_contract_id, initial_price).await;
        let provider = deployer.wallet.get_provider().unwrap();

        mint_and_send_to_address(sell_amount, &seller.asset, seller.wallet.address().into()).await;
        mint_and_send_to_address(reserve_price, &buyer1.asset, buyer1.wallet.address().into())
            .await;

        let auction = auction_info(0, &seller.auction).await;
        assert!(auction.is_none());

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

        let total_duration = provider.latest_block_height().await.unwrap() + duration;
        let auction = auction_info(auction_id, &seller.auction).await;
        assert!(auction.is_some());

        let auction_copy = create_auction_copy(
            buy_asset.clone(),
            None,
            total_duration,
            initial_price,
            Some(reserve_price),
            sell_asset,
            seller_identity,
            State::Open(),
        )
        .await;
        assert_eq!(auction.unwrap(), auction_copy);

        bid(auction_id, bid_asset.clone(), &buyer1.auction).await;

        let auction = auction_info(auction_id, &seller.auction).await.unwrap();
        assert_eq!(auction.highest_bidder.unwrap(), buyer1_identity);
        assert_eq!(auction.bid_asset, bid_asset);
    }

    #[tokio::test]
    async fn returns_multiple_auction_info() {
        let (deployer, seller, buyer1, _, _, sell_asset_contract_id, _, buy_asset_contract_id, _) =
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration) = defaults_token().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = token_asset(sell_asset_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_asset_contract_id, 0).await;
        let provider = deployer.wallet.get_provider().unwrap();

        mint_and_send_to_address(
            sell_amount * 2,
            &seller.asset,
            seller.wallet.address().into(),
        )
        .await;
        mint_and_send_to_address(
            reserve_price * 2,
            &buyer1.asset,
            buyer1.wallet.address().into(),
        )
        .await;

        let auction1 = auction_info(0, &seller.auction).await;
        let auction2 = auction_info(1, &seller.auction).await;
        assert!(auction1.is_none());
        assert!(auction2.is_none());

        let auction1_id = create(
            buy_asset.clone(),
            &seller.auction,
            duration,
            initial_price,
            Some(reserve_price),
            seller_identity.clone(),
            sell_asset.clone(),
        )
        .await;

        let total_duration1 = provider.latest_block_height().await.unwrap() + duration;
        let auction1 = auction_info(auction1_id, &seller.auction).await;
        let auction2 = auction_info(1, &seller.auction).await;
        assert!(auction1.is_some());
        assert!(auction2.is_none());

        let auction1_copy = create_auction_copy(
            buy_asset.clone(),
            None,
            total_duration1,
            initial_price,
            Some(reserve_price),
            sell_asset.clone(),
            seller_identity.clone(),
            State::Open(),
        )
        .await;
        assert_eq!(auction1.unwrap(), auction1_copy);

        let auction2_id = create(
            buy_asset.clone(),
            &seller.auction,
            duration,
            initial_price,
            Some(reserve_price),
            seller_identity.clone(),
            sell_asset.clone(),
        )
        .await;

        let total_duration2 = provider.latest_block_height().await.unwrap() + duration;
        let auction1 = auction_info(auction1_id, &seller.auction).await;
        let auction2 = auction_info(auction2_id, &seller.auction).await;
        assert!(auction1.is_some());
        assert!(auction2.is_some());
        assert_eq!(auction1.unwrap(), auction1_copy);

        let auction2_copy = create_auction_copy(
            buy_asset.clone(),
            None,
            total_duration2,
            initial_price,
            Some(reserve_price),
            sell_asset,
            seller_identity.clone(),
            State::Open(),
        )
        .await;
        assert_eq!(auction2.unwrap(), auction2_copy);
    }

    #[tokio::test]
    async fn returns_none() {
        let (_, seller, _, _, _, _, _, _, _) = setup().await;

        assert!(auction_info(0, &seller.auction).await.is_none());
    }
}

use crate::utils::{
    asset_abi_calls::mint_and_send_to_address,
    english_auction_abi_calls::{auction_info, create},
    englishauction_mod::State,
    test_helpers::{defaults_token, defaults_nft, setup, token_asset},
};
use fuels::prelude::Identity;

mod success {

    use super::*;

    #[tokio::test]
    async fn creates_new_token_auction() {
        let (deployer, seller, buyer1, _, sell_asset_contract_id, buy_asset_contract_id) = setup().await;
        let (sell_amount, initial_price, reserve_price, duration) = defaults_token().await;

        mint_and_send_to_address(sell_amount, &seller.asset.unwrap(), seller.wallet.address().into()).await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = token_asset(sell_asset_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_asset_contract_id, 0).await;
        let provider = deployer.wallet.get_provider().unwrap();

        let auction_id = create(buy_asset.clone(), &seller.auction, duration, initial_price, Some(reserve_price), seller_identity.clone(), sell_asset.clone()).await;
        let total_duration = provider.latest_block_height().await.unwrap() + duration;

        let auction = auction_info(auction_id, &seller.auction).await;

        assert!(auction.is_some());
        let auction = auction.unwrap();

        assert_eq!(auction.bid_asset, buy_asset);
        assert_eq!(auction.highest_bidder, None);
        assert_eq!(auction.end_block, total_duration);
        assert_eq!(auction.initial_price, initial_price);
        assert_eq!(auction.reserve_price.unwrap(), reserve_price);
        assert_eq!(auction.sell_asset, sell_asset);
        assert_eq!(auction.seller, seller_identity);
        assert_eq!(auction.state, State::Open());
    }

    #[tokio::test]
    async fn creates_new_nft_auction() {
        let (deployer, seller, buyer1, _, sell_asset_contract_id, buy_asset_contract_id) = setup().await;
        let (sell_amount, initial_price, reserve_price, duration) = defaults_nft().await;
    }
}

mod revert {

    use super::*;

}

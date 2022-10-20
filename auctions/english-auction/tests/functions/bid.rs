use crate::utils::{
    asset_abi_calls::mint_and_send_to_address,
    english_auction_abi_calls::{auction_info, bid, create, deposit},
    englishauction_mod::{Auction, Asset, State},
    nft_abi_calls::{approve, constructor, mint, set_approval_for_all},
    test_helpers::{defaults_nft, defaults_token, nft_asset, setup, token_asset},
};
use fuels::prelude::{AssetId, CallParameters, Identity, TxParameters};

mod success {

    use super::*;

    #[tokio::test]
    async fn places_nft_bid_on_nft_asset() {
        let (_, seller, buyer1, _, auction_contract_id, _, sell_nft_contract_id, _, buy_nft_contract_id) = 
            setup().await;
        let (sell_count, initial_count, reserve_count, duration, access_control) =
            defaults_nft().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let auction_identity = Identity::ContractId(auction_contract_id.into());
        let buyer1_identity = Identity::Address(buyer1.wallet.address().into());
        let sell_asset = nft_asset(sell_nft_contract_id, 0).await;
        let buy_asset = nft_asset(buy_nft_contract_id, 0).await;
        let bid_asset = nft_asset(buy_nft_contract_id, 0).await;

        constructor(
            access_control,
            &seller.nft,
            seller_identity.clone(),
            sell_count,
        )
        .await;
        mint(sell_count, &seller.nft, seller_identity.clone()).await;
        approve(auction_identity.clone(), &seller.nft, 0).await;
        constructor(
            access_control,
            &buyer1.nft,
            buyer1_identity.clone(),
            reserve_count,
        )
        .await;
        mint(reserve_count, &buyer1.nft, buyer1_identity.clone()).await;
        approve(auction_identity.clone(), &buyer1.nft, 0).await;

        let auction_id = create(
            buy_asset.clone(),
            &seller.auction,
            duration,
            initial_count,
            Some(reserve_count),
            seller_identity.clone(),
            sell_asset.clone(),
        )
        .await;

        let buyer1_deposit = deposit(auction_id, &buyer1.auction, buyer1_identity.clone()).await;
        assert!(buyer1_deposit.is_none());

        bid(auction_id, bid_asset.clone(), &buyer1.auction).await;

        let buyer1_deposit: Asset = deposit(auction_id, &buyer1.auction, buyer1_identity.clone()).await.unwrap();
        let auction: Auction = auction_info(auction_id, &seller.auction).await.unwrap();
        assert_eq!(buyer1_deposit, bid_asset);
        assert_eq!(auction.bid_asset, bid_asset);
        assert_eq!(auction.highest_bidder.unwrap(), buyer1_identity);
        assert_eq!(auction.state, State::Closed());
    }

    #[tokio::test]
    async fn places_nft_bid_on_token_asset() {
        let (_, seller, buyer1, _, auction_contract_id, sell_token_contract_id, _, _, buy_nft_contract_id) = 
            setup().await;
        let (_, initial_count, reserve_count, duration, access_control) =
            defaults_nft().await;
        let (sell_amount, _, _, _) = defaults_token().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let auction_identity = Identity::ContractId(auction_contract_id.into());
        let buyer1_identity = Identity::Address(buyer1.wallet.address().into());
        let sell_asset = token_asset(sell_token_contract_id, sell_amount).await;
        let buy_asset = nft_asset(buy_nft_contract_id, 0).await;
        let bid_asset = nft_asset(buy_nft_contract_id, 0).await;

        mint_and_send_to_address(sell_amount, &seller.asset, seller.wallet.address().into()).await;
        constructor(
            access_control,
            &buyer1.nft,
            buyer1_identity.clone(),
            reserve_count,
        )
        .await;
        mint(reserve_count, &buyer1.nft, buyer1_identity.clone()).await;
        approve(auction_identity.clone(), &buyer1.nft, 0).await;

        let auction_id = create(
            buy_asset.clone(),
            &seller.auction,
            duration,
            initial_count,
            Some(reserve_count),
            seller_identity.clone(),
            sell_asset.clone(),
        )
        .await;

        let buyer1_deposit = deposit(auction_id, &buyer1.auction, buyer1_identity.clone()).await;
        assert!(buyer1_deposit.is_none());

        bid(auction_id, bid_asset.clone(), &buyer1.auction).await;

        let buyer1_deposit: Asset = deposit(auction_id, &buyer1.auction, buyer1_identity.clone()).await.unwrap();
        let auction: Auction = auction_info(auction_id, &seller.auction).await.unwrap();
        assert_eq!(buyer1_deposit, bid_asset);
        assert_eq!(auction.bid_asset, bid_asset);
        assert_eq!(auction.highest_bidder.unwrap(), buyer1_identity);
        assert_eq!(auction.state, State::Closed());
    }

    #[tokio::test]
    async fn places_token_bid_on_nft_asset() {
        let (_, seller, buyer1, _, auction_contract_id, _, sell_nft_contract_id, buy_token_contract_id, _) = 
            setup().await;
        let (sell_count, _, _, duration, access_control) =
            defaults_nft().await;
        let (_, initial_price, reserve_price, _) = defaults_token().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let auction_identity = Identity::ContractId(auction_contract_id.into());
        let buyer1_identity = Identity::Address(buyer1.wallet.address().into());
        let sell_asset = nft_asset(sell_nft_contract_id, 0).await;
        let buy_asset = token_asset(buy_token_contract_id, 0).await;
        let bid_asset = token_asset(buy_token_contract_id, initial_price).await;

        constructor(
            access_control,
            &seller.nft,
            seller_identity.clone(),
            sell_count,
        )
        .await;
        mint(sell_count, &seller.nft, seller_identity.clone()).await;
        approve(auction_identity.clone(), &seller.nft, 0).await;
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
        

        let buyer1_deposit = deposit(auction_id, &buyer1.auction, buyer1_identity.clone()).await;
        assert!(buyer1_deposit.is_none());

        bid(auction_id, bid_asset.clone(), &buyer1.auction).await;

        let buyer1_deposit: Asset = deposit(auction_id, &buyer1.auction, buyer1_identity.clone()).await.unwrap();
        let auction: Auction = auction_info(auction_id, &seller.auction).await.unwrap();
        assert_eq!(buyer1_deposit, bid_asset);
        assert_eq!(auction.bid_asset, bid_asset);
        assert_eq!(auction.highest_bidder.unwrap(), buyer1_identity);
        assert_eq!(auction.state, State::Open());
    }

    #[tokio::test]
    async fn places_token_bid_on_token_asset() {
        let (_, seller, buyer1, _, _, sell_token_contract_id, _, buy_token_contract_id, _) = 
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration) = defaults_token().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let buyer1_identity = Identity::Address(buyer1.wallet.address().into());
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
    
        let buyer1_deposit = deposit(auction_id, &buyer1.auction, buyer1_identity.clone()).await;
        assert!(buyer1_deposit.is_none());

        bid(auction_id, bid_asset.clone(), &buyer1.auction).await;

        let buyer1_deposit: Asset = deposit(auction_id, &buyer1.auction, buyer1_identity.clone()).await.unwrap();
        let auction: Auction = auction_info(auction_id, &seller.auction).await.unwrap();
        assert_eq!(buyer1_deposit, bid_asset);
        assert_eq!(auction.bid_asset, bid_asset);
        assert_eq!(auction.highest_bidder.unwrap(), buyer1_identity);
        assert_eq!(auction.state, State::Open());
    }

    #[tokio::test]
    async fn places_bid_at_reserve() {
        let (_, seller, buyer1, _, _, sell_token_contract_id, _, buy_token_contract_id, _) = 
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration) = defaults_token().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let buyer1_identity = Identity::Address(buyer1.wallet.address().into());
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

        let buyer1_deposit = deposit(auction_id, &buyer1.auction, buyer1_identity.clone()).await;
        assert!(buyer1_deposit.is_none());

        bid(auction_id, bid_asset.clone(), &buyer1.auction).await;

        let buyer1_deposit: Asset = deposit(auction_id, &buyer1.auction, buyer1_identity.clone()).await.unwrap();
        let auction: Auction = auction_info(auction_id, &seller.auction).await.unwrap();
        assert_eq!(buyer1_deposit, bid_asset);
        assert_eq!(auction.bid_asset, bid_asset);
        assert_eq!(auction.highest_bidder.unwrap(), buyer1_identity);
        assert_eq!(auction.state, State::Closed());
    }

    #[tokio::test]
    async fn overrides_bid() {
        let (_, seller, buyer1, buyer2, _, sell_token_contract_id, _, buy_token_contract_id, _) = 
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration) = defaults_token().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let buyer1_identity = Identity::Address(buyer1.wallet.address().into());
        let buyer2_identity = Identity::Address(buyer2.wallet.address().into());
        let sell_asset = token_asset(sell_token_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_token_contract_id, 0).await;
        let bid_asset = token_asset(buy_token_contract_id, initial_price).await;
        let bid2_asset = token_asset(buy_token_contract_id, initial_price + 1).await;
        assert!(initial_price + 1 < reserve_price);

        mint_and_send_to_address(sell_amount, &seller.asset, seller.wallet.address().into()).await;
        mint_and_send_to_address(reserve_price, &buyer1.asset, buyer1.wallet.address().into()).await;
        mint_and_send_to_address(reserve_price, &buyer1.asset, buyer2.wallet.address().into()).await;

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
        
        let buyer1_deposit = deposit(auction_id, &buyer1.auction, buyer1_identity.clone()).await;
        assert!(buyer1_deposit.is_none());

        bid(auction_id, bid_asset.clone(), &buyer1.auction).await;

        let buyer1_deposit: Asset = deposit(auction_id, &buyer1.auction, buyer1_identity.clone()).await.unwrap();
        let auction: Auction = auction_info(auction_id, &seller.auction).await.unwrap();
        assert_eq!(buyer1_deposit, bid_asset);
        assert_eq!(auction.bid_asset, bid_asset);
        assert_eq!(auction.highest_bidder.unwrap(), buyer1_identity);
        assert_eq!(auction.state, State::Open());

        let buyer2_deposit = deposit(auction_id, &buyer2.auction, buyer2_identity.clone()).await;
        assert!(buyer2_deposit.is_none());

        bid(auction_id, bid2_asset.clone(), &buyer2.auction).await;

        let buyer2_deposit: Asset = deposit(auction_id, &buyer2.auction, buyer2_identity.clone()).await.unwrap();
        let auction: Auction = auction_info(auction_id, &seller.auction).await.unwrap();
        assert_eq!(buyer2_deposit, bid2_asset);
        assert_eq!(auction.bid_asset, bid2_asset);
        assert_eq!(auction.highest_bidder.unwrap(), buyer2_identity);
        assert_eq!(auction.state, State::Open());
    }

    #[tokio::test]
    async fn overrides_bid_to_reserve() {
        let (_, seller, buyer1, buyer2, _, sell_token_contract_id, _, buy_token_contract_id, _) = 
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration) = defaults_token().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let buyer1_identity = Identity::Address(buyer1.wallet.address().into());
        let buyer2_identity = Identity::Address(buyer2.wallet.address().into());
        let sell_asset = token_asset(sell_token_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_token_contract_id, 0).await;
        let bid_asset = token_asset(buy_token_contract_id, initial_price).await;
        let bid2_asset = token_asset(buy_token_contract_id, reserve_price).await;

        mint_and_send_to_address(sell_amount, &seller.asset, seller.wallet.address().into()).await;
        mint_and_send_to_address(reserve_price, &buyer1.asset, buyer1.wallet.address().into()).await;
        mint_and_send_to_address(reserve_price, &buyer1.asset, buyer2.wallet.address().into()).await;

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

        let buyer1_deposit = deposit(auction_id, &buyer1.auction, buyer1_identity.clone()).await;
        assert!(buyer1_deposit.is_none());

        bid(auction_id, bid_asset.clone(), &buyer1.auction).await;

        let buyer1_deposit: Asset = deposit(auction_id, &buyer1.auction, buyer1_identity.clone()).await.unwrap();
        let auction: Auction = auction_info(auction_id, &seller.auction).await.unwrap();
        assert_eq!(buyer1_deposit, bid_asset);
        assert_eq!(auction.bid_asset, bid_asset);
        assert_eq!(auction.highest_bidder.unwrap(), buyer1_identity);
        assert_eq!(auction.state, State::Open());

        let buyer2_deposit = deposit(auction_id, &buyer2.auction, buyer2_identity.clone()).await;
        assert!(buyer2_deposit.is_none());

        bid(auction_id, bid2_asset.clone(), &buyer2.auction).await;

        let buyer2_deposit: Asset = deposit(auction_id, &buyer2.auction, buyer2_identity.clone()).await.unwrap();
        let auction: Auction = auction_info(auction_id, &seller.auction).await.unwrap();
        assert_eq!(buyer2_deposit, bid2_asset);
        assert_eq!(auction.bid_asset, bid2_asset);
        assert_eq!(auction.highest_bidder.unwrap(), buyer2_identity);
        assert_eq!(auction.state, State::Closed());
    }

    #[tokio::test]
    async fn places_multiple_bids() {
        let (_, seller, buyer1, _, _, sell_token_contract_id, _, buy_token_contract_id, _) = 
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration) = defaults_token().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let buyer1_identity = Identity::Address(buyer1.wallet.address().into());
        let sell_asset = token_asset(sell_token_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_token_contract_id, 0).await;
        let bid1_asset = token_asset(buy_token_contract_id, initial_price).await;
        let bid2_asset = token_asset(buy_token_contract_id, 1).await;
        let total_bid_asset = token_asset(buy_token_contract_id, initial_price + 1).await;

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
    
        let buyer1_deposit = deposit(auction_id, &buyer1.auction, buyer1_identity.clone()).await;
        assert!(buyer1_deposit.is_none());

        bid(auction_id, bid1_asset.clone(), &buyer1.auction).await;

        let buyer1_deposit: Asset = deposit(auction_id, &buyer1.auction, buyer1_identity.clone()).await.unwrap();
        let auction: Auction = auction_info(auction_id, &seller.auction).await.unwrap();
        assert_eq!(buyer1_deposit, bid1_asset);
        assert_eq!(auction.bid_asset, bid1_asset);
        assert_eq!(auction.highest_bidder.unwrap(), buyer1_identity);
        assert_eq!(auction.state, State::Open());

        bid(auction_id, bid2_asset.clone(), &buyer1.auction).await;

        let buyer1_deposit: Asset = deposit(auction_id, &buyer1.auction, buyer1_identity.clone()).await.unwrap();
        let auction: Auction = auction_info(auction_id, &seller.auction).await.unwrap();
        assert_eq!(buyer1_deposit, total_bid_asset);
        assert_eq!(auction.bid_asset, total_bid_asset);
        assert_eq!(auction.highest_bidder.unwrap(), buyer1_identity);
        assert_eq!(auction.state, State::Open());
    }
}

mod revert {
    
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_auction_id_does_not_map_to_existing_auction() {
        let (_, _, buyer1, _, _, _, _, buy_token_contract_id, _) = 
            setup().await;
        let (_, initial_price, reserve_price, _) = defaults_token().await;
        let bid_asset = token_asset(buy_token_contract_id, initial_price).await;

        mint_and_send_to_address(reserve_price, &buyer1.asset, buyer1.wallet.address().into()).await;

        bid(0, bid_asset.clone(), &buyer1.auction).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_auction_has_closed() {
        let (_, seller, buyer1, buyer2, _, sell_token_contract_id, _, buy_token_contract_id, _) = 
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration) = defaults_token().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = token_asset(sell_token_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_token_contract_id, 0).await;
        let bid_asset = token_asset(buy_token_contract_id, reserve_price).await;
        let bid2_asset = token_asset(buy_token_contract_id, reserve_price).await;

        mint_and_send_to_address(sell_amount, &seller.asset, seller.wallet.address().into()).await;
        mint_and_send_to_address(reserve_price, &buyer1.asset, buyer1.wallet.address().into()).await;
        mint_and_send_to_address(reserve_price, &buyer1.asset, buyer2.wallet.address().into()).await;

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
        bid(auction_id, bid2_asset.clone(), &buyer2.auction).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_bidding_period_has_ended() {
        let (deployer, seller, buyer1, _, _, sell_token_contract_id, _, buy_token_contract_id, _) = 
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration) = defaults_token().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = token_asset(sell_token_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_token_contract_id, 0).await;
        let bid_asset = token_asset(buy_token_contract_id, initial_price).await;
        let provider = deployer.wallet.get_provider().unwrap();

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
        
        let _result = provider.produce_blocks(duration + 1).await;

        bid(auction_id, bid_asset.clone(), &buyer1.auction).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_asset_provided_not_accepted() {
        let (_, seller, buyer1, _, _, sell_token_contract_id, _, buy_token_contract_id, _) = 
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration) = defaults_token().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = token_asset(sell_token_contract_id, sell_amount).await;
        let buy_asset = token_asset(sell_token_contract_id, 0).await;
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
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_asset_type_and_struct_mismatch() {
        let (_, seller, buyer1, _, _, sell_token_contract_id, _, buy_token_contract_id, _) = 
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration) = defaults_token().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = token_asset(sell_token_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_token_contract_id, 0).await;
        let bid_asset = token_asset(buy_token_contract_id, initial_price).await;

        mint_and_send_to_address(sell_amount, &seller.asset, seller.wallet.address().into()).await;
        mint_and_send_to_address(reserve_price, &seller.asset, buyer1.wallet.address().into()).await;

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
        
        let tx_params = TxParameters::new(None, Some(1_000_000), None);
        let call_params = CallParameters::new(
            Some(initial_price),
            Some(AssetId::from(*sell_token_contract_id)),
            None,
        );

        buyer1
            .auction
            .methods()
            .bid(auction_id, bid_asset.clone())
            .tx_params(tx_params)
            .call_params(call_params)
            .call()
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_bidder_does_not_own_nft() {
        let (_, seller, buyer1, _, _, sell_token_contract_id, _, _, buy_nft_contract_id) = 
            setup().await;
        let (_, initial_count, reserve_count, duration, _) =
            defaults_nft().await;
        let (sell_amount, _, _, _) = defaults_token().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = token_asset(sell_token_contract_id, sell_amount).await;
        let buy_asset = nft_asset(buy_nft_contract_id, 0).await;
        let bid_asset = nft_asset(buy_nft_contract_id, 0).await;

        mint_and_send_to_address(sell_amount, &seller.asset, seller.wallet.address().into()).await;

        let auction_id = create(
            buy_asset.clone(),
            &seller.auction,
            duration,
            initial_count,
            Some(reserve_count),
            seller_identity.clone(),
            sell_asset.clone(),
        )
        .await;

        bid(auction_id, bid_asset.clone(), &buyer1.auction).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_auction_contract_does_not_have_permission_to_transfer_nft() {
        let (_, seller, buyer1, _, _, sell_token_contract_id, _, _, buy_nft_contract_id) = 
            setup().await;
        let (_, initial_count, reserve_count, duration, access_control) =
            defaults_nft().await;
        let (sell_amount, _, _, _) = defaults_token().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let buyer1_identity = Identity::Address(buyer1.wallet.address().into());
        let sell_asset = token_asset(sell_token_contract_id, sell_amount).await;
        let buy_asset = nft_asset(buy_nft_contract_id, 0).await;
        let bid_asset = nft_asset(buy_nft_contract_id, 0).await;

        mint_and_send_to_address(sell_amount, &seller.asset, seller.wallet.address().into()).await;
        constructor(
            access_control,
            &buyer1.nft,
            buyer1_identity.clone(),
            reserve_count,
        )
        .await;
        mint(reserve_count, &buyer1.nft, buyer1_identity.clone()).await;

        let auction_id = create(
            buy_asset.clone(),
            &seller.auction,
            duration,
            initial_count,
            Some(reserve_count),
            seller_identity.clone(),
            sell_asset.clone(),
        )
        .await;

        bid(auction_id, bid_asset.clone(), &buyer1.auction).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_asset_amount_and_struct_mismatch() {
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
        
        let tx_params = TxParameters::new(None, Some(1_000_000), None);
        let call_params = CallParameters::new(
            Some(initial_price + 1),
            Some(AssetId::from(*buy_token_contract_id)),
            None,
        );

        buyer1
            .auction
            .methods()
            .bid(auction_id, bid_asset.clone())
            .tx_params(tx_params)
            .call_params(call_params)
            .call()
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_sender_is_the_seller() {
        let (_, seller, buyer1, _, _, sell_token_contract_id, _, buy_token_contract_id, _) = 
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration) = defaults_token().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = token_asset(sell_token_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_token_contract_id, 0).await;
        let bid_asset = token_asset(buy_token_contract_id, initial_price).await;

        mint_and_send_to_address(sell_amount, &seller.asset, seller.wallet.address().into()).await;
        mint_and_send_to_address(reserve_price, &buyer1.asset, seller.wallet.address().into()).await;

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
        
        bid(auction_id, bid_asset.clone(), &seller.auction).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_bid_is_less_than_initial_price() {
        let (_, seller, buyer1, _, _, sell_token_contract_id, _, buy_token_contract_id, _) = 
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration) = defaults_token().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = token_asset(sell_token_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_token_contract_id, 0).await;
        let bid_asset = token_asset(buy_token_contract_id, initial_price - 1).await;

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
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_bid_is_less_than_last_bid() {
        let (_, seller, buyer1, buyer2, _, sell_token_contract_id, _, buy_token_contract_id, _) = 
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration) = defaults_token().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = token_asset(sell_token_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_token_contract_id, 0).await;
        let bid_asset = token_asset(buy_token_contract_id, initial_price).await;
        let bid2_asset = token_asset(buy_token_contract_id, initial_price + 2).await;
        let bid3_asset = token_asset(buy_token_contract_id, 1).await;

        mint_and_send_to_address(sell_amount, &seller.asset, seller.wallet.address().into()).await;
        mint_and_send_to_address(reserve_price, &buyer1.asset, buyer1.wallet.address().into()).await;
        mint_and_send_to_address(reserve_price, &buyer1.asset, buyer2.wallet.address().into()).await;

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
        bid(auction_id, bid2_asset.clone(), &buyer2.auction).await;
        bid(auction_id, bid3_asset.clone(), &buyer1.auction).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_bid_is_greater_than_reserve_price() {
        let (_, seller, buyer1, _, _, sell_token_contract_id, _, buy_token_contract_id, _) = 
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration) = defaults_token().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = token_asset(sell_token_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_token_contract_id, 0).await;
        let bid_asset = token_asset(buy_token_contract_id, reserve_price + 1).await;

        mint_and_send_to_address(sell_amount, &seller.asset, seller.wallet.address().into()).await;
        mint_and_send_to_address(reserve_price + 1, &buyer1.asset, buyer1.wallet.address().into()).await;

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
    }
}

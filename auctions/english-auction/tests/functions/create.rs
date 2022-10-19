use crate::utils::{
    asset_abi_calls::mint_and_send_to_address,
    english_auction_abi_calls::{auction_info, create},
    englishauction_mod::{Asset, State},
    nft_abi_calls::{approve, constructor, mint},
    test_helpers::{defaults_nft, defaults_token, nft_asset, setup, token_asset},
};
use fuels::prelude::{AssetId, CallParameters, Identity, TxParameters};

mod success {

    use super::*;

    #[tokio::test]
    async fn creates_new_nft_auction() {
        let (
            deployer,
            seller,
            _,
            _,
            auction_contract_id,
            _,
            sell_nft_contract_id,
            _,
            buy_nft_contract_id,
        ) = setup().await;
        let (sell_count, initial_count, reserve_count, duration, access_control) =
            defaults_nft().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let auction_identity = Identity::ContractId(auction_contract_id.into());

        constructor(
            access_control,
            &seller.nft,
            seller_identity.clone(),
            sell_count,
        )
        .await;
        mint(sell_count, &seller.nft, seller_identity.clone()).await;
        approve(auction_identity.clone(), &seller.nft, 0).await;

        let sell_asset = nft_asset(sell_nft_contract_id, 0).await;
        let buy_asset = nft_asset(buy_nft_contract_id, 0).await;
        let provider = deployer.wallet.get_provider().unwrap();
        let auction = auction_info(0, &seller.auction).await;
        assert!(auction.is_none());

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

        let total_duration = provider.latest_block_height().await.unwrap() + duration;
        let auction = auction_info(auction_id, &seller.auction).await;
        assert!(auction.is_some());
        let auction = auction.unwrap();

        assert_eq!(auction.bid_asset, buy_asset);
        assert_eq!(auction.highest_bidder, None);
        assert_eq!(auction.end_block, total_duration);
        assert_eq!(auction.initial_price, initial_count);
        assert_eq!(auction.reserve_price.unwrap(), reserve_count);
        assert_eq!(auction.sell_asset, sell_asset);
        assert_eq!(auction.seller, seller_identity);
        assert_eq!(auction.state, State::Open());
    }

    #[tokio::test]
    async fn creates_new_nft_auction_with_token_bid_asset() {
        let (
            deployer,
            seller,
            _,
            _,
            auction_contract_id,
            _,
            sell_nft_contract_id,
            buy_asset_contract_id,
            _,
        ) = setup().await;
        let (sell_count, initial_count, reserve_count, duration, access_control) =
            defaults_nft().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let auction_identity = Identity::ContractId(auction_contract_id.into());

        constructor(
            access_control,
            &seller.nft,
            seller_identity.clone(),
            sell_count,
        )
        .await;
        mint(sell_count, &seller.nft, seller_identity.clone()).await;
        approve(auction_identity.clone(), &seller.nft, 0).await;

        let sell_asset = nft_asset(sell_nft_contract_id, 0).await;
        let buy_asset = token_asset(buy_asset_contract_id, 0).await;
        let provider = deployer.wallet.get_provider().unwrap();
        let auction = auction_info(0, &seller.auction).await;
        assert!(auction.is_none());

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

        let total_duration = provider.latest_block_height().await.unwrap() + duration;
        let auction = auction_info(auction_id, &seller.auction).await;
        assert!(auction.is_some());
        let auction = auction.unwrap();

        assert_eq!(auction.bid_asset, buy_asset);
        assert_eq!(auction.highest_bidder, None);
        assert_eq!(auction.end_block, total_duration);
        assert_eq!(auction.initial_price, initial_count);
        assert_eq!(auction.reserve_price.unwrap(), reserve_count);
        assert_eq!(auction.sell_asset, sell_asset);
        assert_eq!(auction.seller, seller_identity);
        assert_eq!(auction.state, State::Open());
    }

    #[tokio::test]
    async fn creates_new_token_auction() {
        let (deployer, seller, _, _, _, sell_asset_contract_id, _, buy_asset_contract_id, _) =
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration) = defaults_token().await;

        mint_and_send_to_address(sell_amount, &seller.asset, seller.wallet.address().into()).await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = token_asset(sell_asset_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_asset_contract_id, 0).await;
        let provider = deployer.wallet.get_provider().unwrap();
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
    async fn creates_new_token_auction_with_nft_bid_asset() {
        let (deployer, seller, _, _, _, sell_asset_contract_id, _, _, buy_nft_contract_id) =
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration) = defaults_token().await;

        mint_and_send_to_address(sell_amount, &seller.asset, seller.wallet.address().into()).await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = token_asset(sell_asset_contract_id, sell_amount).await;
        let buy_asset = nft_asset(buy_nft_contract_id, 0).await;
        let provider = deployer.wallet.get_provider().unwrap();
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
    async fn creates_new_token_auction_without_reserve() {
        let (deployer, seller, _, _, _, sell_asset_contract_id, _, buy_asset_contract_id, _) =
            setup().await;
        let (sell_amount, initial_price, _, duration) = defaults_token().await;

        mint_and_send_to_address(sell_amount, &seller.asset, seller.wallet.address().into()).await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = token_asset(sell_asset_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_asset_contract_id, 0).await;
        let provider = deployer.wallet.get_provider().unwrap();
        let auction = auction_info(0, &seller.auction).await;
        assert!(auction.is_none());

        let auction_id = create(
            buy_asset.clone(),
            &seller.auction,
            duration,
            initial_price,
            None,
            seller_identity.clone(),
            sell_asset.clone(),
        )
        .await;

        let total_duration = provider.latest_block_height().await.unwrap() + duration;
        let auction = auction_info(auction_id, &seller.auction).await;
        assert!(auction.is_some());
        let auction = auction.unwrap();

        assert_eq!(auction.bid_asset, buy_asset);
        assert_eq!(auction.highest_bidder, None);
        assert_eq!(auction.end_block, total_duration);
        assert_eq!(auction.initial_price, initial_price);
        assert_eq!(auction.reserve_price, None);
        assert_eq!(auction.sell_asset, sell_asset);
        assert_eq!(auction.seller, seller_identity);
        assert_eq!(auction.state, State::Open());
    }

    #[tokio::test]
    async fn creates_new_token_auction_with_reserve_equal_to_initial_price() {
        let (deployer, seller, _, _, _, sell_asset_contract_id, _, buy_asset_contract_id, _) =
            setup().await;
        let (sell_amount, initial_price, _, duration) = defaults_token().await;

        mint_and_send_to_address(sell_amount, &seller.asset, seller.wallet.address().into()).await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = token_asset(sell_asset_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_asset_contract_id, 0).await;
        let provider = deployer.wallet.get_provider().unwrap();
        let auction = auction_info(0, &seller.auction).await;
        assert!(auction.is_none());

        let auction_id = create(
            buy_asset.clone(),
            &seller.auction,
            duration,
            initial_price,
            Some(initial_price),
            seller_identity.clone(),
            sell_asset.clone(),
        )
        .await;

        let total_duration = provider.latest_block_height().await.unwrap() + duration;
        let auction = auction_info(auction_id, &seller.auction).await;
        assert!(auction.is_some());
        let auction = auction.unwrap();

        assert_eq!(auction.bid_asset, buy_asset);
        assert_eq!(auction.highest_bidder, None);
        assert_eq!(auction.end_block, total_duration);
        assert_eq!(auction.initial_price, initial_price);
        assert_eq!(auction.reserve_price.unwrap(), initial_price);
        assert_eq!(auction.sell_asset, sell_asset);
        assert_eq!(auction.seller, seller_identity);
        assert_eq!(auction.state, State::Open());
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_reserve_is_less_than_initial_price() {
        let (_, seller, _, _, _, sell_asset_contract_id, _, buy_asset_contract_id, _) =
            setup().await;
        let (sell_amount, initial_price, _, duration) = defaults_token().await;

        mint_and_send_to_address(sell_amount, &seller.asset, seller.wallet.address().into()).await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = token_asset(sell_asset_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_asset_contract_id, 0).await;

        create(
            buy_asset.clone(),
            &seller.auction,
            duration,
            initial_price,
            Some(initial_price - 1),
            seller_identity.clone(),
            sell_asset.clone(),
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_reserve_is_zero() {
        let (_, seller, _, _, _, sell_asset_contract_id, _, buy_asset_contract_id, _) =
            setup().await;
        let (sell_amount, initial_price, _, duration) = defaults_token().await;

        mint_and_send_to_address(sell_amount, &seller.asset, seller.wallet.address().into()).await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = token_asset(sell_asset_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_asset_contract_id, 0).await;

        create(
            buy_asset.clone(),
            &seller.auction,
            duration,
            initial_price,
            Some(0),
            seller_identity.clone(),
            sell_asset.clone(),
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_duration_is_zero() {
        let (_, seller, _, _, _, sell_asset_contract_id, _, buy_asset_contract_id, _) =
            setup().await;
        let (sell_amount, initial_price, reserve_price, _) = defaults_token().await;

        mint_and_send_to_address(sell_amount, &seller.asset, seller.wallet.address().into()).await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = token_asset(sell_asset_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_asset_contract_id, 0).await;

        create(
            buy_asset.clone(),
            &seller.auction,
            0,
            initial_price,
            Some(reserve_price),
            seller_identity.clone(),
            sell_asset.clone(),
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_bid_asset_amount_not_zero() {
        let (_, seller, _, _, _, sell_asset_contract_id, _, buy_asset_contract_id, _) =
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration) = defaults_token().await;

        mint_and_send_to_address(sell_amount, &seller.asset, seller.wallet.address().into()).await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = token_asset(sell_asset_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_asset_contract_id, 1).await;

        create(
            buy_asset.clone(),
            &seller.auction,
            duration,
            initial_price,
            Some(reserve_price),
            seller_identity.clone(),
            sell_asset.clone(),
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_initial_token_price_is_zero() {
        let (_, seller, _, _, _, sell_asset_contract_id, _, buy_asset_contract_id, _) =
            setup().await;
        let (sell_amount, _, reserve_price, duration) = defaults_token().await;

        mint_and_send_to_address(sell_amount, &seller.asset, seller.wallet.address().into()).await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = token_asset(sell_asset_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_asset_contract_id, 0).await;

        create(
            buy_asset.clone(),
            &seller.auction,
            duration,
            0,
            Some(reserve_price),
            seller_identity.clone(),
            sell_asset.clone(),
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_token_asset_sent_less_than_sell_struct() {
        let (_, seller, _, _, _, sell_asset_contract_id, _, buy_asset_contract_id, _) =
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration) = defaults_token().await;

        mint_and_send_to_address(sell_amount, &seller.asset, seller.wallet.address().into()).await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = token_asset(sell_asset_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_asset_contract_id, 0).await;

        match sell_asset {
            Asset::TokenAsset(sell_asset) => {
                let tx_params = TxParameters::new(None, Some(1_000_000), None);
                let call_params = CallParameters::new(
                    Some(sell_asset.amount - 1),
                    Some(AssetId::from(*sell_asset.contract_id)),
                    None,
                );

                seller
                    .auction
                    .methods()
                    .create(
                        buy_asset,
                        duration,
                        initial_price,
                        Some(reserve_price),
                        seller_identity,
                        Asset::TokenAsset(sell_asset.clone()),
                    )
                    .tx_params(tx_params)
                    .call_params(call_params)
                    .call()
                    .await
                    .unwrap();
            }
            _ => {}
        }
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_token_asset_sent_greater_than_sell_struct() {
        let (_, seller, _, _, _, sell_asset_contract_id, _, buy_asset_contract_id, _) =
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration) = defaults_token().await;

        mint_and_send_to_address(
            sell_amount + 1,
            &seller.asset,
            seller.wallet.address().into(),
        )
        .await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = token_asset(sell_asset_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_asset_contract_id, 0).await;

        match sell_asset {
            Asset::TokenAsset(sell_asset) => {
                let tx_params = TxParameters::new(None, Some(1_000_000), None);
                let call_params = CallParameters::new(
                    Some(sell_asset.amount + 1),
                    Some(AssetId::from(*sell_asset.contract_id)),
                    None,
                );

                seller
                    .auction
                    .methods()
                    .create(
                        buy_asset,
                        duration,
                        initial_price,
                        Some(reserve_price),
                        seller_identity,
                        Asset::TokenAsset(sell_asset.clone()),
                    )
                    .tx_params(tx_params)
                    .call_params(call_params)
                    .call()
                    .await
                    .unwrap();
            }
            _ => {}
        }
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_token_asset_sent_not_correct_type() {
        let (_, seller, buyer1, _, _, sell_asset_contract_id, _, buy_asset_contract_id, _) =
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration) = defaults_token().await;

        mint_and_send_to_address(
            sell_amount + 1,
            &seller.asset,
            seller.wallet.address().into(),
        )
        .await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = token_asset(sell_asset_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_asset_contract_id, 0).await;

        match sell_asset {
            Asset::TokenAsset(sell_asset) => {
                let tx_params = TxParameters::new(None, Some(1_000_000), None);
                let call_params = CallParameters::new(
                    Some(sell_asset.amount + 1),
                    Some(AssetId::from(*sell_asset.contract_id)),
                    None,
                );

                seller
                    .auction
                    .methods()
                    .create(
                        buy_asset.clone(),
                        duration,
                        initial_price,
                        Some(reserve_price),
                        seller_identity,
                        buy_asset.clone(),
                    )
                    .tx_params(tx_params)
                    .call_params(call_params)
                    .call()
                    .await
                    .unwrap();
            }
            _ => {}
        }
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_initial_nft_price_not_one() {
        let (
            deployer,
            seller,
            _,
            _,
            auction_contract_id,
            _,
            sell_nft_contract_id,
            _,
            buy_nft_contract_id,
        ) = setup().await;
        let (sell_count, initial_count, reserve_count, duration, access_control) =
            defaults_nft().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let auction_identity = Identity::ContractId(auction_contract_id.into());

        constructor(
            access_control,
            &seller.nft,
            seller_identity.clone(),
            sell_count,
        )
        .await;
        mint(sell_count, &seller.nft, seller_identity.clone()).await;
        approve(auction_identity.clone(), &seller.nft, 0).await;

        let sell_asset = nft_asset(sell_nft_contract_id, 0).await;
        let buy_asset = nft_asset(buy_nft_contract_id, 0).await;

        create(
            buy_asset.clone(),
            &seller.auction,
            duration,
            2,
            Some(reserve_count),
            seller_identity.clone(),
            sell_asset.clone(),
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_sender_does_not_own_nft() {
        let (
            deployer,
            seller,
            _,
            _,
            auction_contract_id,
            _,
            sell_nft_contract_id,
            _,
            buy_nft_contract_id,
        ) = setup().await;
        let (sell_count, initial_count, reserve_count, duration, access_control) =
            defaults_nft().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let auction_identity = Identity::ContractId(auction_contract_id.into());

        constructor(
            access_control,
            &seller.nft,
            seller_identity.clone(),
            sell_count,
        )
        .await;
        mint(sell_count, &seller.nft, auction_identity.clone()).await;
        approve(auction_identity.clone(), &seller.nft, 0).await;

        let sell_asset = nft_asset(sell_nft_contract_id, 0).await;
        let buy_asset = nft_asset(buy_nft_contract_id, 0).await;

        create(
            buy_asset.clone(),
            &seller.auction,
            duration,
            initial_count,
            Some(reserve_count),
            seller_identity.clone(),
            sell_asset.clone(),
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_auction_not_approved_for_transfer() {
        let (
            deployer,
            seller,
            _,
            _,
            auction_contract_id,
            _,
            sell_nft_contract_id,
            _,
            buy_nft_contract_id,
        ) = setup().await;
        let (sell_count, initial_count, reserve_count, duration, access_control) =
            defaults_nft().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());

        constructor(
            access_control,
            &seller.nft,
            seller_identity.clone(),
            sell_count,
        )
        .await;
        mint(sell_count, &seller.nft, seller_identity.clone()).await;

        let sell_asset = nft_asset(sell_nft_contract_id, 0).await;
        let buy_asset = nft_asset(buy_nft_contract_id, 0).await;

        create(
            buy_asset.clone(),
            &seller.auction,
            duration,
            initial_count,
            Some(reserve_count),
            seller_identity.clone(),
            sell_asset.clone(),
        )
        .await;
    }

    // TODO: Test sending mismatching asset structs
}

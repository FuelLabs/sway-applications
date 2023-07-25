use crate::utils::{
    interface::core::{
        auction::create,
        nft::{approve, mint},
    },
    setup::{defaults_nft, defaults_token, nft_asset, setup, token_asset},
};
use fuels::types::Identity;

mod success {

    use super::*;
    use crate::utils::{
        interface::{
            core::nft::{approve, mint, set_approval_for_all},
            info::auction_info,
        },
        setup::{create_auction_copy, State},
    };

    #[tokio::test]
    async fn creates_multiple_auctions() {
        let (deployer, seller, _, _, _, sell_asset_contract_id, _, buy_asset_contract_id, _) =
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration, _initial_wallet_amount) =
            defaults_token().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = token_asset(sell_asset_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_asset_contract_id, 0).await;
        let provider = deployer.wallet.provider().unwrap();

        let auction = auction_info(0, &seller.auction).await;
        assert!(auction.is_none());

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

        let total_duration = (provider.latest_block_height().await.unwrap() as u64) + duration;
        let auction1 = auction_info(auction_id1, &seller.auction).await;
        assert!(auction1.is_some());

        let auction1_copy = create_auction_copy(
            buy_asset.clone(),
            None,
            total_duration,
            initial_price,
            Some(reserve_price),
            sell_asset.clone(),
            seller_identity.clone(),
            State::Open,
        )
        .await;
        assert_eq!(auction1.unwrap(), auction1_copy);

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

        let total_duration = (provider.latest_block_height().await.unwrap() as u64) + duration;
        let auction2 = auction_info(auction_id2, &seller.auction).await;
        assert!(auction2.is_some());

        let auction2_copy = create_auction_copy(
            buy_asset.clone(),
            None,
            total_duration,
            initial_price,
            Some(reserve_price),
            sell_asset.clone(),
            seller_identity.clone(),
            State::Open,
        )
        .await;
        assert_eq!(auction2.unwrap(), auction2_copy);
    }

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
        let (sell_count, initial_count, reserve_count, duration) = defaults_nft().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let auction_identity = Identity::ContractId(auction_contract_id);
        let sell_asset = nft_asset(sell_nft_contract_id, 0).await;
        let buy_asset = nft_asset(buy_nft_contract_id, 0).await;
        let provider = deployer.wallet.provider().unwrap();

        mint(sell_count, &seller.nft, seller_identity.clone()).await;
        approve(Some(auction_identity.clone()), &seller.nft, 0).await;

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

        let total_duration = (provider.latest_block_height().await.unwrap() as u64) + duration;
        let auction = auction_info(auction_id, &seller.auction).await;
        assert!(auction.is_some());

        let auction_copy = create_auction_copy(
            buy_asset.clone(),
            None,
            total_duration,
            initial_count,
            Some(reserve_count),
            sell_asset,
            seller_identity,
            State::Open,
        )
        .await;
        assert_eq!(auction.unwrap(), auction_copy);
    }

    #[tokio::test]
    async fn creates_new_nft_auction_with_approval_for_all() {
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
        let (sell_count, initial_count, reserve_count, duration) = defaults_nft().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let auction_identity = Identity::ContractId(auction_contract_id);
        let sell_asset = nft_asset(sell_nft_contract_id, 0).await;
        let buy_asset = nft_asset(buy_nft_contract_id, 0).await;
        let provider = deployer.wallet.provider().unwrap();
        let auction = auction_info(0, &seller.auction).await;

        mint(sell_count, &seller.nft, seller_identity.clone()).await;
        set_approval_for_all(true, &seller.nft, auction_identity.clone()).await;

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

        let total_duration = (provider.latest_block_height().await.unwrap() as u64) + duration;
        let auction = auction_info(auction_id, &seller.auction).await;
        assert!(auction.is_some());

        let auction_copy = create_auction_copy(
            buy_asset.clone(),
            None,
            total_duration,
            initial_count,
            Some(reserve_count),
            sell_asset,
            seller_identity,
            State::Open,
        )
        .await;
        assert_eq!(auction.unwrap(), auction_copy);
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
        let (sell_count, initial_count, reserve_count, duration) = defaults_nft().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let auction_identity = Identity::ContractId(auction_contract_id);
        let sell_asset = nft_asset(sell_nft_contract_id, 0).await;
        let buy_asset = token_asset(buy_asset_contract_id, 0).await;
        let provider = deployer.wallet.provider().unwrap();
        let auction = auction_info(0, &seller.auction).await;

        mint(sell_count, &seller.nft, seller_identity.clone()).await;
        approve(Some(auction_identity.clone()), &seller.nft, 0).await;

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

        let total_duration = (provider.latest_block_height().await.unwrap() as u64) + duration;
        let auction = auction_info(auction_id, &seller.auction).await;
        assert!(auction.is_some());

        let auction_copy = create_auction_copy(
            buy_asset.clone(),
            None,
            total_duration,
            initial_count,
            Some(reserve_count),
            sell_asset,
            seller_identity,
            State::Open,
        )
        .await;
        assert_eq!(auction.unwrap(), auction_copy);
    }

    #[tokio::test]
    async fn creates_new_token_auction() {
        let (deployer, seller, _, _, _, sell_asset_contract_id, _, buy_asset_contract_id, _) =
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration, _initial_wallet_amount) =
            defaults_token().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = token_asset(sell_asset_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_asset_contract_id, 0).await;
        let provider = deployer.wallet.provider().unwrap();

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

        let total_duration = (provider.latest_block_height().await.unwrap() as u64) + duration;
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
            State::Open,
        )
        .await;
        assert_eq!(auction.unwrap(), auction_copy);
    }

    #[tokio::test]
    async fn creates_new_token_auction_with_nft_bid_asset() {
        let (deployer, seller, _, _, _, sell_asset_contract_id, _, _, buy_nft_contract_id) =
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration, _initial_wallet_amount) =
            defaults_token().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = token_asset(sell_asset_contract_id, sell_amount).await;
        let buy_asset = nft_asset(buy_nft_contract_id, 0).await;
        let provider = deployer.wallet.provider().unwrap();

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

        let total_duration = (provider.latest_block_height().await.unwrap() as u64) + duration;
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
            State::Open,
        )
        .await;
        assert_eq!(auction.unwrap(), auction_copy);
    }

    #[tokio::test]
    async fn creates_new_token_auction_without_reserve() {
        let (deployer, seller, _, _, _, sell_asset_contract_id, _, buy_asset_contract_id, _) =
            setup().await;
        let (sell_amount, initial_price, _, duration, _) = defaults_token().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = token_asset(sell_asset_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_asset_contract_id, 0).await;
        let provider = deployer.wallet.provider().unwrap();

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

        let total_duration = (provider.latest_block_height().await.unwrap() as u64) + duration;
        let auction = auction_info(auction_id, &seller.auction).await;
        assert!(auction.is_some());

        let auction_copy = create_auction_copy(
            buy_asset.clone(),
            None,
            total_duration,
            initial_price,
            None,
            sell_asset,
            seller_identity,
            State::Open,
        )
        .await;
        assert_eq!(auction.unwrap(), auction_copy);
    }

    #[tokio::test]
    async fn creates_new_token_auction_with_reserve_equal_to_initial_price() {
        let (deployer, seller, _, _, _, sell_asset_contract_id, _, buy_asset_contract_id, _) =
            setup().await;
        let (sell_amount, initial_price, _, duration, _) = defaults_token().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = token_asset(sell_asset_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_asset_contract_id, 0).await;
        let provider = deployer.wallet.provider().unwrap();

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

        let total_duration = (provider.latest_block_height().await.unwrap() as u64) + duration;
        let auction = auction_info(auction_id, &seller.auction).await;
        assert!(auction.is_some());

        let auction_copy = create_auction_copy(
            buy_asset.clone(),
            None,
            total_duration,
            initial_price,
            Some(initial_price),
            sell_asset,
            seller_identity,
            State::Open,
        )
        .await;
        assert_eq!(auction.unwrap(), auction_copy);
    }
}

mod revert {

    use super::*;
    use fuels::prelude::{AssetId, CallParameters, TxParameters};

    #[tokio::test]
    #[should_panic(expected = "ReserveLessThanInitialPrice")]
    async fn when_reserve_is_less_than_initial_price() {
        let (_, seller, _, _, _, sell_asset_contract_id, _, buy_asset_contract_id, _) =
            setup().await;
        let (sell_amount, initial_price, _, duration, _) = defaults_token().await;

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
    #[should_panic(expected = "ReserveLessThanInitialPrice")]
    async fn when_reserve_is_zero() {
        let (_, seller, _, _, _, sell_asset_contract_id, _, buy_asset_contract_id, _) =
            setup().await;
        let (sell_amount, initial_price, _, duration, _) = defaults_token().await;

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
    #[should_panic(expected = "AuctionDurationNotProvided")]
    async fn when_duration_is_zero() {
        let (_, seller, _, _, _, sell_asset_contract_id, _, buy_asset_contract_id, _) =
            setup().await;
        let (sell_amount, initial_price, reserve_price, _, _) = defaults_token().await;

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
    #[should_panic(expected = "BidAssetAmountNotZero")]
    async fn when_bid_token_amount_not_zero() {
        let (_, seller, _, _, _, sell_asset_contract_id, _, buy_asset_contract_id, _) =
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration, _initial_wallet_amount) =
            defaults_token().await;

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
    #[should_panic(expected = "BidAssetAmountNotZero")]
    async fn when_bid_nft_id_not_zero() {
        let (_, seller, _, _, _, sell_asset_contract_id, _, _, buy_nft_contract_id) = setup().await;
        let (sell_amount, initial_price, reserve_price, duration, _initial_wallet_amount) =
            defaults_token().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = token_asset(sell_asset_contract_id, sell_amount).await;
        let buy_asset = nft_asset(buy_nft_contract_id, 1).await;

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
    #[should_panic(expected = "InitialPriceCannotBeZero")]
    async fn when_initial_token_price_is_zero() {
        let (_, seller, _, _, _, sell_asset_contract_id, _, buy_asset_contract_id, _) =
            setup().await;
        let (sell_amount, _, reserve_price, duration, _) = defaults_token().await;

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
    #[should_panic(expected = "IncorrectAmountProvided")]
    async fn when_token_asset_sent_less_than_sell_struct() {
        let (_, seller, _, _, _, sell_asset_contract_id, _, buy_asset_contract_id, _) =
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration, _initial_wallet_amount) =
            defaults_token().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = token_asset(sell_asset_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_asset_contract_id, 0).await;

        let tx_params = TxParameters::new(0, 2_000_000, 0);
        let call_params = CallParameters::new(
            sell_amount - 1,
            AssetId::from(*sell_asset_contract_id),
            1_000_000,
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
                sell_asset.clone(),
            )
            .tx_params(tx_params)
            .call_params(call_params)
            .unwrap()
            .call()
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "IncorrectAmountProvided")]
    async fn when_token_asset_sent_greater_than_sell_struct() {
        let (_, seller, _, _, _, sell_asset_contract_id, _, buy_asset_contract_id, _) =
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration, _initial_wallet_amount) =
            defaults_token().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = token_asset(sell_asset_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_asset_contract_id, 0).await;

        let tx_params = TxParameters::new(0, 2_000_000, 0);
        let call_params = CallParameters::new(
            sell_amount + 1,
            AssetId::from(*sell_asset_contract_id),
            1_000_000,
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
                sell_asset.clone(),
            )
            .tx_params(tx_params)
            .call_params(call_params)
            .unwrap()
            .call()
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "IncorrectAmountProvided")]
    async fn when_token_struct_not_correct_type() {
        let (_, seller, _, _, _, sell_asset_contract_id, _, buy_asset_contract_id, _) =
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration, _initial_wallet_amount) =
            defaults_token().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let buy_asset = token_asset(buy_asset_contract_id, 0).await;

        let tx_params = TxParameters::new(0, 2_000_000, 0);
        let call_params = CallParameters::new(
            sell_amount,
            AssetId::from(*sell_asset_contract_id),
            1_000_000,
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
            .unwrap()
            .call()
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "IncorrectAssetProvided")]
    async fn when_token_sent_not_correct_type() {
        let (_, seller, _buyer1, _, _, sell_asset_contract_id, _, buy_asset_contract_id, _) =
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration, _initial_wallet_amount) =
            defaults_token().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = token_asset(sell_asset_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_asset_contract_id, 0).await;

        let tx_params = TxParameters::new(0, 2_000_000, 0);
        let call_params = CallParameters::new(
            sell_amount,
            AssetId::from(*buy_asset_contract_id),
            1_000_000,
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
                sell_asset.clone(),
            )
            .tx_params(tx_params)
            .call_params(call_params)
            .unwrap()
            .call()
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "ReserveLessThanInitialPrice")]
    async fn when_initial_nft_price_not_one() {
        let (_, seller, _, _, auction_contract_id, _, sell_nft_contract_id, _, buy_nft_contract_id) =
            setup().await;
        let (sell_count, _, reserve_count, duration) = defaults_nft().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let auction_identity = Identity::ContractId(auction_contract_id);
        let sell_asset = nft_asset(sell_nft_contract_id, 0).await;
        let buy_asset = nft_asset(buy_nft_contract_id, 0).await;

        mint(sell_count, &seller.nft, seller_identity.clone()).await;
        approve(Some(auction_identity.clone()), &seller.nft, 0).await;

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
    #[should_panic(expected = "SenderNotOwner")]
    async fn when_sender_does_not_own_nft() {
        let (_, seller, _, _, auction_contract_id, _, sell_nft_contract_id, _, buy_nft_contract_id) =
            setup().await;
        let (sell_count, initial_count, reserve_count, duration) = defaults_nft().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let auction_identity = Identity::ContractId(auction_contract_id);
        let sell_asset = nft_asset(sell_nft_contract_id, 0).await;
        let buy_asset = nft_asset(buy_nft_contract_id, 0).await;

        mint(sell_count, &seller.nft, auction_identity.clone()).await;
        approve(Some(auction_identity.clone()), &seller.nft, 0).await;

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

    #[ignore]
    #[tokio::test]
    // TODO: test is not set up to hit the error properly: https://github.com/FuelLabs/sway-applications/issues/330
    #[should_panic(expected = "NFTTransferNotApproved")]
    async fn when_auction_not_approved_for_transfer() {
        let (_, seller, _, _, _, _, sell_nft_contract_id, _, buy_nft_contract_id) = setup().await;
        let (sell_count, initial_count, reserve_count, duration) = defaults_nft().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = nft_asset(sell_nft_contract_id, 0).await;
        let buy_asset = nft_asset(buy_nft_contract_id, 0).await;

        mint(sell_count, &seller.nft, seller_identity.clone()).await;

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
}

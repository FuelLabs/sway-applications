use crate::utils::{
    interface::core::auction::{bid, create, withdraw},
    setup::{defaults_token, setup, token_asset},
};
use fuels::types::Identity;

mod success {

    use super::*;
    use crate::utils::{
        interface::{
            core::nft::{approve, mint, owner_of},
            info::deposit_balance,
        },
        setup::{defaults_nft, nft_asset},
    };
    use fuels::{accounts::ViewOnlyAccount, prelude::AssetId};

    #[tokio::test]
    async fn buyer_withdraws_expired_auction() {
        let (deployer, seller, buyer1, _, _, sell_token_contract_id, _, buy_token_contract_id, _) =
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration, initial_wallet_amount) =
            defaults_token().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let buyer1_identity = Identity::Address(buyer1.wallet.address().into());
        let sell_asset = token_asset(sell_token_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_token_contract_id, 0).await;
        let bid_asset = token_asset(buy_token_contract_id, initial_price).await;
        let provider = deployer.wallet.provider().unwrap();

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

        let _result = provider.produce_blocks(duration + 1, Option::None).await;

        assert_eq!(
            deposit_balance(auction_id, &seller.auction, buyer1_identity.clone())
                .await
                .unwrap(),
            bid_asset
        );

        withdraw(auction_id, &buyer1.auction, sell_asset).await;

        assert_eq!(
            deposit_balance(auction_id, &seller.auction, buyer1_identity.clone()).await,
            None
        );
        assert_eq!(
            buyer1
                .wallet
                .get_asset_balance(&AssetId::new(*sell_token_contract_id))
                .await
                .unwrap(),
            sell_amount + initial_wallet_amount
        );
    }

    #[tokio::test]
    async fn buyer_withdraws_nft() {
        let (
            _,
            seller,
            buyer1,
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
        let buyer1_identity = Identity::Address(buyer1.wallet.address().into());
        let sell_asset = nft_asset(sell_nft_contract_id, 0).await;
        let buy_asset = nft_asset(buy_nft_contract_id, 0).await;
        let bid_asset = nft_asset(buy_nft_contract_id, 0).await;

        mint(sell_count, &seller.nft, seller_identity.clone()).await;
        approve(Some(auction_identity.clone()), &seller.nft, 0).await;
        mint(reserve_count, &buyer1.nft, buyer1_identity.clone()).await;
        approve(Some(auction_identity.clone()), &buyer1.nft, 0).await;

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

        assert_eq!(
            deposit_balance(auction_id, &seller.auction, buyer1_identity.clone())
                .await
                .unwrap(),
            bid_asset
        );

        withdraw(auction_id, &buyer1.auction, sell_asset).await;

        assert_eq!(
            deposit_balance(auction_id, &seller.auction, buyer1_identity.clone()).await,
            None
        );
        assert_eq!(owner_of(&seller.nft, 0).await.unwrap(), buyer1_identity);
    }

    #[tokio::test]
    async fn buyer_withdraws_tokens() {
        let (_, seller, buyer1, _, _, sell_token_contract_id, _, buy_token_contract_id, _) =
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration, initial_wallet_amount) =
            defaults_token().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let buyer1_identity = Identity::Address(buyer1.wallet.address().into());
        let sell_asset = token_asset(sell_token_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_token_contract_id, 0).await;
        let bid_asset = token_asset(buy_token_contract_id, reserve_price).await;

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

        assert_eq!(
            deposit_balance(auction_id, &seller.auction, buyer1_identity.clone())
                .await
                .unwrap(),
            bid_asset
        );

        withdraw(auction_id, &buyer1.auction, sell_asset).await;

        assert_eq!(
            deposit_balance(auction_id, &seller.auction, buyer1_identity.clone()).await,
            None
        );
        assert_eq!(
            buyer1
                .wallet
                .get_asset_balance(&AssetId::new(*sell_token_contract_id))
                .await
                .unwrap(),
            sell_amount + initial_wallet_amount
        );
    }

    #[tokio::test]
    async fn out_bid_withdraws_tokens() {
        let (_, seller, buyer1, buyer2, _, sell_token_contract_id, _, buy_token_contract_id, _) =
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration, initial_wallet_amount) =
            defaults_token().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let buyer1_identity = Identity::Address(buyer1.wallet.address().into());
        let sell_asset = token_asset(sell_token_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_token_contract_id, 0).await;
        let bid1_asset = token_asset(buy_token_contract_id, initial_price).await;
        let bid2_asset = token_asset(buy_token_contract_id, reserve_price).await;

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

        bid(auction_id, bid1_asset.clone(), &buyer1.auction).await;
        bid(auction_id, bid2_asset.clone(), &buyer2.auction).await;

        assert_eq!(
            deposit_balance(auction_id, &buyer1.auction, buyer1_identity.clone())
                .await
                .unwrap(),
            bid1_asset
        );

        withdraw(auction_id, &buyer1.auction, bid1_asset).await;

        assert_eq!(
            deposit_balance(auction_id, &seller.auction, buyer1_identity.clone()).await,
            None
        );
        assert_eq!(
            buyer1
                .wallet
                .get_asset_balance(&AssetId::new(*buy_token_contract_id))
                .await
                .unwrap(),
            initial_wallet_amount
        );
    }

    #[tokio::test]
    async fn seller_withdraws_nft() {
        let (
            _,
            seller,
            buyer1,
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
        let buyer1_identity = Identity::Address(buyer1.wallet.address().into());
        let sell_asset = nft_asset(sell_nft_contract_id, 0).await;
        let buy_asset = nft_asset(buy_nft_contract_id, 0).await;
        let bid_asset = nft_asset(buy_nft_contract_id, 0).await;

        mint(sell_count, &seller.nft, seller_identity.clone()).await;
        approve(Some(auction_identity.clone()), &seller.nft, 0).await;
        mint(reserve_count, &buyer1.nft, buyer1_identity.clone()).await;
        approve(Some(auction_identity.clone()), &buyer1.nft, 0).await;

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

        assert_eq!(
            deposit_balance(auction_id, &seller.auction, seller_identity.clone())
                .await
                .unwrap(),
            sell_asset
        );

        withdraw(auction_id, &seller.auction, bid_asset).await;

        assert_eq!(
            deposit_balance(auction_id, &seller.auction, seller_identity.clone()).await,
            None
        );
        assert_eq!(owner_of(&buyer1.nft, 0).await.unwrap(), seller_identity);
    }

    #[tokio::test]
    async fn seller_withdraws_no_bids() {
        let (deployer, seller, _, _, _, sell_token_contract_id, _, buy_token_contract_id, _) =
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration, initial_wallet_amount) =
            defaults_token().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = token_asset(sell_token_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_token_contract_id, 0).await;
        let provider = deployer.wallet.provider().unwrap();

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

        let _result = provider.produce_blocks(duration + 1, Option::None).await;

        assert_eq!(
            deposit_balance(auction_id, &seller.auction, seller_identity.clone())
                .await
                .unwrap(),
            sell_asset
        );

        withdraw(auction_id, &seller.auction, sell_asset).await;

        assert_eq!(
            deposit_balance(auction_id, &seller.auction, seller_identity.clone()).await,
            None
        );
        assert_eq!(
            seller
                .wallet
                .get_asset_balance(&AssetId::new(*sell_token_contract_id))
                .await
                .unwrap(),
            initial_wallet_amount
        );
    }

    #[tokio::test]
    async fn seller_withdraws_tokens() {
        let (_, seller, buyer1, _, _, sell_token_contract_id, _, buy_token_contract_id, _) =
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration, initial_wallet_amount) =
            defaults_token().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = token_asset(sell_token_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_token_contract_id, 0).await;
        let bid_asset = token_asset(buy_token_contract_id, reserve_price).await;

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

        assert_eq!(
            deposit_balance(auction_id, &seller.auction, seller_identity.clone())
                .await
                .unwrap(),
            sell_asset
        );

        withdraw(auction_id, &seller.auction, buy_asset).await;

        assert_eq!(
            deposit_balance(auction_id, &seller.auction, seller_identity.clone()).await,
            None
        );
        assert_eq!(
            seller
                .wallet
                .get_asset_balance(&AssetId::new(*buy_token_contract_id))
                .await
                .unwrap(),
            reserve_price + initial_wallet_amount
        );
    }

    #[tokio::test]
    async fn withdraws_multiple_auctions() {
        let (_, seller, buyer1, _, _, sell_token_contract_id, _, buy_token_contract_id, _) =
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration, initial_wallet_amount) =
            defaults_token().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let buyer1_identity = Identity::Address(buyer1.wallet.address().into());
        let sell_asset = token_asset(sell_token_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_token_contract_id, 0).await;
        let bid_asset = token_asset(buy_token_contract_id, reserve_price).await;

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

        bid(auction_id1, bid_asset.clone(), &buyer1.auction).await;
        bid(auction_id2, bid_asset.clone(), &buyer1.auction).await;

        assert_eq!(
            deposit_balance(auction_id1, &seller.auction, buyer1_identity.clone())
                .await
                .unwrap(),
            bid_asset
        );
        assert_eq!(
            deposit_balance(auction_id2, &seller.auction, buyer1_identity.clone())
                .await
                .unwrap(),
            bid_asset
        );

        withdraw(auction_id1, &buyer1.auction, sell_asset.clone()).await;

        assert_eq!(
            deposit_balance(auction_id1, &seller.auction, buyer1_identity.clone()).await,
            None
        );
        assert_eq!(
            deposit_balance(auction_id2, &seller.auction, buyer1_identity.clone())
                .await
                .unwrap(),
            bid_asset
        );
        assert_eq!(
            buyer1
                .wallet
                .get_asset_balance(&AssetId::new(*sell_token_contract_id))
                .await
                .unwrap(),
            sell_amount + initial_wallet_amount
        );

        withdraw(auction_id2, &buyer1.auction, sell_asset.clone()).await;

        assert_eq!(
            deposit_balance(auction_id1, &seller.auction, buyer1_identity.clone()).await,
            None
        );
        assert_eq!(
            deposit_balance(auction_id2, &seller.auction, buyer1_identity.clone()).await,
            None
        );
        assert_eq!(
            buyer1
                .wallet
                .get_asset_balance(&AssetId::new(*sell_token_contract_id))
                .await
                .unwrap(),
            (sell_amount * 2) + initial_wallet_amount
        );
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "AuctionDoesNotExist")]
    async fn when_auction_id_does_not_exist() {
        let (_, _, buyer1, _, _, sell_token_contract_id, _, _, _) = setup().await;
        let (sell_amount, _, _, _, _) = defaults_token().await;
        let sell_asset = token_asset(sell_token_contract_id, sell_amount).await;

        withdraw(0, &buyer1.auction, sell_asset).await;
    }

    #[tokio::test]
    #[should_panic(expected = "AuctionIsNotClosed")]
    async fn when_auction_has_not_ended() {
        let (_, seller, buyer1, _, _, sell_token_contract_id, _, buy_token_contract_id, _) =
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration, _initial_wallet_amount) =
            defaults_token().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = token_asset(sell_token_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_token_contract_id, 0).await;
        let bid_asset = token_asset(buy_token_contract_id, initial_price).await;

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

        withdraw(auction_id, &buyer1.auction, sell_asset).await;
    }

    #[tokio::test]
    #[should_panic(expected = "UserHasAlreadyWithdrawn")]
    async fn when_sender_withdraws_twice() {
        let (_, seller, buyer1, _, _, sell_token_contract_id, _, buy_token_contract_id, _) =
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration, _initial_wallet_amount) =
            defaults_token().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = token_asset(sell_token_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_token_contract_id, 0).await;
        let bid_asset = token_asset(buy_token_contract_id, reserve_price).await;

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

        withdraw(auction_id, &buyer1.auction, sell_asset.clone()).await;
        withdraw(auction_id, &buyer1.auction, sell_asset.clone()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "UserHasAlreadyWithdrawn")]
    async fn when_sender_did_not_deposit_balance() {
        let (_, seller, buyer1, buyer2, _, sell_token_contract_id, _, buy_token_contract_id, _) =
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration, _initial_wallet_amount) =
            defaults_token().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = token_asset(sell_token_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_token_contract_id, 0).await;
        let bid_asset = token_asset(buy_token_contract_id, reserve_price).await;

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

        withdraw(auction_id, &buyer2.auction, sell_asset).await;
    }
}

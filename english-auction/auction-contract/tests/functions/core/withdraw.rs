use crate::utils::{
    interface::core::auction::{bid, create, withdraw},
    setup::{defaults, setup},
};
use fuels::types::Identity;

mod success {

    use super::*;
    use crate::utils::interface::info::deposit_balance;
    use fuels::accounts::ViewOnlyAccount;

    #[tokio::test]
    async fn buyer_withdraws_expired_auction() {
        let (deployer, seller, buyer1, _, _, sell_asset, buy_asset) = setup().await;
        let (sell_amount, initial_price, reserve_price, duration, initial_wallet_amount) =
            defaults().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let buyer1_identity = Identity::Address(buyer1.wallet.address().into());
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

        bid(auction_id, buy_asset, initial_price, &buyer1.auction).await;

        let _result = provider.produce_blocks(duration + 1, Option::None).await;

        assert_eq!(
            deposit_balance(auction_id, &seller.auction, buyer1_identity)
                .await
                .unwrap(),
            initial_price
        );

        withdraw(auction_id, &buyer1.auction).await;

        assert_eq!(
            deposit_balance(auction_id, &seller.auction, buyer1_identity).await,
            None
        );
        assert_eq!(
            buyer1.wallet.get_asset_balance(&sell_asset).await.unwrap(),
            sell_amount + initial_wallet_amount
        );
    }

    #[tokio::test]
    async fn buyer_withdraws_assets() {
        let (_, seller, buyer1, _, _, sell_asset, buy_asset) = setup().await;
        let (sell_amount, initial_price, reserve_price, duration, initial_wallet_amount) =
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

        bid(auction_id, buy_asset, reserve_price, &buyer1.auction).await;

        assert_eq!(
            deposit_balance(auction_id, &seller.auction, buyer1_identity)
                .await
                .unwrap(),
            reserve_price
        );

        withdraw(auction_id, &buyer1.auction).await;

        assert_eq!(
            deposit_balance(auction_id, &seller.auction, buyer1_identity).await,
            None
        );
        assert_eq!(
            buyer1.wallet.get_asset_balance(&sell_asset).await.unwrap(),
            sell_amount + initial_wallet_amount
        );
    }

    #[tokio::test]
    async fn out_bid_withdraws_assets() {
        let (_, seller, buyer1, buyer2, _, sell_asset, buy_asset) = setup().await;
        let (sell_amount, initial_price, reserve_price, duration, initial_wallet_amount) =
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

        bid(auction_id, buy_asset, initial_price, &buyer1.auction).await;
        bid(auction_id, buy_asset, reserve_price, &buyer2.auction).await;

        assert_eq!(
            deposit_balance(auction_id, &buyer1.auction, buyer1_identity)
                .await
                .unwrap(),
            initial_price
        );

        withdraw(auction_id, &buyer1.auction).await;

        assert_eq!(
            deposit_balance(auction_id, &seller.auction, buyer1_identity).await,
            None
        );
        assert_eq!(
            buyer1.wallet.get_asset_balance(&buy_asset).await.unwrap(),
            initial_wallet_amount
        );
    }

    #[tokio::test]
    async fn seller_withdraws_no_bids() {
        let (deployer, seller, _, _, _, sell_asset, buy_asset) = setup().await;
        let (sell_amount, initial_price, reserve_price, duration, initial_wallet_amount) =
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

        assert_eq!(
            deposit_balance(auction_id, &seller.auction, seller_identity)
                .await
                .unwrap(),
            sell_amount
        );

        withdraw(auction_id, &seller.auction).await;

        assert_eq!(
            deposit_balance(auction_id, &seller.auction, seller_identity).await,
            None
        );
        assert_eq!(
            seller.wallet.get_asset_balance(&sell_asset).await.unwrap(),
            initial_wallet_amount
        );
    }

    #[tokio::test]
    async fn seller_withdraws_assets() {
        let (_, seller, buyer1, _, _, sell_asset, buy_asset) = setup().await;
        let (sell_amount, initial_price, reserve_price, duration, initial_wallet_amount) =
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

        assert_eq!(
            deposit_balance(auction_id, &seller.auction, seller_identity)
                .await
                .unwrap(),
            sell_amount
        );

        withdraw(auction_id, &seller.auction).await;

        assert_eq!(
            deposit_balance(auction_id, &seller.auction, seller_identity).await,
            None
        );
        assert_eq!(
            seller.wallet.get_asset_balance(&buy_asset).await.unwrap(),
            reserve_price + initial_wallet_amount
        );
    }

    #[tokio::test]
    async fn withdraws_multiple_auctions() {
        let (_, seller, buyer1, _, _, sell_asset, buy_asset) = setup().await;
        let (sell_amount, initial_price, reserve_price, duration, initial_wallet_amount) =
            defaults().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let buyer1_identity = Identity::Address(buyer1.wallet.address().into());

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

        bid(auction_id1, buy_asset, reserve_price, &buyer1.auction).await;
        bid(auction_id2, buy_asset, reserve_price, &buyer1.auction).await;

        assert_eq!(
            deposit_balance(auction_id1, &seller.auction, buyer1_identity)
                .await
                .unwrap(),
            reserve_price
        );
        assert_eq!(
            deposit_balance(auction_id2, &seller.auction, buyer1_identity)
                .await
                .unwrap(),
            reserve_price
        );

        withdraw(auction_id1, &buyer1.auction).await;

        assert_eq!(
            deposit_balance(auction_id1, &seller.auction, buyer1_identity).await,
            None
        );
        assert_eq!(
            deposit_balance(auction_id2, &seller.auction, buyer1_identity)
                .await
                .unwrap(),
            reserve_price
        );
        assert_eq!(
            buyer1.wallet.get_asset_balance(&sell_asset).await.unwrap(),
            sell_amount + initial_wallet_amount
        );

        withdraw(auction_id2, &buyer1.auction).await;

        assert_eq!(
            deposit_balance(auction_id1, &seller.auction, buyer1_identity).await,
            None
        );
        assert_eq!(
            deposit_balance(auction_id2, &seller.auction, buyer1_identity).await,
            None
        );
        assert_eq!(
            buyer1.wallet.get_asset_balance(&sell_asset).await.unwrap(),
            (sell_amount * 2) + initial_wallet_amount
        );
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "AuctionDoesNotExist")]
    async fn when_auction_id_does_not_exist() {
        let (_, _, buyer1, _, _, _, _) = setup().await;
        let (_, _, _, _, _) = defaults().await;

        withdraw(0, &buyer1.auction).await;
    }

    #[tokio::test]
    #[should_panic(expected = "AuctionIsNotClosed")]
    async fn when_auction_has_not_ended() {
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

        withdraw(auction_id, &buyer1.auction).await;
    }

    #[tokio::test]
    #[should_panic(expected = "UserHasAlreadyWithdrawn")]
    async fn when_sender_withdraws_twice() {
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

        withdraw(auction_id, &buyer1.auction).await;
        withdraw(auction_id, &buyer1.auction).await;
    }

    #[tokio::test]
    #[should_panic(expected = "UserHasAlreadyWithdrawn")]
    async fn when_sender_did_not_deposit_balance() {
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

        withdraw(auction_id, &buyer2.auction).await;
    }
}

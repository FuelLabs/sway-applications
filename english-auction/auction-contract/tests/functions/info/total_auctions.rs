mod success {
    use crate::utils::{
        interface::{core::auction::create, info::total_auctions},
        setup::{defaults, setup},
    };
    use fuels::types::Identity;

    #[tokio::test]
    async fn returns_one_auction() {
        let (_, seller, _, _, _, sell_asset, buy_asset) = setup().await;
        let (sell_amount, initial_price, reserve_price, duration, _initial_wallet_amount) =
            defaults().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());

        assert_eq!(0, total_auctions(&seller.auction).await);

        create(
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

        assert_eq!(1, total_auctions(&seller.auction).await);
    }

    #[tokio::test]
    async fn returns_expired_auctions() {
        let (deployer, seller, _, _, _, sell_asset, buy_asset) = setup().await;
        let (sell_amount, initial_price, reserve_price, duration, _initial_wallet_amount) =
            defaults().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let provider = deployer.wallet.provider().unwrap();

        assert_eq!(0, total_auctions(&seller.auction).await);

        create(
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

        assert_eq!(1, total_auctions(&seller.auction).await);

        let _result = provider.produce_blocks(duration + 1, Option::None).await;

        assert_eq!(1, total_auctions(&seller.auction).await);
    }

    #[tokio::test]
    async fn returns_multiple_auctions() {
        let (_, seller, _, _, _, sell_asset, buy_asset) = setup().await;
        let (sell_amount, initial_price, reserve_price, duration, _initial_wallet_amount) =
            defaults().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());

        assert_eq!(0, total_auctions(&seller.auction).await);

        create(
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

        assert_eq!(1, total_auctions(&seller.auction).await);

        create(
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

        assert_eq!(2, total_auctions(&seller.auction).await);

        create(
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

        assert_eq!(3, total_auctions(&seller.auction).await);
    }
}

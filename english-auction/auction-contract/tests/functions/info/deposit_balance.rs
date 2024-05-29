mod success {
    use crate::utils::{
        interface::{
            core::auction::{bid, create},
            info::deposit_balance,
        },
        setup::{defaults, setup},
    };
    use fuels::types::Identity;

    #[tokio::test]
    async fn returns_deposit_balance() {
        let (_, seller, buyer1, _, _, sell_asset, buy_asset) = setup().await;
        let (sell_amount, initial_price, reserve_price, duration, _initial_wallet_amount) =
            defaults().await;

        assert!(initial_price > 0);

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

        let buyer1_deposit = deposit_balance(auction_id, &buyer1.auction, buyer1_identity).await;
        assert!(buyer1_deposit.is_none());

        bid(auction_id, buy_asset, initial_price, &buyer1.auction).await;

        let buyer1_deposit = deposit_balance(auction_id, &buyer1.auction, buyer1_identity)
            .await
            .unwrap();
        assert_eq!(buyer1_deposit, initial_price);
    }

    #[tokio::test]
    async fn returns_mutliple_deposits() {
        let (_, seller, buyer1, _, _, sell_asset, buy_asset) = setup().await;
        let (sell_amount, initial_price, reserve_price, duration, _initial_wallet_amount) =
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

        let buyer1_deposit1 = deposit_balance(auction_id1, &buyer1.auction, buyer1_identity).await;
        let buyer1_deposit2 = deposit_balance(auction_id2, &buyer1.auction, buyer1_identity).await;
        assert!(buyer1_deposit1.is_none());
        assert!(buyer1_deposit2.is_none());

        bid(auction_id1, buy_asset, initial_price, &buyer1.auction).await;

        let buyer1_deposit1 = deposit_balance(auction_id1, &buyer1.auction, buyer1_identity)
            .await
            .unwrap();
        let buyer1_deposit2 = deposit_balance(auction_id2, &buyer1.auction, buyer1_identity).await;
        assert_eq!(buyer1_deposit1, initial_price);
        assert!(buyer1_deposit2.is_none());

        bid(auction_id2, buy_asset, initial_price + 1, &buyer1.auction).await;

        let buyer1_deposit1 = deposit_balance(auction_id1, &buyer1.auction, buyer1_identity)
            .await
            .unwrap();
        let buyer1_deposit2 = deposit_balance(auction_id2, &buyer1.auction, buyer1_identity)
            .await
            .unwrap();
        assert_eq!(buyer1_deposit1, initial_price);
        assert_eq!(buyer1_deposit2, initial_price + 1);
    }
}

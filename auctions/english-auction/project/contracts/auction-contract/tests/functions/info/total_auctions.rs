mod success {
    use crate::utils::{
        interface::{
            core::{
                auction::create,
                nft::{approve, mint},
            },
            info::total_auctions,
        },
        setup::{defaults_nft, defaults_token, nft_asset, setup, token_asset},
    };
    use fuels::types::Identity;

    #[tokio::test]
    async fn returns_auctions_of_different_types() {
        let (
            _,
            seller,
            _,
            _,
            auction_contract_id,
            sell_asset_contract_id,
            sell_nft_contract_id,
            buy_asset_contract_id,
            _,
        ) = setup().await;
        let (sell_amount, initial_price, reserve_price, duration, _initial_wallet_amount) =
            defaults_token().await;
        let (sell_count, _, _, _) = defaults_nft().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = token_asset(sell_asset_contract_id, sell_amount).await;
        let sell_nft = nft_asset(sell_nft_contract_id, 0).await;
        let buy_asset = token_asset(buy_asset_contract_id, 0).await;
        let auction_identity = Identity::ContractId(auction_contract_id);

        mint(sell_count, &seller.nft, seller_identity.clone()).await;
        approve(Some(auction_identity.clone()), &seller.nft, 0).await;

        assert_eq!(0, total_auctions(&seller.auction).await);

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

        assert_eq!(1, total_auctions(&seller.auction).await);

        create(
            buy_asset.clone(),
            &seller.auction,
            duration,
            initial_price,
            Some(reserve_price),
            seller_identity.clone(),
            sell_nft.clone(),
        )
        .await;

        assert_eq!(2, total_auctions(&seller.auction).await);

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

        assert_eq!(3, total_auctions(&seller.auction).await);
    }

    #[tokio::test]
    async fn returns_expired_auctions() {
        let (deployer, seller, _, _, _, sell_asset_contract_id, _, buy_asset_contract_id, _) =
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration, _initial_wallet_amount) =
            defaults_token().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = token_asset(sell_asset_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_asset_contract_id, 0).await;
        let provider = deployer.wallet.provider().unwrap();

        assert_eq!(0, total_auctions(&seller.auction).await);

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

        assert_eq!(1, total_auctions(&seller.auction).await);

        let _result = provider.produce_blocks(duration + 1, Option::None).await;

        assert_eq!(1, total_auctions(&seller.auction).await);
    }

    #[tokio::test]
    async fn returns_multiple_auctions() {
        let (_, seller, _, _, _, sell_asset_contract_id, _, buy_asset_contract_id, _) =
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration, _initial_wallet_amount) =
            defaults_token().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = token_asset(sell_asset_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_asset_contract_id, 0).await;

        assert_eq!(0, total_auctions(&seller.auction).await);

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

        assert_eq!(1, total_auctions(&seller.auction).await);

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

        assert_eq!(2, total_auctions(&seller.auction).await);

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

        assert_eq!(3, total_auctions(&seller.auction).await);
    }

    #[tokio::test]
    async fn returns_one_auction() {
        let (_, seller, _, _, _, sell_asset_contract_id, _, buy_asset_contract_id, _) =
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration, _initial_wallet_amount) =
            defaults_token().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = token_asset(sell_asset_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_asset_contract_id, 0).await;

        assert_eq!(0, total_auctions(&seller.auction).await);

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

        assert_eq!(1, total_auctions(&seller.auction).await);
    }
}

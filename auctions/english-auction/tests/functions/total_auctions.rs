use crate::utils::{
    asset_abi_calls::mint_and_send_to_address,
    english_auction_abi_calls::{create, total_auctions},
    nft_abi_calls::{approve, constructor, mint},
    test_helpers::{defaults_nft, defaults_token, nft_asset, setup, token_asset},
};
use fuels::prelude::Identity;

mod success {

    use super::*;

    #[tokio::test]
    async fn returns_one_auction() {
        let (_, seller, _, _, _, sell_asset_contract_id, _, buy_asset_contract_id, _) =
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration) = defaults_token().await;

        mint_and_send_to_address(sell_amount, &seller.asset, seller.wallet.address().into()).await;

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

    #[tokio::test]
    async fn returns_multiple_auctions() {
        let (_, seller, _, _, _, sell_asset_contract_id, _, buy_asset_contract_id, _) =
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration) = defaults_token().await;

        mint_and_send_to_address(sell_amount * 3, &seller.asset, seller.wallet.address().into()).await;

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
    async fn returns_expired_auctions() {
        let (deployer, seller, _, _, _, sell_asset_contract_id, _, buy_asset_contract_id, _) =
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration) = defaults_token().await;

        mint_and_send_to_address(sell_amount, &seller.asset, seller.wallet.address().into()).await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = token_asset(sell_asset_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_asset_contract_id, 0).await;
        let provider = deployer.wallet.get_provider().unwrap();

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

        let _result = provider.produce_blocks(duration + 1).await;

        assert_eq!(1, total_auctions(&seller.auction).await);
    }

    #[tokio::test]
    async fn returns_auctions_of_different_types() {
        let (_, seller, _, _, auction_contract_id, sell_asset_contract_id, sell_nft_contract_id, buy_asset_contract_id, _) =
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration) = defaults_token().await;
        let (sell_count, _, _, _, access_control) =defaults_nft().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let sell_asset = token_asset(sell_asset_contract_id, sell_amount).await;
        let sell_nft = nft_asset(sell_nft_contract_id, 0).await;
        let buy_asset = token_asset(buy_asset_contract_id, 0).await;
        let auction_identity = Identity::ContractId(auction_contract_id.into());

        mint_and_send_to_address(sell_amount * 2, &seller.asset, seller.wallet.address().into()).await;
        constructor(
            access_control,
            &seller.nft,
            seller_identity.clone(),
            sell_count,
        )
        .await;
        mint(sell_count, &seller.nft, seller_identity.clone()).await;
        approve(auction_identity.clone(), &seller.nft, 0).await;

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
}

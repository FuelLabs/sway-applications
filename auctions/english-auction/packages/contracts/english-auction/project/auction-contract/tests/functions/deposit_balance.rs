use crate::utils::{
    asset_abi_calls::mint_and_send_to_address,
    english_auction_abi_calls::{bid, create, deposit_balance},
    nft_abi_calls::{approve, constructor, mint},
    test_helpers::{defaults_nft, defaults_token, nft_asset, setup, token_asset},
};
use fuels::prelude::Identity;

mod success {

    use super::*;

    #[tokio::test]
    async fn returns_mutliple_deposits() {
        let (_, seller, buyer1, _, _, sell_token_contract_id, _, buy_token_contract_id, _) =
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration) = defaults_token().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let buyer1_identity = Identity::Address(buyer1.wallet.address().into());
        let sell_asset = token_asset(sell_token_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_token_contract_id, 0).await;
        let bid1_asset = token_asset(buy_token_contract_id, initial_price).await;
        let bid2_asset = token_asset(buy_token_contract_id, initial_price + 1).await;

        mint_and_send_to_address(
            sell_amount * 2,
            &seller.asset,
            seller.wallet.address().into(),
        )
        .await;
        mint_and_send_to_address(
            (reserve_price * 2) + 1,
            &buyer1.asset,
            buyer1.wallet.address().into(),
        )
        .await;

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

        let buyer1_deposit1 =
            deposit_balance(auction_id1, &buyer1.auction, buyer1_identity.clone()).await;
        let buyer1_deposit2 =
            deposit_balance(auction_id2, &buyer1.auction, buyer1_identity.clone()).await;
        assert!(buyer1_deposit1.is_none());
        assert!(buyer1_deposit2.is_none());

        bid(auction_id1, bid1_asset.clone(), &buyer1.auction).await;

        let buyer1_deposit1 =
            deposit_balance(auction_id1, &buyer1.auction, buyer1_identity.clone())
                .await
                .unwrap();
        let buyer1_deposit2 =
            deposit_balance(auction_id2, &buyer1.auction, buyer1_identity.clone()).await;
        assert_eq!(buyer1_deposit1, bid1_asset);
        assert!(buyer1_deposit2.is_none());

        bid(auction_id2, bid2_asset.clone(), &buyer1.auction).await;

        let buyer1_deposit1 =
            deposit_balance(auction_id1, &buyer1.auction, buyer1_identity.clone())
                .await
                .unwrap();
        let buyer1_deposit2 =
            deposit_balance(auction_id2, &buyer1.auction, buyer1_identity.clone())
                .await
                .unwrap();
        assert_eq!(buyer1_deposit1, bid1_asset);
        assert_eq!(buyer1_deposit2, bid2_asset);
    }

    #[tokio::test]
    async fn returns_nft_deposit_balance() {
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

        let buyer1_deposit =
            deposit_balance(auction_id, &buyer1.auction, buyer1_identity.clone()).await;
        assert!(buyer1_deposit.is_none());

        bid(auction_id, bid_asset.clone(), &buyer1.auction).await;

        let buyer1_deposit = deposit_balance(auction_id, &buyer1.auction, buyer1_identity.clone())
            .await
            .unwrap();
        assert_eq!(buyer1_deposit, bid_asset);
    }

    #[tokio::test]
    async fn returns_token_deposit_balance() {
        let (_, seller, buyer1, _, _, sell_token_contract_id, _, buy_token_contract_id, _) =
            setup().await;
        let (sell_amount, initial_price, reserve_price, duration) = defaults_token().await;

        let seller_identity = Identity::Address(seller.wallet.address().into());
        let buyer1_identity = Identity::Address(buyer1.wallet.address().into());
        let sell_asset = token_asset(sell_token_contract_id, sell_amount).await;
        let buy_asset = token_asset(buy_token_contract_id, 0).await;
        let bid_asset = token_asset(buy_token_contract_id, initial_price).await;

        mint_and_send_to_address(sell_amount, &seller.asset, seller.wallet.address().into()).await;
        mint_and_send_to_address(reserve_price, &buyer1.asset, buyer1.wallet.address().into())
            .await;

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

        let buyer1_deposit =
            deposit_balance(auction_id, &buyer1.auction, buyer1_identity.clone()).await;
        assert!(buyer1_deposit.is_none());

        bid(auction_id, bid_asset.clone(), &buyer1.auction).await;

        let buyer1_deposit = deposit_balance(auction_id, &buyer1.auction, buyer1_identity.clone())
            .await
            .unwrap();
        assert_eq!(buyer1_deposit, bid_asset);
    }
}

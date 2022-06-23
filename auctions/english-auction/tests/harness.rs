mod utils;

use utils::{
    abi_calls::{
        // auction_end_block,
        bid_nft,
        bid_tokens,
        // current_bid,
        init_nft,
        init_token,
        // Uncomment when https://github.com/FuelLabs/fuels-rs/issues/420 is resolved
        // deposits,
        // Uncomment when https://github.com/FuelLabs/fuels-rs/issues/421 is resolved
        //highest_bidder,
        // reserve,
        // sell_amount,
        // sell_asset,
        // state,
        withdraw,
    },
    test_helpers::{setup, nft_asset, token_asset},
    Asset
};

mod asset_test {

    use super::*;

    #[tokio::test]
    #[should_panic]
    async fn assets_different() {
        let (
            _deploy_wallet,
            _seller,
            _buyer1,
            _buyer2,
            sell_asset_id,
            buy_asset_id,
            _sell_amount,
            _inital_price,
            _reserve_price,
            _time,
        ) = setup().await;

        assert_eq!(sell_asset_id, buy_asset_id);
    }
}

mod constructor {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn inits_token() {
            let (
                deploy_wallet,
                seller,
                _buyer1,
                _buyer2,
                sell_asset_id,
                buy_asset_id,
                amount_selling,
                inital_price,
                reserve_price,
                time,
            ) = setup().await;

            let buy_asset_struct: Asset = token_asset(buy_asset_id, 0).await;
            let sell_asset_struct: Asset = token_asset(sell_asset_id, amount_selling).await;

            let auction_id = init_token(
                &deploy_wallet,
                &seller,
                sell_asset_id,
                amount_selling,
                buy_asset_id,
                inital_price,
                reserve_price,
                time,
                buy_asset_struct,
                sell_asset_struct,
            )
            .await;

            // assert_eq!(state(&deploy_wallet, auction_id).await, 1);
            // assert_eq!(current_bid(&deploy_wallet, auction_id).await, 0);
            // TODO: Get the current block to test what block the auction should end
            // assert_eq!(auction_end_block(&deploy_wallet, auction_it).await, 0);
            // assert_eq!(
            //     sell_amount(&deploy_wallet, auction_id).await,
            //     amount_selling
            // );
            // assert_eq!(sell_asset(&deploy_wallet, auction_id).await, sell_asset_id);
            // assert_eq!(reserve(&deploy_wallet, auction_id).await, reserve_price);
        }

        // TODO: Impliment NFT testing
        // #[tokio::test]
        // async fn inits_nft_with_approval() {
        //     let (deploy_wallet, seller, _buyer1, _buyer2, sell_asset_id, buy_asset_id, amount_selling, inital_price, reserve_price, time) = setup().await;

        //     let buy_asset_struct = nft_asset(buy_asset_id, id);
        //     let sell_asset_struct = nft_asset(sell_asset_id, id);

        //     let auction_id = init_nft(
        //                 &deploy_wallet,
        //                 &seller,
        //                 sell_asset_id,
        //                 amount_selling,
        //                 buy_asset_id,
        //                 inital_price,
        //                 reserve_price,
        //                 time,
        //                 buy_asset_struct,
        //                 sell_asset_struct
        //             ).await;

        //     assert_eq!(state(&deploy_wallet, auction_id).await, 1);
        //     assert_eq!(current_bid(&deploy_wallet, auction_id).await, 0);
        //     // TODO: Get the current block to test what block the auction should end
        //     // assert_eq!(auction_end_block(&deploy_wallet, auction_it).await, 0);
        //     assert_eq!(sell_amount(&deploy_wallet, auction_id).await, amount_selling);
        //     assert_eq!(sell_asset(&deploy_wallet, auction_id).await, sell_asset_id);
        //     // assert_eq!(reserve(&deploy_wallet, auction_id).await, reserve_price);
        // }

        // #[tokio::test]
        // async fn inits_nft_with_approval_for_all() {
        //     let (deploy_wallet, seller, _buyer1, _buyer2, sell_asset_id, buy_asset_id, amount_selling, inital_price, reserve_price, time) = setup().await;

        //     let buy_asset_struct = Asset {
        //         contract_id: buy_asset_id,
        //         amount: 0,
        //         nft_id: Option::None()
        //     };

        //     let sell_asset_struct = Asset {
        //         contract_id: sell_asset_id,
        //         amount: amount_selling,
        //         nft_id: Option::None()
        //     };

        //     let auction_id = init_nft(
        //                 &deploy_wallet,
        //                 &seller,
        //                 sell_asset_id,
        //                 amount_selling,
        //                 buy_asset_id,
        //                 inital_price,
        //                 reserve_price,
        //                 time,
        //                 buy_asset_struct,
        //                 sell_asset_struct
        //             ).await;

        //     assert_eq!(state(&deploy_wallet, auction_id).await, 1);
        //     assert_eq!(current_bid(&deploy_wallet, auction_id).await, 0);
        //     // TODO: Get the current block to test what block the auction should end
        //     // assert_eq!(auction_end_block(&deploy_wallet, auction_it).await, 0);
        //     assert_eq!(sell_amount(&deploy_wallet, auction_id).await, amount_selling);
        //     assert_eq!(sell_asset(&deploy_wallet, auction_id).await, sell_asset_id);
        //     // assert_eq!(reserve(&deploy_wallet, auction_id).await, reserve_price);
        // }
    }

    mod reverts {

        use super::*;

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_no_tokens_sent() {
            let (
                deploy_wallet,
                seller,
                _buyer1,
                _buyer2,
                sell_asset_id,
                buy_asset_id,
                amount_selling,
                inital_price,
                reserve_price,
                time,
            ) = setup().await;

            let buy_asset_struct: Asset = token_asset(buy_asset_id, 0).await;
            let sell_asset_struct: Asset = token_asset(sell_asset_id, amount_selling).await;

            let _auction_id = init_token(
                &deploy_wallet,
                &seller,
                sell_asset_id,
                0,
                buy_asset_id,
                inital_price,
                reserve_price,
                time,
                buy_asset_struct,
                sell_asset_struct,
            )
            .await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_inital_price_higher_than_reserve() {
            let (
                deploy_wallet,
                seller,
                _buyer1,
                _buyer2,
                sell_asset_id,
                buy_asset_id,
                amount_selling,
                _inital_price,
                _reserve_price,
                time,
            ) = setup().await;

            let buy_asset_struct: Asset = token_asset(buy_asset_id, 0).await;
            let sell_asset_struct: Asset = token_asset(sell_asset_id, amount_selling).await;

            let _auction_id = init_token(
                &deploy_wallet,
                &seller,
                sell_asset_id,
                amount_selling,
                buy_asset_id,
                2,
                1,
                time,
                buy_asset_struct,
                sell_asset_struct,
            )
            .await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_time_for_auction_is_zero() {
            let (
                deploy_wallet,
                seller,
                _buyer1,
                _buyer2,
                sell_asset_id,
                buy_asset_id,
                amount_selling,
                inital_price,
                reserve_price,
                _time,
            ) = setup().await;

            let buy_asset_struct: Asset = token_asset(buy_asset_id, 0).await;
            let sell_asset_struct: Asset = token_asset(sell_asset_id, amount_selling).await;

            let _auction_id = init_token(
                &deploy_wallet,
                &seller,
                sell_asset_id,
                amount_selling,
                buy_asset_id,
                inital_price,
                reserve_price,
                0,
                buy_asset_struct,
                sell_asset_struct,
            )
            .await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_token_amount_doesnt_match_struct() {
            let (
                deploy_wallet,
                seller,
                _buyer1,
                _buyer2,
                sell_asset_id,
                buy_asset_id,
                amount_selling,
                inital_price,
                reserve_price,
                time,
            ) = setup().await;

            let buy_asset_struct: Asset = token_asset(buy_asset_id, 0).await;
            let sell_asset_struct: Asset = token_asset(sell_asset_id, 0).await;

            let _auction_id = init_token(
                &deploy_wallet,
                &seller,
                sell_asset_id,
                amount_selling,
                buy_asset_id,
                inital_price,
                reserve_price,
                time,
                buy_asset_struct,
                sell_asset_struct,
            )
            .await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_token_asset_doesnt_match_struct() {
            let (
                deploy_wallet,
                seller,
                _buyer1,
                _buyer2,
                sell_asset_id,
                buy_asset_id,
                amount_selling,
                inital_price,
                reserve_price,
                time,
            ) = setup().await;

            let buy_asset_struct: Asset = token_asset(buy_asset_id, 0).await;
            let sell_asset_struct: Asset = token_asset(buy_asset_id, amount_selling).await;

            let _auction_id = init_token(
                &deploy_wallet,
                &seller,
                sell_asset_id,
                amount_selling,
                buy_asset_id,
                inital_price,
                reserve_price,
                time,
                buy_asset_struct,
                sell_asset_struct,
            )
            .await;
        }

        // TODO: Implement NFT
        // #[tokio::test]
        // #[should_panic(expected = "Revert(42)")]
        // async fn panics_when_nft_transfer_not_approved() {
        //     let (deploy_wallet, seller, _buyer1, _buyer2, sell_asset_id, buy_asset_id, amount_selling, inital_price, reserve_price, time) = setup().await;

        //     let buy_asset_struct = Asset {
        //         contract_id: buy_asset_id,
        //         amount: 0,
        //         nft_id: Option::None()
        //     };

        //     let sell_asset_struct = Asset {
        //         contract_id: sell_asset_id,
        //         amount: amount_selling,
        //         nft_id: Option::None()
        //     };

        //     let _auction_id = init_nft(
        //             &deploy_wallet,
        //             &seller,
        //             sell_asset_id,
        //             amount_selling,
        //             buy_asset_id,
        //             inital_price,
        //             reserve_price,
        //             time,
        //             buy_asset_struct,
        //             sell_asset_struct
        //         ).await;
        // }

        // #[tokio::test]
        // #[should_panic(expected = "Revert(42)")]
        // async fn panics_when_nft_amount_doesnt_match_struct() {
        //     let (deploy_wallet, seller, _buyer1, _buyer2, sell_asset_id, buy_asset_id, amount_selling, inital_price, reserve_price, time) = setup().await;

        //     let buy_asset_struct = Asset {
        //         contract_id: buy_asset_id,
        //         amount: 0,
        //         nft_id: Option::None()
        //     };

        //     let sell_asset_struct = Asset {
        //         contract_id: sell_asset_id,
        //         amount: 0,
        //         nft_id: Option::None()
        //     };

        //     let _auction_id = init_nft(
        //             &deploy_wallet,
        //             &seller,
        //             sell_asset_id,
        //             amount_selling,
        //             buy_asset_id,
        //             inital_price,
        //             reserve_price,
        //             time,
        //             buy_asset_struct,
        //             sell_asset_struct
        //         ).await;
        // }

        // #[tokio::test]
        // #[should_panic(expected = "Revert(42)")]
        // async fn panics_when_nft_asset_doesnt_match_struct() {
        //     let (deploy_wallet, seller, _buyer1, _buyer2, sell_asset_id, buy_asset_id, amount_selling, inital_price, reserve_price, time) = setup().await;

        //     deploy_funds(&deploy_wallet, &seller.wallet, 100).await;

        //     let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        //     let call_params = CallParameters::new(Some(amount_selling), Some(AssetId::from(*sell_asset_id)), None);

        //     let buy_asset_struct = Asset {
        //         contract_id: buy_asset_id,
        //         amount: 0,
        //         nft_id: Option::None()
        //     };

        //     let sell_asset_struct = Asset {
        //         contract_id: buy_asset_id,
        //         amount: amount_selling,
        //         nft_id: Option::None()
        //     };

        //     let _auction_id = init_nft(
        //             &deploy_wallet,
        //             &seller,
        //             sell_asset_id,
        //             amount_selling,
        //             buy_asset_id,
        //             inital_price,
        //             reserve_price,
        //             time,
        //             buy_asset_struct,
        //             sell_asset_struct
        //         ).await;
        // }

        // #[tokio::test]
        // #[should_panic(expected = "Revert(42)")]
        // async fn panics_when_sender_is_not_nft_owner() {
        //     let (deploy_wallet, seller, _buyer1, _buyer2, sell_asset_id, buy_asset_id, amount_selling, inital_price, reserve_price, time) = setup().await;

        //     let buy_asset_struct = Asset {
        //         contract_id: buy_asset_id,
        //         amount: 0,
        //         nft_id: Option::None()
        //     };

        //     let sell_asset_struct = Asset {
        //         contract_id: sell_asset_id,
        //         amount: amount_selling,
        //         nft_id: Option::None()
        //     };

        //     let _auction_id = init_nft(
        //             &deploy_wallet,
        //             &seller,
        //             sell_asset_id,
        //             amount_selling,
        //             buy_asset_id,
        //             inital_price,
        //             reserve_price,
        //             time,
        //             buy_asset_struct,
        //             sell_asset_struct
        //         ).await;
        // }
    }
}


mod bid {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn bids_inital_price() {
            let (
                deploy_wallet,
                seller,
                buyer1,
                _buyer2,
                sell_asset_id,
                buy_asset_id,
                amount_selling,
                inital_price,
                reserve_price,
                time,
            ) = setup().await;

            let buy_asset_struct: Asset = token_asset(buy_asset_id, 0).await;
            let sell_asset_struct: Asset = token_asset(sell_asset_id, amount_selling).await;

            let auction_id = init_token(
                &deploy_wallet,
                &seller,
                sell_asset_id,
                amount_selling,
                buy_asset_id,
                inital_price,
                reserve_price,
                time,
                buy_asset_struct,
                sell_asset_struct,
            )
            .await;

            let bid_asset_struct: Asset = token_asset(buy_asset_id, inital_price).await;

            bid_tokens(
                &buyer1,
                &buyer1,
                auction_id,
                buy_asset_id,
                inital_price,
                bid_asset_struct,
            )
            .await;

            // Uncomment when https://github.com/FuelLabs/fuels-rs/issues/421 is resolved
            // assert_eq!(highest_bidder(&deploy_wallet, auction_id).await, buyer1.wallet.address);
            // assert_eq!(current_bid(&deploy_wallet, auction_id).await, inital_price);
            // Uncomment when https://github.com/FuelLabs/fuels-rs/issues/420 is resolved
            // assert_eq!(deposits(&buyer1, auction_id), inital_price);
        }

        #[tokio::test]
        async fn bids_over_inital_price() {
            let (
                deploy_wallet,
                seller,
                buyer1,
                _buyer2,
                sell_asset_id,
                buy_asset_id,
                amount_selling,
                inital_price,
                reserve_price,
                time,
            ) = setup().await;

            let buy_asset_struct: Asset = token_asset(buy_asset_id, 0).await;
            let sell_asset_struct: Asset = token_asset(sell_asset_id, amount_selling).await;

            let auction_id = init_token(
                &deploy_wallet,
                &seller,
                sell_asset_id,
                amount_selling,
                buy_asset_id,
                inital_price,
                reserve_price,
                time,
                buy_asset_struct,
                sell_asset_struct,
            )
            .await;

            let bid_asset_struct: Asset = token_asset(buy_asset_id, inital_price + 1).await;
            bid_tokens(
                &buyer1,
                &buyer1,
                auction_id,
                buy_asset_id,
                inital_price + 1,
                bid_asset_struct,
            )
            .await;

            // Uncomment when https://github.com/FuelLabs/fuels-rs/issues/421 is resolved
            // assert_eq!(highest_bidder(&deploy_wallet, auction_id).await, buyer1.wallet.address);
            // assert_eq!(
            //     current_bid(&deploy_wallet, auction_id).await,
            //     inital_price + 1
            // );
            // Uncomment when https://github.com/FuelLabs/fuels-rs/issues/420 is resolved
            // assert_eq!(deposits(&buyer1, auction_id), inital_price + 1);
        }

        #[tokio::test]
        async fn bid_overtaken_by_different_bidder() {
            let (
                deploy_wallet,
                seller,
                buyer1,
                buyer2,
                sell_asset_id,
                buy_asset_id,
                amount_selling,
                inital_price,
                reserve_price,
                time,
            ) = setup().await;

            let buy_asset_struct: Asset = token_asset(buy_asset_id, 0).await;
            let sell_asset_struct: Asset = token_asset(sell_asset_id, amount_selling).await;

            let auction_id = init_token(
                &deploy_wallet,
                &seller,
                sell_asset_id,
                amount_selling,
                buy_asset_id,
                inital_price,
                reserve_price,
                time,
                buy_asset_struct,
                sell_asset_struct,
            )
            .await;

            let bid_asset_struct: Asset = token_asset(buy_asset_id, inital_price).await;
            bid_tokens(
                &buyer1,
                &buyer1,
                auction_id,
                buy_asset_id,
                inital_price,
                bid_asset_struct,
            )
            .await;

            let bid_asset_struct: Asset = token_asset(buy_asset_id, inital_price + 1).await;
            bid_tokens(
                &buyer1,
                &buyer2,
                auction_id,
                buy_asset_id,
                inital_price + 1,
                bid_asset_struct,
            )
            .await;

            // Uncomment when https://github.com/FuelLabs/fuels-rs/issues/421 is resolved
            // assert_eq!(highest_bidder(&deploy_wallet, auction_id).await, buyer2.wallet.address);
            // assert_eq!(
            //     current_bid(&deploy_wallet, auction_id).await,
            //     inital_price + 1
            // );
            // Uncomment when https://github.com/FuelLabs/fuels-rs/issues/420 is resolved
            // assert_eq!(deposits(&buyer1, auction_id), inital_price);
            // assert_eq!(deposits(&buyer2, auction_id), inital_price + 1);
        }

        #[tokio::test]
        async fn bid_overtaken_by_original_bidder() {
            let (
                deploy_wallet,
                seller,
                buyer1,
                buyer2,
                sell_asset_id,
                buy_asset_id,
                amount_selling,
                inital_price,
                reserve_price,
                time,
            ) = setup().await;

            let buy_asset_struct: Asset = token_asset(buy_asset_id, 0).await;
            let sell_asset_struct: Asset = token_asset(sell_asset_id, amount_selling).await;

            let auction_id = init_token(
                &deploy_wallet,
                &seller,
                sell_asset_id,
                amount_selling,
                buy_asset_id,
                inital_price,
                reserve_price,
                time,
                buy_asset_struct,
                sell_asset_struct,
            )
            .await;

            let bid_asset_struct: Asset = token_asset(buy_asset_id, inital_price).await;
            bid_tokens(
                &buyer1,
                &buyer1,
                auction_id,
                buy_asset_id,
                inital_price,
                bid_asset_struct,
            )
            .await;
            
            let bid_asset_struct: Asset = token_asset(buy_asset_id, inital_price + inital_price).await;
            bid_tokens(
                &buyer2,
                &buyer2,
                auction_id,
                buy_asset_id,
                inital_price + inital_price,
                bid_asset_struct,
            )
            .await;

            let bid_asset_struct: Asset = token_asset(buy_asset_id, inital_price + 1).await;
            bid_tokens(
                &buyer1,
                &buyer1,
                auction_id,
                buy_asset_id,
                inital_price + 1,
                bid_asset_struct,
            )
            .await;

            // Uncomment when https://github.com/FuelLabs/fuels-rs/issues/421 is resolved
            // assert_eq!(highest_bidder(&deploy_wallet, auction_id).await, buyer1.wallet.address);
            // assert_eq!(
            //     current_bid(&deploy_wallet, auction_id).await,
            //     inital_price + inital_price + 1
            // );
            // Uncomment when https://github.com/FuelLabs/fuels-rs/issues/420 is resolved
            // assert_eq!(deposits(&buyer1, auction_id), inital_price + inital_price + 1);
            // assert_eq!(deposits(&buyer2, auction_id), inital_price + inital_price);
        }

        #[tokio::test]
        async fn bid_is_at_reserve() {
            let (
                deploy_wallet,
                seller,
                buyer1,
                _buyer2,
                sell_asset_id,
                buy_asset_id,
                amount_selling,
                inital_price,
                reserve_price,
                time,
            ) = setup().await;

            let buy_asset_struct: Asset = token_asset(buy_asset_id, 0).await;
            let sell_asset_struct: Asset = token_asset(sell_asset_id, amount_selling).await;

            let auction_id = init_token(
                &deploy_wallet,
                &seller,
                sell_asset_id,
                amount_selling,
                buy_asset_id,
                inital_price,
                reserve_price,
                time,
                buy_asset_struct,
                sell_asset_struct,
            )
            .await;

            let bid_asset_struct: Asset = token_asset(buy_asset_id, reserve_price).await;
            bid_tokens(
                &buyer1,
                &buyer1,
                auction_id,
                buy_asset_id,
                reserve_price,
                bid_asset_struct,
            )
            .await;

            // Uncomment when https://github.com/FuelLabs/fuels-rs/issues/421 is resolved
            // assert_eq!(highest_bidder(&deploy_wallet, auction_id).await, buyer1.wallet.address);
            // assert_eq!(current_bid(&deploy_wallet, auction_id).await, reserve_price);
            // Uncomment when https://github.com/FuelLabs/fuels-rs/issues/420 is resolved
            // assert_eq!(deposits(&buyer1, auction_id).await, 0);
            // assert_eq!(state(&deploy_wallet, auction_id).await, 2);
        }
    }

    mod reverts {

        use super::*;

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_not_intialized() {
            let (
                _deploy_wallet,
                _seller,
                buyer1,
                _buyer2,
                _sell_asset_id,
                buy_asset_id,
                _sell_amount,
                inital_price,
                _reserve_price,
                _time,
            ) = setup().await;

            let bid_asset_struct: Asset = token_asset(buy_asset_id, inital_price).await;
            bid_tokens(
                &buyer1,
                &buyer1,
                0,
                buy_asset_id,
                inital_price,
                bid_asset_struct,
            )
            .await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_over_time() {
            let (
                deploy_wallet,
                seller,
                buyer1,
                _buyer2,
                sell_asset_id,
                buy_asset_id,
                amount_selling,
                inital_price,
                reserve_price,
                _time,
            ) = setup().await;

            let buy_asset_struct: Asset = token_asset(buy_asset_id, 0).await;
            let sell_asset_struct: Asset = token_asset(sell_asset_id, amount_selling).await;

            let auction_id = init_token(
                &deploy_wallet,
                &seller,
                sell_asset_id,
                amount_selling,
                buy_asset_id,
                inital_price,
                reserve_price,
                1,
                buy_asset_struct,
                sell_asset_struct,
            )
            .await;

            let bid_asset_struct: Asset = token_asset(buy_asset_id, inital_price).await;
            bid_tokens(
                &buyer1,
                &buyer1,
                auction_id,
                buy_asset_id,
                inital_price,
                bid_asset_struct,
            )
            .await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_auction_is_over() {
            let (
                deploy_wallet,
                seller,
                buyer1,
                buyer2,
                sell_asset_id,
                buy_asset_id,
                amount_selling,
                inital_price,
                reserve_price,
                time,
            ) = setup().await;

            let buy_asset_struct: Asset = token_asset(buy_asset_id, 0).await;
            let sell_asset_struct: Asset = token_asset(sell_asset_id, amount_selling).await;

            let auction_id = init_token(
                &deploy_wallet,
                &seller,
                sell_asset_id,
                amount_selling,
                buy_asset_id,
                inital_price,
                reserve_price,
                time,
                buy_asset_struct,
                sell_asset_struct,
            )
            .await;

            let bid_asset_struct: Asset = token_asset(buy_asset_id, reserve_price).await;
            bid_tokens(
                &buyer1,
                &buyer1,
                auction_id,
                buy_asset_id,
                reserve_price,
                bid_asset_struct,
            )
            .await;

            let bid_asset_struct: Asset = token_asset(buy_asset_id, reserve_price + 1).await;
            bid_tokens(
                &buyer2,
                &buyer2,
                auction_id,
                buy_asset_id,
                reserve_price + 1,
                bid_asset_struct,
            )
            .await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_bidder_is_seller() {
            let (
                deploy_wallet,
                seller,
                buyer1,
                _buyer2,
                sell_asset_id,
                buy_asset_id,
                amount_selling,
                inital_price,
                reserve_price,
                time,
            ) = setup().await;

            let buy_asset_struct: Asset = token_asset(buy_asset_id, 0).await;
            let sell_asset_struct: Asset = token_asset(sell_asset_id, amount_selling).await;

            let auction_id = init_token(
                &deploy_wallet,
                &seller,
                sell_asset_id,
                amount_selling,
                buy_asset_id,
                inital_price,
                reserve_price,
                time,
                buy_asset_struct,
                sell_asset_struct,
            )
            .await;

            let bid_asset_struct: Asset = token_asset(buy_asset_id, inital_price).await;
            bid_tokens(
                &buyer1,
                &seller,
                auction_id,
                buy_asset_id,
                inital_price,
                bid_asset_struct,
            )
            .await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_sending_incorrect_token() {
            let (
                deploy_wallet,
                seller,
                buyer1,
                _buyer2,
                sell_asset_id,
                buy_asset_id,
                amount_selling,
                inital_price,
                reserve_price,
                time,
            ) = setup().await;

            let buy_asset_struct: Asset = token_asset(buy_asset_id, 0).await;
            let sell_asset_struct: Asset = token_asset(sell_asset_id, amount_selling).await;

            let auction_id = init_token(
                &deploy_wallet,
                &seller,
                sell_asset_id,
                amount_selling,
                buy_asset_id,
                inital_price,
                reserve_price,
                time,
                buy_asset_struct,
                sell_asset_struct,
            )
            .await;

            let bid_asset_struct: Asset = token_asset(sell_asset_id, inital_price).await;
            bid_tokens(
                &seller,
                &buyer1,
                auction_id,
                sell_asset_id,
                inital_price,
                bid_asset_struct,
            )
            .await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_sending_token_struct_mismatch() {
            let (
                deploy_wallet,
                seller,
                buyer1,
                _buyer2,
                sell_asset_id,
                buy_asset_id,
                amount_selling,
                inital_price,
                _reserve_price,
                time,
            ) = setup().await;

            let buy_asset_struct: Asset = token_asset(buy_asset_id, 0).await;
            let sell_asset_struct: Asset = token_asset(sell_asset_id, amount_selling).await;

            let auction_id = init_token(
                &deploy_wallet,
                &seller,
                sell_asset_id,
                amount_selling,
                buy_asset_id,
                inital_price,
                inital_price,
                time,
                buy_asset_struct,
                sell_asset_struct,
            )
            .await;

            let bid_asset_struct: Asset = token_asset(sell_asset_id, inital_price).await;
            bid_tokens(
                &buyer1,
                &buyer1,
                auction_id,
                buy_asset_id,
                inital_price,
                bid_asset_struct,
            )
            .await;
        }

        // TODO: Implement NFT
        // #[tokio::test]
        // #[should_panic(expected = "Revert(42)")]
        // async fn panics_when_sending_incorrect_nft() {
        //     let (deploy_wallet, seller, buyer1, _buyer2, sell_asset_id, buy_asset_id, amount_selling, inital_price, reserve_price, time) = setup().await;

        // let buy_asset_struct: Asset = nft_asset(buy_asset_id, 0).await;
        // let sell_asset_struct: Asset = nft_asset(sell_asset_id, amount_selling).await;

        //     let auction_id = init_token(
        //             &deploy_wallet,
        //             &seller,
        //             sell_asset_id,
        //             amount_selling,
        //             buy_asset_id,
        //             inital_price,
        //             reserve_price,
        //             time,
        //             buy_asset_struct,
        //             sell_asset_struct
        //         ).await;

        //     let buy_asset_struct: Asset = nft_asset(buy_asset_id, inital_price).await;

        //     bid_nft(&seller, &buyer1, auction_id, sell_asset_id, inital_price, bid_asset_struct).await;
        // }

        // #[tokio::test]
        // #[should_panic(expected = "Revert(42)")]
        // async fn panics_when_bidding_nft_not_approved() {
        //     let (deploy_wallet, seller, buyer1, _buyer2, sell_asset_id, buy_asset_id, amount_selling, inital_price, reserve_price, time) = setup().await;

        // let buy_asset_struct: Asset = nft_asset(buy_asset_id, 0).await;
        // let sell_asset_struct: Asset = nft_asset(sell_asset_id, amount_selling).await;

        //     let auction_id = init_token(
        //             &deploy_wallet,
        //             &seller,
        //             sell_asset_id,
        //             amount_selling,
        //             buy_asset_id,
        //             inital_price,
        //             inital_price,
        //             time,
        //             buy_asset_struct,
        //             sell_asset_struct
        //         ).await;

        //     let bid_asset_struct = Asset {
        //         contract_id: sell_asset_id,
        //         amount: inital_price,
        //         nft_id: Option::None()
        //     };
        //     bid_nft(&seller, &buyer1, auction_id, buy_asset_id, inital_price, bid_asset_struct).await;
        // }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_less_then_inital_price() {
            let (
                deploy_wallet,
                seller,
                buyer1,
                _buyer2,
                sell_asset_id,
                buy_asset_id,
                amount_selling,
                inital_price,
                reserve_price,
                time,
            ) = setup().await;

            let buy_asset_struct: Asset = token_asset(buy_asset_id, 0).await;
            let sell_asset_struct: Asset = token_asset(sell_asset_id, amount_selling).await;

            let auction_id = init_token(
                &deploy_wallet,
                &seller,
                sell_asset_id,
                amount_selling,
                buy_asset_id,
                inital_price,
                reserve_price,
                time,
                buy_asset_struct,
                sell_asset_struct,
            )
            .await;

            let bid_asset_struct: Asset = token_asset(buy_asset_id, inital_price - 1).await;
            bid_tokens(
                &buyer1,
                &buyer1,
                auction_id,
                buy_asset_id,
                inital_price - 1,
                bid_asset_struct,
            )
            .await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_second_bid_less_then_current_bid() {
            let (
                deploy_wallet,
                seller,
                buyer1,
                buyer2,
                sell_asset_id,
                buy_asset_id,
                amount_selling,
                inital_price,
                reserve_price,
                time,
            ) = setup().await;

            let buy_asset_struct: Asset = token_asset(buy_asset_id, 0).await;
            let sell_asset_struct: Asset = token_asset(sell_asset_id, amount_selling).await;

            let auction_id = init_token(
                &deploy_wallet,
                &seller,
                sell_asset_id,
                amount_selling,
                buy_asset_id,
                inital_price,
                reserve_price,
                time,
                buy_asset_struct,
                sell_asset_struct,
            )
            .await;

            let bid_asset_struct: Asset = token_asset(buy_asset_id, inital_price + 1).await;
            bid_tokens(
                &buyer1,
                &buyer1,
                auction_id,
                buy_asset_id,
                inital_price + 1,
                bid_asset_struct,
            )
            .await;
            
            let bid_asset_struct: Asset = token_asset(buy_asset_id, inital_price).await;
            bid_tokens(
                &buyer1,
                &buyer2,
                auction_id,
                buy_asset_id,
                inital_price,
                bid_asset_struct,
            )
            .await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn bid_is_over_reserve() {
            let (
                deploy_wallet,
                seller,
                buyer1,
                _buyer2,
                sell_asset_id,
                buy_asset_id,
                amount_selling,
                inital_price,
                reserve_price,
                time,
            ) = setup().await;

            let buy_asset_struct: Asset = token_asset(buy_asset_id, 0).await;
            let sell_asset_struct: Asset = token_asset(sell_asset_id, amount_selling).await;

            let auction_id = init_token(
                &deploy_wallet,
                &seller,
                sell_asset_id,
                amount_selling,
                buy_asset_id,
                inital_price,
                reserve_price,
                time,
                buy_asset_struct,
                sell_asset_struct,
            )
            .await;

            let bid_asset_struct: Asset = token_asset(buy_asset_id, reserve_price + 1).await;
            bid_tokens(
                &buyer1,
                &buyer1,
                auction_id,
                buy_asset_id,
                reserve_price + 1,
                bid_asset_struct,
            )
            .await;

            // Uncomment when https://github.com/FuelLabs/fuels-rs/issues/421 is resolved
            // assert_eq!(highest_bidder(&deploy_wallet, auction_id).await, buyer1.wallet.address);
            // assert_eq!(current_bid(&deploy_wallet, auction_id).await, reserve_price);
            // Uncomment when https://github.com/FuelLabs/fuels-rs/issues/420 is resolved
            //assert_eq!(deposits(&buyer1, auction_id).await, 0);
            // assert_eq!(state(&deploy_wallet, auction_id).await, 2);
        }
    }
}

mod withdraw {

    use super::*;

    mod succes {

        use super::*;

        #[tokio::test]
        async fn withdraws_for_buyer_token() {
            let (
                deploy_wallet,
                seller,
                buyer1,
                _buyer2,
                sell_asset_id,
                buy_asset_id,
                amount_selling,
                inital_price,
                reserve_price,
                _time,
            ) = setup().await;

            let buy_asset_struct: Asset = token_asset(buy_asset_id, 0).await;
            let sell_asset_struct: Asset = token_asset(sell_asset_id, amount_selling).await;

            let auction_id = init_token(
                &deploy_wallet,
                &seller,
                sell_asset_id,
                amount_selling,
                buy_asset_id,
                inital_price,
                reserve_price,
                3,
                buy_asset_struct,
                sell_asset_struct,
            )
            .await;

            let bid_asset_struct: Asset = token_asset(buy_asset_id, inital_price).await;
            bid_tokens(
                &buyer1,
                &buyer1,
                auction_id,
                buy_asset_id,
                inital_price,
                bid_asset_struct,
            )
            .await;

            // TODO: Speed up time

            withdraw(&buyer1, auction_id).await;

            // TODO: Ensure the buyer has the seller assets

            // Uncomment when https://github.com/FuelLabs/fuels-rs/issues/420 is resolved
            // assert_eq!(deposits(&buyer1, auction_id), 0);
            // assert_eq!(state(&deploy_wallet, auction_id).await, 2);
        }

        // TODO: Implement NFT
        // #[tokio::test]
        // async fn withdraws_for_buyer_nft() {
        //     let (deploy_wallet, seller, buyer1, _buyer2, sell_asset_id, buy_asset_id, amount_selling, inital_price, reserve_price, _time) = setup().await;

        //     let buy_asset_struct = Asset {
        //         contract_id: buy_asset_id,
        //         amount: 0,
        //         nft_id: Option::None()
        //     };

        //     let sell_asset_struct = Asset {
        //         contract_id: sell_asset_id,
        //         amount: amount_selling,
        //         nft_id: Option::None()
        //     };

        //     let auction_id = init_nft(
        //             &deploy_wallet,
        //             &seller,
        //             sell_asset_id,
        //             amount_selling,
        //             buy_asset_id,
        //             inital_price,
        //             reserve_price,
        //             3,
        //             buy_asset_struct,
        //             sell_asset_struct
        //         ).await;

        //     let bid_asset_struct = Asset {
        //         contract_id: buy_asset_id,
        //         amount: inital_price,
        //         nft_id: Option::None()
        //     };
        //     bid_nft(&buyer1, &buyer1, auction_id, buy_asset_id, inital_price, bid_asset_struct).await;

        //     // TODO: Speed up time

        //     withdraw(&buyer1, auction_id).await;

        //     // TODO: Ensure the buyer has the seller assets

        //     // Uncomment when https://github.com/FuelLabs/fuels-rs/issues/420 is resolved
        //     //assert_eq!(deposits(&buyer1, auction_id), 0);
        //     assert_eq!(state(&deploy_wallet, auction_id).await, 2);
        // }

        #[tokio::test]
        async fn withdraws_for_seller_token() {
            let (
                deploy_wallet,
                seller,
                buyer1,
                _buyer2,
                sell_asset_id,
                buy_asset_id,
                amount_selling,
                inital_price,
                reserve_price,
                _time,
            ) = setup().await;

            let buy_asset_struct: Asset = token_asset(buy_asset_id, 0).await;
            let sell_asset_struct: Asset = token_asset(sell_asset_id, amount_selling).await;

            let auction_id = init_token(
                &deploy_wallet,
                &seller,
                sell_asset_id,
                amount_selling,
                buy_asset_id,
                inital_price,
                reserve_price,
                3,
                buy_asset_struct,
                sell_asset_struct,
            )
            .await;

            let bid_asset_struct: Asset = token_asset(buy_asset_id, inital_price).await;
            bid_tokens(
                &buyer1,
                &buyer1,
                auction_id,
                buy_asset_id,
                inital_price,
                bid_asset_struct,
            )
            .await;

            // TODO: Speed up time

            withdraw(&seller, auction_id).await;
            // TODO: Ensure the seller has the buyer assets
            // assert_eq!(state(&deploy_wallet, auction_id).await, 2);
            // Uncomment when https://github.com/FuelLabs/fuels-rs/issues/420 is resolved
            // assert_eq!(deposits(&seller, auction_id), 0);
        }

        // TODO: Implement NFT
        // #[tokio::test]
        // async fn withdraws_for_seller_nft() {
        //     let (deploy_wallet, seller, buyer1, _buyer2, sell_asset_id, buy_asset_id, amount_selling, inital_price, reserve_price, _time) = setup().await;

        //     let buy_asset_struct = Asset {
        //         contract_id: buy_asset_id,
        //         amount: 0,
        //         nft_id: Option::None()
        //     };

        //     let sell_asset_struct = Asset {
        //         contract_id: sell_asset_id,
        //         amount: amount_selling,
        //         nft_id: Option::None()
        //     };

        //     let auction_id = init_nft(
        //             &deploy_wallet,
        //             &seller,
        //             sell_asset_id,
        //             amount_selling,
        //             buy_asset_id,
        //             inital_price,
        //             reserve_price,
        //             3,
        //             buy_asset_struct,
        //             sell_asset_struct
        //         ).await;

        //     let bid_asset_struct = Asset {
        //         contract_id: buy_asset_id,
        //         amount: inital_price,
        //         nft_id: Option::None()
        //     };
        //     bid_nft(&buyer1, &buyer1, auction_id, buy_asset_id, inital_price, bid_asset_struct).await;

        //     // TODO: Speed up time

        //     withdraw(&seller, auction_id).await;
        //     // TODO: Ensure the seller has the buyer assets
        //     assert_eq!(state(&deploy_wallet, auction_id).await, 2);
        //     // Uncomment when https://github.com/FuelLabs/fuels-rs/issues/420 is resolved
        //     //assert_eq!(deposits(&seller, auction_id), 0);
        // }

        #[tokio::test]
        async fn withdraws_for_failed_bids_token() {
            let (
                deploy_wallet,
                seller,
                buyer1,
                buyer2,
                sell_asset_id,
                buy_asset_id,
                amount_selling,
                inital_price,
                reserve_price,
                _time,
            ) = setup().await;

            let buy_asset_struct: Asset = token_asset(buy_asset_id, 0).await;
            let sell_asset_struct: Asset = token_asset(sell_asset_id, amount_selling).await;

            let auction_id = init_token(
                &deploy_wallet,
                &seller,
                sell_asset_id,
                amount_selling,
                buy_asset_id,
                inital_price,
                reserve_price,
                4,
                buy_asset_struct,
                sell_asset_struct,
            )
            .await;

            let bid_asset_struct: Asset = token_asset(buy_asset_id, inital_price).await;
            bid_tokens(
                &buyer1,
                &buyer1,
                auction_id,
                buy_asset_id,
                inital_price,
                bid_asset_struct,
            )
            .await;

            let bid_asset_struct: Asset = token_asset(buy_asset_id, inital_price + 1).await;
            bid_tokens(
                &buyer1,
                &buyer2,
                auction_id,
                buy_asset_id,
                inital_price + 1,
                bid_asset_struct,
            )
            .await;

            // TODO: Speed up time

            withdraw(&buyer1, auction_id).await;

            // TODO: Ensure the failed buyer has the inital price of assests again

            // Uncomment when https://github.com/FuelLabs/fuels-rs/issues/420 is resolved
            // assert_eq!(deposits(&buyer1, auction_id), 0);
            // assert_eq!(state(&deploy_wallet, auction_id).await, 2);
        }

        // TODO: Implement NFT
        // #[tokio::test]
        // async fn withdraws_for_failed_bids_nft() {
        //     let (deploy_wallet, seller, buyer1, buyer2, sell_asset_id, buy_asset_id, amount_selling, inital_price, reserve_price, _time) = setup().await;

        //     let buy_asset_struct = Asset {
        //         contract_id: buy_asset_id,
        //         amount: 0,
        //         nft_id: Option::None()
        //     };

        //     let sell_asset_struct = Asset {
        //         contract_id: sell_asset_id,
        //         amount: amount_selling,
        //         nft_id: Option::None()
        //     };

        //     let auction_id = init_nft(
        //             &deploy_wallet,
        //             &seller,
        //             sell_asset_id,
        //             amount_selling,
        //             buy_asset_id,
        //             inital_price,
        //             reserve_price,
        //             3,
        //             buy_asset_struct,
        //             sell_asset_struct
        //         ).await;

        //     let bid_asset_struct = Asset {
        //         contract_id: buy_asset_id,
        //         amount: inital_price,
        //         nft_id: Option::None()
        //     };
        //     bid_nft(&buyer1, &buyer1, auction_id, buy_asset_id, inital_price, bid_asset_struct).await;

        //     let bid_asset_struct = Asset {
        //         contract_id: buy_asset_id,
        //         amount: inital_price + 1,
        //         nft_id: Option::None()
        //     };
        //     bid_nft(&buyer1, &buyer2, auction_id, buy_asset_id, inital_price + 1, bid_asset_struct).await;

        //     // TODO: Speed up time

        //     withdraw(&buyer1, auction_id).await;

        //     // TODO: Ensure the failed buyer has the inital price of assests again

        //     // Uncomment when https://github.com/FuelLabs/fuels-rs/issues/420 is resolved
        //     //assert_eq!(deposits(&buyer1, auction_id), 0);
        //     assert_eq!(state(&deploy_wallet, auction_id).await, 2);
        // }
    }

    mod reverts {

        use super::*;

        #[tokio::test]
        #[should_panic]
        async fn panics_when_not_initalized() {
            let (
                _deploy_wallet,
                _seller,
                buyer1,
                _buyer2,
                _sell_asset_id,
                _buy_asset_id,
                _amount_selling,
                _inital_price,
                _reserve_price,
                _time,
            ) = setup().await;

            withdraw(&buyer1, 0).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_not_over_time() {
            let (
                deploy_wallet,
                seller,
                buyer1,
                _buyer2,
                sell_asset_id,
                buy_asset_id,
                amount_selling,
                inital_price,
                reserve_price,
                time,
            ) = setup().await;

            let buy_asset_struct: Asset = token_asset(buy_asset_id, 0).await;
            let sell_asset_struct: Asset = token_asset(sell_asset_id, amount_selling).await;

            let auction_id = init_token(
                &deploy_wallet,
                &seller,
                sell_asset_id,
                amount_selling,
                buy_asset_id,
                inital_price,
                reserve_price,
                time,
                buy_asset_struct,
                sell_asset_struct,
            )
            .await;

            let bid_asset_struct: Asset = token_asset(buy_asset_id, inital_price).await;
            bid_tokens(
                &buyer1,
                &buyer1,
                auction_id,
                buy_asset_id,
                inital_price,
                bid_asset_struct,
            )
            .await;

            withdraw(&buyer1, auction_id).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_buyer_already_withdrawn() {
            let (
                deploy_wallet,
                seller,
                buyer1,
                _buyer2,
                sell_asset_id,
                buy_asset_id,
                amount_selling,
                inital_price,
                reserve_price,
                _time,
            ) = setup().await;

            let buy_asset_struct: Asset = token_asset(buy_asset_id, 0).await;
            let sell_asset_struct: Asset = token_asset(sell_asset_id, amount_selling).await;

            let auction_id = init_token(
                &deploy_wallet,
                &seller,
                sell_asset_id,
                amount_selling,
                buy_asset_id,
                inital_price,
                reserve_price,
                3,
                buy_asset_struct,
                sell_asset_struct,
            )
            .await;

            let bid_asset_struct: Asset = token_asset(buy_asset_id, inital_price).await;
            bid_tokens(
                &buyer1,
                &buyer1,
                auction_id,
                buy_asset_id,
                inital_price,
                bid_asset_struct,
            )
            .await;

            // TODO: Speed up time

            withdraw(&buyer1, auction_id).await;
            withdraw(&buyer1, auction_id).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_seller_already_withdrawn() {
            let (
                deploy_wallet,
                seller,
                buyer1,
                _buyer2,
                sell_asset_id,
                buy_asset_id,
                amount_selling,
                inital_price,
                reserve_price,
                _time,
            ) = setup().await;

            let buy_asset_struct: Asset = token_asset(buy_asset_id, 0).await;
            let sell_asset_struct: Asset = token_asset(sell_asset_id, amount_selling).await;

            let auction_id = init_token(
                &deploy_wallet,
                &seller,
                sell_asset_id,
                amount_selling,
                buy_asset_id,
                inital_price,
                reserve_price,
                3,
                buy_asset_struct,
                sell_asset_struct,
            )
            .await;

            let bid_asset_struct: Asset = token_asset(buy_asset_id, inital_price).await;
            bid_tokens(
                &buyer1,
                &buyer1,
                auction_id,
                buy_asset_id,
                inital_price,
                bid_asset_struct,
            )
            .await;

            // TODO: Speed up time

            withdraw(&seller, auction_id).await;
            withdraw(&seller, auction_id).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_failed_bid_withdrawn_twice() {
            let (
                deploy_wallet,
                seller,
                buyer1,
                buyer2,
                sell_asset_id,
                buy_asset_id,
                amount_selling,
                inital_price,
                reserve_price,
                _time,
            ) = setup().await;

            let buy_asset_struct: Asset = token_asset(buy_asset_id, 0).await;
            let sell_asset_struct: Asset = token_asset(sell_asset_id, amount_selling).await;

            let auction_id = init_token(
                &deploy_wallet,
                &seller,
                sell_asset_id,
                amount_selling,
                buy_asset_id,
                inital_price,
                reserve_price,
                3,
                buy_asset_struct,
                sell_asset_struct,
            )
            .await;

            let bid_asset_struct: Asset = token_asset(buy_asset_id, inital_price).await;
            bid_tokens(
                &buyer1,
                &buyer1,
                auction_id,
                buy_asset_id,
                inital_price,
                bid_asset_struct,
            )
            .await;

            // TODO: Speed up time

            withdraw(&buyer1, auction_id).await;
            withdraw(&buyer1, auction_id).await;
        }
    }
}
/*
mod auction_end_block {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn gets_end_block() {
            let (
                deploy_wallet,
                seller,
                _buyer1,
                _buyer2,
                sell_asset_id,
                buy_asset_id,
                amount_selling,
                inital_price,
                reserve_price,
                time,
            ) = setup().await;

            let buy_asset_struct = Asset {
                contract_id: buy_asset_id,
                amount: 0,
                nft_id: Option::None(),
            };

            let sell_asset_struct = Asset {
                contract_id: sell_asset_id,
                amount: amount_selling,
                nft_id: Option::None(),
            };

            let auction_id = init_token(
                &deploy_wallet,
                &seller,
                sell_asset_id,
                amount_selling,
                buy_asset_id,
                inital_price,
                reserve_price,
                time,
                buy_asset_struct,
                sell_asset_struct,
            )
            .await;

            // TODO: This really shouldn't be hard coded. Need to add a get block function
            let block = 4;

            assert_eq!(
                auction_end_block(&deploy_wallet, auction_id).await,
                time + block
            );
        }
    }

    mod reverts {

        use super::*;

        #[tokio::test]
        #[should_panic]
        async fn panics_when_not_initalized() {
            let (
                deploy_wallet,
                _seller,
                _buyer1,
                _buyer2,
                _sell_asset_id,
                _buy_asset_id,
                _sell_amount,
                _inital_price,
                _reserve_price,
                time,
            ) = setup().await;

            let block = 0;

            assert_eq!(auction_end_block(&deploy_wallet, 0).await, time + block);
        }
    }
}

mod current_bid {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn gets_current_bid() {
            let (
                deploy_wallet,
                seller,
                buyer1,
                _buyer2,
                sell_asset_id,
                buy_asset_id,
                amount_selling,
                inital_price,
                reserve_price,
                time,
            ) = setup().await;

            let buy_asset_struct = Asset {
                contract_id: buy_asset_id,
                amount: 0,
                nft_id: Option::None(),
            };

            let sell_asset_struct = Asset {
                contract_id: sell_asset_id,
                amount: amount_selling,
                nft_id: Option::None(),
            };

            let auction_id = init_token(
                &deploy_wallet,
                &seller,
                sell_asset_id,
                amount_selling,
                buy_asset_id,
                inital_price,
                reserve_price,
                time,
                buy_asset_struct,
                sell_asset_struct,
            )
            .await;

            assert_eq!(current_bid(&deploy_wallet, auction_id).await, 0);

            let bid_asset_struct = Asset {
                contract_id: buy_asset_id,
                amount: inital_price,
                nft_id: Option::None(),
            };
            bid_tokens(
                &buyer1,
                &buyer1,
                auction_id,
                buy_asset_id,
                inital_price,
                bid_asset_struct,
            )
            .await;

            assert_eq!(current_bid(&deploy_wallet, auction_id).await, inital_price);
        }
    }
}

// Uncomment when https://github.com/FuelLabs/fuels-rs/issues/420 is resolved
// mod deposits {

//     use super::*;

//     mod succes {

//         use super::*;
//         #[tokio::test]
//         async fn gets_deposit() {
//             let (deploy_wallet, seller, buyer1, _buyer2, sell_asset_id, buy_asset_id, sell_amount, inital_price, reserve_price, time) = setup().await;

//             let buy_asset_struct = Asset {
//                 contract_id: buy_asset_id,
//                 amount: 0,
//                 nft_id: Option::None()
//             };

//             let sell_asset_struct = Asset {
//                 contract_id: sell_asset_id,
//                 amount: amount_selling,
//                 nft_id: Option::None()
//             };

//             let auction_id = init_token(
//                     &deploy_wallet,
//                     &seller,
//                     sell_asset_id,
//                     amount_selling,
//                     buy_asset_id,
//                     inital_price,
//                     reserve_price,
//                     time,
//                     buy_asset_struct,
//                     sell_asset_struct
//                 ).await;

//             assert_eq!(deposits(&buyer1, auction_id).await, 0);

//             let bid_asset_struct = Asset {
//                 contract_id: buy_asset_id,
//                 amount: inital_price,
//                 nft_id: Option::None()
//             };
//             bid_tokens(&buyer1, &buyer1, auction_id, buy_asset_id, inital_price, bid_asset_struct).await;

//             assert_eq!(deposits(&buyer1, auction_id).await, inital_price);
//         }
//     }
// }

// Uncomment when https://github.com/FuelLabs/fuels-rs/issues/421 is resolved
// mod highest_bidder {

//     use super::*;

//     mod succes {

//         use super::*;

//         #[tokio::test]
//         async fn get_highest_bidder() {
//             let (deploy_wallet, seller, buyer1, buyer2, sell_asset_id, buy_asset_id, amount_selling, inital_price, reserve_price, time) = setup().await;

//             let buy_asset_struct = Asset {
//                 contract_id: buy_asset_id,
//                 amount: 0,
//                 nft_id: Option::None()
//             };

//             let sell_asset_struct = Asset {
//                 contract_id: sell_asset_id,
//                 amount: amount_selling,
//                 nft_id: Option::None()
//             };

//             let auction_id = init_token(
//                     &deploy_wallet,
//                     &seller,
//                     sell_asset_id,
//                     amount_selling,
//                     buy_asset_id,
//                     inital_price,
//                     reserve_price,
//                     time,
//                     buy_asset_struct,
//                     sell_asset_struct
//                 ).await;

//             assert_eq!(highest_bidder(&deploy_wallet, auction_id).await, Option::None());

//             let bid_asset_struct = Asset {
//                 contract_id: buy_asset_id,
//                 amount: inital_price,
//                 nft_id: Option::None()
//             };
//             bid_tokens(&buyer1, &buyer1, auction_id, buy_asset_id, inital_price, bid_asset_struct).await;

//             assert_eq!(highest_bidder(&deploy_wallet, auction_id).await, buyer1.wallet.address());
//         }
//     }

//     mod reverts {

//         use super::*;

//         #[tokio::test]
//         #[should_panic(expected = "Revert(42)")]
//         async fn panics_when_not_initalized() {
//             let (deploy_wallet, _seller, buyer1, _buyer2, _sell_asset_id, _buy_asset_id, _sell_amount, _inital_price, _reserve_price, _time) = setup().await;

//             assert_eq!(highest_bidder(&deploy_wallet, auction_id).await, Option::None());
//         }
//     }
// }

// Uncomment when https://github.com/FuelLabs/fuels-rs/issues/421 is resolved
// mod reserve {

//     use super::*;

//     mod success {

//         use super::*;

//         #[tokio::test]
//         async fn gets_reserve() {
//             let (deploy_wallet, seller, _buyer1, _buyer2, sell_asset_id, buy_asset_id, amount_selling, inital_price, reserve_price, time) = setup().await;

//             let buy_asset_struct = Asset {
//                 contract_id: buy_asset_id,
//                 amount: 0,
//                 nft_id: Option::None()
//             };

//             let sell_asset_struct = Asset {
//                 contract_id: sell_asset_id,
//                 amount: amount_selling,
//                 nft_id: Option::None()
//             };

//             let auction_id = init_token(
//                     &deploy_wallet,
//                     &seller,
//                     sell_asset_id,
//                     amount_selling,
//                     buy_asset_id,
//                     inital_price,
//                     reserve_price,
//                     time,
//                     buy_asset_struct,
//                     sell_asset_struct
//                 ).await;

//             assert_eq!(reserve(&deploy_wallet, auction_id).await, reserve_price);
//         }
//     }

//     mod reverts {

//         use super::*;

//         #[tokio::test]
//         #[should_panic(expected = "Revert(42)")]
//         async fn panics_when_not_initalized() {
//             let (deploy_wallet, _seller, _buyer1, _buyer2, _sell_asset_id, _buy_asset_id, _sell_amount, _inital_price, reserve_price, _time) = setup().await;

//             assert_eq!(reserve(&deploy_wallet, auction_id).await, reserve_price);
//         }
//     }
// }

mod state {

    use super::*;

    mod succes {

        use super::*;
        #[tokio::test]
        async fn gets_state() {
            let (
                deploy_wallet,
                seller,
                _buyer1,
                _buyer2,
                sell_asset_id,
                buy_asset_id,
                amount_selling,
                inital_price,
                reserve_price,
                time,
            ) = setup().await;

            let buy_asset_struct = Asset {
                contract_id: buy_asset_id,
                amount: 0,
                nft_id: Option::None(),
            };

            let sell_asset_struct = Asset {
                contract_id: sell_asset_id,
                amount: amount_selling,
                nft_id: Option::None(),
            };

            let auction_id = init_token(
                &deploy_wallet,
                &seller,
                sell_asset_id,
                amount_selling,
                buy_asset_id,
                inital_price,
                reserve_price,
                time,
                buy_asset_struct,
                sell_asset_struct,
            )
            .await;

            assert_eq!(state(&deploy_wallet, auction_id).await, 1);
        }
    }
}

mod sell_amount {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn gets_sell_amount() {
            let (
                deploy_wallet,
                seller,
                _buyer1,
                _buyer2,
                sell_asset_id,
                buy_asset_id,
                amount_selling,
                inital_price,
                reserve_price,
                time,
            ) = setup().await;

            let buy_asset_struct = Asset {
                contract_id: buy_asset_id,
                amount: 0,
                nft_id: Option::None(),
            };

            let sell_asset_struct = Asset {
                contract_id: sell_asset_id,
                amount: amount_selling,
                nft_id: Option::None(),
            };

            let auction_id = init_token(
                &deploy_wallet,
                &seller,
                sell_asset_id,
                amount_selling,
                buy_asset_id,
                inital_price,
                reserve_price,
                time,
                buy_asset_struct,
                sell_asset_struct,
            )
            .await;

            assert_eq!(
                sell_amount(&deploy_wallet, auction_id).await,
                amount_selling
            );
        }
    }
}

mod sell_asset {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn gets_sell_asset() {
            let (
                deploy_wallet,
                seller,
                _buyer1,
                _buyer2,
                sell_asset_id,
                buy_asset_id,
                amount_selling,
                inital_price,
                reserve_price,
                time,
            ) = setup().await;

            let buy_asset_struct = Asset {
                contract_id: buy_asset_id,
                amount: 0,
                nft_id: Option::None(),
            };

            let sell_asset_struct = Asset {
                contract_id: sell_asset_id,
                amount: amount_selling,
                nft_id: Option::None(),
            };

            let auction_id = init_token(
                &deploy_wallet,
                &seller,
                sell_asset_id,
                amount_selling,
                buy_asset_id,
                inital_price,
                reserve_price,
                time,
                buy_asset_struct,
                sell_asset_struct,
            )
            .await;

            assert_eq!(sell_asset(&deploy_wallet, auction_id).await, sell_asset_id);
        }
    }
}*/

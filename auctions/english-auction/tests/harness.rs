mod utils;

use utils::{
    abi_calls::{
        init_token,
        init_nft,
        bid_tokens,
        bid_nft,
        buy_reserve_tokens,
        buy_reserve_nft,
        withdraw,
        auction_end_block,
        current_bid,
        // Uncomment when https://github.com/FuelLabs/fuels-rs/issues/420 is resolved
        // deposits,
        // Uncomment when https://github.com/FuelLabs/fuels-rs/issues/421 is resolved
        //highest_bidder,
        // reserve,
        sell_amount,
        sell_asset,
        state
    },
    test_helpers::setup,
    Asset,
    Option
};

mod asset_test {

    use super::*;

    #[tokio::test]
    #[should_panic]
    async fn assets_different() {
        let (_deploy_wallet, _seller, _buyer1, _buyer2, sell_asset_id, buy_asset_id, _sell_amount, _inital_price, _reserve_price, _time) = setup().await;

        assert_eq!(
            sell_asset_id,
            buy_asset_id
        );
    }
}

mod constructor {

    use super::*;

    mod success {

        use super::*;

        #[tokio::test]
        async fn inits_token() {
            let (deploy_wallet, seller, _buyer1, _buyer2, sell_asset_id, buy_asset_id, amount_selling, inital_price, reserve_price, time) = setup().await;

            let buy_asset_struct = Asset {
                contract_id: buy_asset_id,
                amount: 0,
                nft_id: Option::None()
            };
        
            let sell_asset_struct = Asset {
                contract_id: sell_asset_id,
                amount: amount_selling,
                nft_id: Option::None()
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
                    sell_asset_struct
                ).await;

            assert_eq!(state(&deploy_wallet, auction_id).await, 1);
            assert_eq!(current_bid(&deploy_wallet, auction_id).await, 0);
            // TODO: Get the current block to test what block the auction should end
            // assert_eq!(auction_end_block(&deploy_wallet, auction_it).await, 0);
            assert_eq!(sell_amount(&deploy_wallet, auction_id).await, amount_selling);
            assert_eq!(sell_asset(&deploy_wallet, auction_id).await, sell_asset_id);
            // assert_eq!(reserve(&deploy_wallet, auction_id).await, reserve_price);
        }

        // TODO: Impliment NFT testing
        // #[tokio::test]
        // async fn inits_nft_with_approval() {
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
            let (deploy_wallet, seller, _buyer1, _buyer2, sell_asset_id, buy_asset_id, sell_amount, inital_price, reserve_price, time) = setup().await;

            let buy_asset_struct = Asset {
                contract_id: buy_asset_id,
                amount: 0,
                nft_id: Option::None()
            };
        
            let sell_asset_struct = Asset {
                contract_id: sell_asset_id,
                amount: sell_amount,
                nft_id: Option::None()
            };

            let auction_id = init_token(
                    &deploy_wallet, 
                    &seller, 
                    sell_asset_id, 
                    0, 
                    buy_asset_id, 
                    inital_price, 
                    reserve_price, 
                    time, 
                    buy_asset_struct, 
                    sell_asset_struct
                ).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_inital_price_higher_than_reserve() {
            let (deploy_wallet, seller, _buyer1, _buyer2, sell_asset_id, buy_asset_id, sell_amount, _inital_price, _reserve_price, time) = setup().await;

            let buy_asset_struct = Asset {
                contract_id: buy_asset_id,
                amount: 0,
                nft_id: Option::None()
            };
        
            let sell_asset_struct = Asset {
                contract_id: sell_asset_id,
                amount: sell_amount,
                nft_id: Option::None()
            };


            let auction_id = init_token(
                    &deploy_wallet, 
                    &seller, 
                    sell_asset_id, 
                    sell_amount, 
                    buy_asset_id, 
                    2, 
                    1, 
                    time, 
                    buy_asset_struct, 
                    sell_asset_struct
                ).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_time_for_auction_is_zero() {
            let (deploy_wallet, seller, _buyer1, _buyer2, sell_asset_id, buy_asset_id, sell_amount, inital_price, reserve_price, _time) = setup().await;

            let buy_asset_struct = Asset {
                contract_id: buy_asset_id,
                amount: 0,
                nft_id: Option::None()
            };
        
            let sell_asset_struct = Asset {
                contract_id: sell_asset_id,
                amount: 0,
                nft_id: Option::None()
            };

            let auction_id = init_token(
                    &deploy_wallet, 
                    &seller, 
                    sell_asset_id, 
                    sell_amount, 
                    buy_asset_id, 
                    inital_price, 
                    reserve_price, 
                    0,
                    buy_asset_struct,
                    sell_asset_struct
                ).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_token_amount_doesnt_match_struct() {
            let (deploy_wallet, seller, _buyer1, _buyer2, sell_asset_id, buy_asset_id, sell_amount, inital_price, reserve_price, time) = setup().await;

            let buy_asset_struct = Asset {
                contract_id: buy_asset_id,
                amount: 0,
                nft_id: Option::None()
            };
        
            let sell_asset_struct = Asset {
                contract_id: sell_asset_id,
                amount: 0,
                nft_id: Option::None()
            };
        
            let auction_id = init_token(
                    &deploy_wallet, 
                    &seller, 
                    sell_asset_id, 
                    sell_amount, 
                    buy_asset_id, 
                    inital_price, 
                    reserve_price, 
                    time, 
                    buy_asset_struct, 
                    sell_asset_struct
                ).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Revert(42)")]
        async fn panics_when_token_asset_doesnt_match_struct() {
            let (deploy_wallet, seller, _buyer1, _buyer2, sell_asset_id, buy_asset_id, sell_amount, inital_price, reserve_price, time) = setup().await;

            let buy_asset_struct = Asset {
                contract_id: buy_asset_id,
                amount: 0,
                nft_id: Option::None()
            };
        
            let sell_asset_struct = Asset {
                contract_id: buy_asset_id,
                amount: sell_amount,
                nft_id: Option::None()
            };
        
            let auction_id = init_token(
                    &deploy_wallet, 
                    &seller, 
                    sell_asset_id, 
                    sell_amount, 
                    buy_asset_id, 
                    inital_price, 
                    reserve_price, 
                    time, 
                    buy_asset_struct, 
                    sell_asset_struct
                ).await;
        }

        // TODO: Implement NFT
        // #[tokio::test]
        // #[should_panic(expected = "Revert(42)")]
        // async fn panics_when_nft_transfer_not_approved() {
        //     let (deploy_wallet, seller, _buyer1, _buyer2, sell_asset_id, buy_asset_id, sell_amount, inital_price, reserve_price, time) = setup().await;

        //     let buy_asset_struct = Asset {
        //         contract_id: buy_asset_id,
        //         amount: 0,
        //         nft_id: Option::None()
        //     };
        
        //     let sell_asset_struct = Asset {
        //         contract_id: sell_asset_id,
        //         amount: sell_amount,
        //         nft_id: Option::None()
        //     };
        
        //     let auction_id = init_nft(
        //             &deploy_wallet, 
        //             &seller, 
        //             sell_asset_id, 
        //             sell_amount, 
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
        //     let (deploy_wallet, seller, _buyer1, _buyer2, sell_asset_id, buy_asset_id, sell_amount, inital_price, reserve_price, time) = setup().await;

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

        //     let auction_id = init_nft(
        //             &deploy_wallet, 
        //             &seller, 
        //             sell_asset_id, 
        //             sell_amount, 
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
        //     let (deploy_wallet, seller, _buyer1, _buyer2, sell_asset_id, buy_asset_id, sell_amount, inital_price, reserve_price, time) = setup().await;

        //     deploy_funds(&deploy_wallet, &seller.wallet, 100).await;

        //     let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        //     let call_params = CallParameters::new(Some(sell_amount), Some(AssetId::from(*sell_asset_id)), None);

        //     let buy_asset_struct = Asset {
        //         contract_id: buy_asset_id,
        //         amount: 0,
        //         nft_id: Option::None()
        //     };

        //     let sell_asset_struct = Asset {
        //         contract_id: buy_asset_id,
        //         amount: sell_amount,
        //         nft_id: Option::None()
        //     };

        //     let auction_id = init_nft(
        //             &deploy_wallet, 
        //             &seller, 
        //             sell_asset_id, 
        //             sell_amount, 
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
        //     let (deploy_wallet, seller, _buyer1, _buyer2, sell_asset_id, buy_asset_id, sell_amount, inital_price, reserve_price, time) = setup().await;

        //     let buy_asset_struct = Asset {
        //         contract_id: buy_asset_id,
        //         amount: 0,
        //         nft_id: Option::None()
        //     };

        //     let sell_asset_struct = Asset {
        //         contract_id: sell_asset_id,
        //         amount: sell_amount,
        //         nft_id: Option::None()
        //     };

        //     let auction_id = init_nft(
        //             &deploy_wallet, 
        //             &seller, 
        //             sell_asset_id, 
        //             sell_amount, 
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

/*
mod bid {

    use super::*;

    #[tokio::test]
    async fn bids_inital_price() {
        let (deploy_wallet, seller, buyer1, _buyer2, sell_asset_id, buy_asset_id, sell_amount, inital_price, reserve_price, time) = setup().await;

        init(&deploy_wallet,
            &seller,
            sell_asset_id,
            sell_amount,
            buy_asset_id,
            inital_price,
            reserve_price,
            time
        )
        .await;

        deploy_funds(&buyer1, &buyer1.wallet, 100).await;
    
        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(inital_price), Some(AssetId::from(*buy_asset_id)));

        assert!(
            buyer1
                .auction
                .bid()
                .tx_params(tx_params)
                .call_params(call_params)
                .call()
                .await
                .unwrap()
                .value
        );

        // Uncommment when https://github.com/FuelLabs/fuels-rs/issues/375 is resolved
        // assert_eq!(
        //     buyer1.auction.highest_bidder().call().await.unwrap().value,
        //     buyer1.wallet.address
        // );

        assert_eq!(
            buyer1.auction.current_bid().call().await.unwrap().value,
            inital_price
        );

        assert_eq!(
            buyer1
                .auction
                .deposits(englishauction_mod::Identity::Address(buyer1.wallet.address()))
                .call()
                .await
                .unwrap()
                .value,
            inital_price
        );
    }

    #[tokio::test]
    async fn bids_over_inital_price() {
        let (deploy_wallet, seller, buyer1, _buyer2, sell_asset_id, buy_asset_id, sell_amount, inital_price, reserve_price, time) = setup().await;

        init(&deploy_wallet,
            &seller,
            sell_asset_id,
            sell_amount,
            buy_asset_id,
            inital_price,
            reserve_price,
            time
        )
        .await;

        deploy_funds(&buyer1, &buyer1.wallet, 100).await;
    
        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(inital_price + 1), Some(AssetId::from(*buy_asset_id)));

        assert!(
            buyer1
                .auction
                .bid()
                .tx_params(tx_params)
                .call_params(call_params)
                .call()
                .await
                .unwrap()
                .value
        );

        // Uncommment when https://github.com/FuelLabs/fuels-rs/issues/375 is resolved
        // assert_eq!(
        //     buyer1.auction.highest_bidder().call().await.unwrap().value,
        //     buyer1.wallet.address
        // );

        assert_eq!(
            buyer1.auction.current_bid().call().await.unwrap().value,
            inital_price + 1
        );

        assert_eq!(
            buyer1
                .auction
                .deposits(englishauction_mod::Identity::Address(buyer1.wallet.address()))
                .call()
                .await
                .unwrap()
                .value,
            inital_price + 1
        );
    }

    #[tokio::test]
    async fn bid_overtaken_by_different_bidder() {
        let (deploy_wallet, seller, buyer1, buyer2, sell_asset_id, buy_asset_id, sell_amount, inital_price, reserve_price, time) = setup().await;

        init(&deploy_wallet,
            &seller,
            sell_asset_id,
            sell_amount,
            buy_asset_id,
            inital_price,
            reserve_price,
            time
        )
        .await;

        deploy_funds(&buyer1, &buyer1.wallet, 100).await;
    
        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(inital_price), Some(AssetId::from(*buy_asset_id)));

        let _bid1 = buyer1
            .auction
            .bid()
            .tx_params(tx_params)
            .call_params(call_params)
            .call()
            .await
            .unwrap()
            .value;

        
        deploy_funds(&buyer1, &buyer2.wallet, 100).await;

        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(inital_price + 1), Some(AssetId::from(*buy_asset_id)));

        assert!(
            buyer2
                .auction
                .bid()
                .tx_params(tx_params)
                .call_params(call_params)
                .call()
                .await
                .unwrap()
                .value
        );

        // Uncommment when https://github.com/FuelLabs/fuels-rs/issues/375 is resolved
        // assert_eq!(
        //     buyer2.auction.highest_bidder().call().await.unwrap().value,
        //     buyer2.wallet.address
        // );

        assert_eq!(
            buyer2.auction.current_bid().call().await.unwrap().value,
            inital_price + 1
        );

        assert_eq!(
            buyer1
                .auction
                .deposits(englishauction_mod::Identity::Address(buyer1.wallet.address()))
                .call()
                .await
                .unwrap()
                .value,
            inital_price
        );

        assert_eq!(
            buyer2
                .auction
                .deposits(englishauction_mod::Identity::Address(buyer2.wallet.address()))
                .call()
                .await
                .unwrap()
                .value,
            inital_price + 1
        );
    }

    #[tokio::test]
    async fn bid_overtaken_by_original_bidder() {
        let (deploy_wallet, seller, buyer1, buyer2, sell_asset_id, buy_asset_id, sell_amount, inital_price, reserve_price, time) = setup().await;

        init(&deploy_wallet,
            &seller,
            sell_asset_id,
            sell_amount,
            buy_asset_id,
            inital_price,
            reserve_price,
            time
        )
        .await;

        deploy_funds(&buyer1, &buyer1.wallet, 100).await;
    
        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(inital_price), Some(AssetId::from(*buy_asset_id)));

        let _bid1 = buyer1
            .auction
            .bid()
            .tx_params(tx_params)
            .call_params(call_params)
            .call()
            .await
            .unwrap()
            .value;

        
        deploy_funds(&buyer1, &buyer2.wallet, 100).await;

        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(inital_price + inital_price), Some(AssetId::from(*buy_asset_id)));

        let _bid2 = buyer2
            .auction
            .bid()
            .tx_params(tx_params)
            .call_params(call_params)
            .call()
            .await
            .unwrap()
            .value;

        // The first bidder already bid the inital price, so bidding the inital price again plus one 
        // should overbid the second bidder with the original deposit
        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(inital_price + 1), Some(AssetId::from(*buy_asset_id)));

        assert!(
            buyer1
                .auction
                .bid()
                .tx_params(tx_params)
                .call_params(call_params)
                .call()
                .await
                .unwrap()
                .value
        );

        // Uncommment when https://github.com/FuelLabs/fuels-rs/issues/375 is resolved
        // assert_eq!(
        //     buyer1.auction.get_highest_bidder().call().await.unwrap().value,
        //     buyer1.wallet.address
        // );

        assert_eq!(
            buyer1.auction.current_bid().call().await.unwrap().value,
            inital_price + inital_price + 1
        );

        assert_eq!(
            buyer2
                .auction
                .deposits(englishauction_mod::Identity::Address(buyer2.wallet.address()))
                .call()
                .await
                .unwrap()
                .value,
            inital_price + inital_price
        );

        assert_eq!(
            buyer1
                .auction
                .deposits(englishauction_mod::Identity::Address(buyer1.wallet.address()))
                .call()
                .await
                .unwrap()
                .value,
            inital_price + inital_price + 1
        );
    }

    #[tokio::test]
    async fn bid_is_at_reserve() {
        let (deploy_wallet, seller, buyer1, _buyer2, sell_asset_id, buy_asset_id, sell_amount, inital_price, reserve_price, time) = setup().await;

        init(&deploy_wallet,
            &seller,
            sell_asset_id,
            sell_amount,
            buy_asset_id,
            inital_price,
            reserve_price,
            time
        )
        .await;

        deploy_funds(&buyer1, &buyer1.wallet, 100).await;
    
        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(reserve_price), Some(AssetId::from(*buy_asset_id)));

        assert!(
            buyer1
                .auction
                .bid()
                .tx_params(tx_params)
                .call_params(call_params)
                .append_variable_outputs(2)
                .call()
                .await
                .unwrap()
                .value
        );

        // Uncommment when https://github.com/FuelLabs/fuels-rs/issues/375 is resolved
        // assert_eq!(
        //     buyer1.auction.highest_bidder().call().await.unwrap().value,
        //     buyer1.wallet.address
        // );

        assert_eq!(
            buyer1.auction.current_bid().call().await.unwrap().value,
            reserve_price
        );

        assert_eq!(
            buyer1
                .auction
                .deposits(englishauction_mod::Identity::Address(buyer1.wallet.address()))
                .call()
                .await
                .unwrap()
                .value,
            0
        );

        assert_eq!(
            buyer1.auction.state().call().await.unwrap().value,
            2
        );
    }

    #[tokio::test]
    async fn bid_is_over_reserve() {
        let (deploy_wallet, seller, buyer1, _buyer2, sell_asset_id, buy_asset_id, sell_amount, inital_price, reserve_price, time) = setup().await;

        init(&deploy_wallet,
            &seller,
            sell_asset_id,
            sell_amount,
            buy_asset_id,
            inital_price,
            reserve_price,
            time
        )
        .await;

        deploy_funds(&buyer1, &buyer1.wallet, 100).await;
    
        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(reserve_price + 1), Some(AssetId::from(*buy_asset_id)));

        assert!(
            buyer1
                .auction
                .bid()
                .tx_params(tx_params)
                .call_params(call_params)
                .append_variable_outputs(2)
                .call()
                .await
                .unwrap()
                .value
        );

        // Uncommment when https://github.com/FuelLabs/fuels-rs/issues/375 is resolved
        // assert_eq!(
        //     buyer1.auction.highest_bidder().call().await.unwrap().value,
        //     buyer1.wallet.address
        // );

        assert_eq!(
            buyer1.auction.current_bid().call().await.unwrap().value,
            reserve_price
        );

        assert_eq!(
            buyer1
                .auction
                .deposits(englishauction_mod::Identity::Address(buyer1.wallet.address()))
                .call()
                .await
                .unwrap()
                .value,
            0
        );

        assert_eq!(
            buyer1.auction.state().call().await.unwrap().value,
            2
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_not_intialized() {
        let (_deploy_wallet, _seller, buyer1, _buyer2, _sell_asset_id, buy_asset_id, _sell_amount, inital_price, _reserve_price, _time) = setup().await;

        deploy_funds(&buyer1, &buyer1.wallet, 100).await;
    
        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(inital_price), Some(AssetId::from(*buy_asset_id)));

        assert!(
            buyer1
                .auction
                .bid()
                .tx_params(tx_params)
                .call_params(call_params)
                .call()
                .await
                .unwrap()
                .value
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_over_time() {
        let (deploy_wallet, seller, buyer1, _buyer2, sell_asset_id, buy_asset_id, sell_amount, inital_price, reserve_price, _time) = setup().await;

        init(&deploy_wallet,
            &seller,
            sell_asset_id,
            sell_amount,
            buy_asset_id,
            inital_price,
            reserve_price,
            1
        )
        .await;

        deploy_funds(&buyer1, &buyer1.wallet, 100).await;
    
        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(inital_price), Some(AssetId::from(*buy_asset_id)));

        assert!(
            buyer1
                .auction
                .bid()
                .tx_params(tx_params)
                .call_params(call_params)
                .call()
                .await
                .unwrap()
                .value
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_sending_incorrect_asset() {
        let (deploy_wallet, seller, buyer1, _buyer2, sell_asset_id, buy_asset_id, sell_amount, inital_price, reserve_price, time) = setup().await;

        init(&deploy_wallet,
            &seller,
            sell_asset_id,
            sell_amount,
            buy_asset_id,
            inital_price,
            reserve_price,
            time
        )
        .await;

        deploy_funds(&deploy_wallet, &buyer1.wallet, 100).await;
    
        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(inital_price), Some(AssetId::from(*sell_asset_id)));

        assert!(
            buyer1
                .auction
                .bid()
                .tx_params(tx_params)
                .call_params(call_params)
                .call()
                .await
                .unwrap()
                .value
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_less_then_inital_price() {
        let (deploy_wallet, seller, buyer1, _buyer2, sell_asset_id, buy_asset_id, sell_amount, inital_price, reserve_price, time) = setup().await;

        init(&deploy_wallet,
            &seller,
            sell_asset_id,
            sell_amount,
            sell_asset_id,
            inital_price,
            reserve_price,
            time
        )
        .await;

        deploy_funds(&buyer1, &buyer1.wallet, 100).await;
    
        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(inital_price - 1), Some(AssetId::from(*buy_asset_id)));

        assert!(
            buyer1
                .auction
                .bid()
                .tx_params(tx_params)
                .call_params(call_params)
                .call()
                .await
                .unwrap()
                .value
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_bidder_is_seller() {
        let (deploy_wallet, seller, buyer1, _buyer2, sell_asset_id, buy_asset_id, sell_amount, inital_price, reserve_price, time) = setup().await;

        init(&deploy_wallet,
            &seller,
            sell_asset_id,
            sell_amount,
            buy_asset_id,
            inital_price,
            reserve_price,
            time
        )
        .await;

        deploy_funds(&buyer1, &seller.wallet, 100).await;
    
        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(inital_price), Some(AssetId::from(*buy_asset_id)));

        assert!(
            seller
                .auction
                .bid()
                .tx_params(tx_params)
                .call_params(call_params)
                .call()
                .await
                .unwrap()
                .value
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_second_bid_less_then_current_bid() {
        let (deploy_wallet, seller, buyer1, buyer2, sell_asset_id, buy_asset_id, sell_amount, inital_price, reserve_price, time) = setup().await;

        init(&deploy_wallet,
            &seller,
            sell_asset_id,
            sell_amount,
            sell_asset_id,
            inital_price,
            reserve_price,
            time
        )
        .await;

        deploy_funds(&buyer1, &buyer1.wallet, 100).await;
    
        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(inital_price), Some(AssetId::from(*buy_asset_id)));

        let _bid1 = buyer1
            .auction
            .bid()
            .tx_params(tx_params)
            .call_params(call_params)
            .call()
            .await
            .unwrap()
            .value;

        deploy_funds(&buyer1, &buyer2.wallet, 100).await;

        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(inital_price + inital_price + 1), Some(AssetId::from(*buy_asset_id)));

        let _bid2 = buyer2
            .auction
            .bid()
            .tx_params(tx_params)
            .call_params(call_params)
            .call()
            .await
            .unwrap()
            .value;

        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(inital_price - 1), Some(AssetId::from(*buy_asset_id)));

        assert!(
            buyer1
                .auction
                .bid()
                .tx_params(tx_params)
                .call_params(call_params)
                .call()
                .await
                .unwrap()
                .value
        );

        // Uncommment when https://github.com/FuelLabs/fuels-rs/issues/375 is resolved
        // assert_eq!(
        //     buyer2.auction.get_highest_bidder().call().await.unwrap().value,
        //     buyer2.wallet.address
        // );
    }
}

mod buy_reserve {

    use super::*;

    #[tokio::test]
    async fn buys_reserve() {
        let (deploy_wallet, seller, buyer1, _buyer2, sell_asset_id, buy_asset_id, sell_amount, inital_price, reserve_price, time) = setup().await;

        init(&deploy_wallet,
            &seller,
            sell_asset_id,
            sell_amount,
            buy_asset_id,
            inital_price,
            reserve_price,
            time
        )
        .await;

        deploy_funds(&buyer1, &buyer1.wallet, 100).await;

        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(reserve_price), Some(AssetId::from(*buy_asset_id)));

        assert!(
            buyer1
                .auction
                .buy_reserve()
                .tx_params(tx_params)
                .call_params(call_params)
                .append_variable_outputs(2)
                .call()
                .await
                .unwrap()
                .value
        );

        // Uncommment when https://github.com/FuelLabs/fuels-rs/issues/375 is resolved
        // assert_eq!(
        //     buyer1.auction.get_highest_bidder().call().await.unwrap().value,
        //     buyer1.wallet.address
        // );

        assert_eq!(
            buyer1
                .auction
                .deposits(englishauction_mod::Identity::Address(buyer1.wallet.address()))
                .call()
                .await
                .unwrap()
                .value,
            0
        );

        assert_eq!(
            buyer1.auction.current_bid().call().await.unwrap().value,
            reserve_price
        );

        assert_eq!(
            buyer1.auction.state().call().await.unwrap().value,
            2
        );
    }

    #[tokio::test]
    async fn buys_after_bid() {
        let (deploy_wallet, seller, buyer1, _buyer2, sell_asset_id, buy_asset_id, sell_amount, inital_price, reserve_price, time) = setup().await;

        init(&deploy_wallet,
            &seller,
            sell_asset_id,
            sell_amount,
            buy_asset_id,
            inital_price,
            reserve_price,
            time
        )
        .await;

        deploy_funds(&buyer1, &buyer1.wallet, 100).await;

        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(inital_price), Some(AssetId::from(*buy_asset_id)));

        let _bid1 = buyer1
            .auction
            .bid()
            .tx_params(tx_params)
            .call_params(call_params)
            .call()
            .await
            .unwrap()
            .value;
    
        let remaining_deposit = reserve_price - inital_price;
    
        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(remaining_deposit), Some(AssetId::from(*buy_asset_id)));

        assert!(
            buyer1
                .auction
                .buy_reserve()
                .tx_params(tx_params)
                .call_params(call_params)
                .append_variable_outputs(2)
                .call()
                .await
                .unwrap()
                .value
        );

        // Uncommment when https://github.com/FuelLabs/fuels-rs/issues/375 is resolved
        // assert_eq!(
        //     buyer1.auction.get_highest_bidder().call().await.unwrap().value,
        //     buyer1.wallet.address
        // );

        assert_eq!(
            buyer1
                .auction
                .deposits(englishauction_mod::Identity::Address(buyer1.wallet.address()))
                .call()
                .await
                .unwrap()
                .value,
            0
        );

        assert_eq!(
            buyer1.auction.current_bid().call().await.unwrap().value,
            reserve_price
        );

        assert_eq!(
            buyer1.auction.state().call().await.unwrap().value,
            2
        );
    }

    #[tokio::test]
    async fn buys_over_reserve() {
        let (deploy_wallet, seller, buyer1, _buyer2, sell_asset_id, buy_asset_id, sell_amount, inital_price, reserve_price, time) = setup().await;

        init(&deploy_wallet,
            &seller,
            sell_asset_id,
            sell_amount,
            buy_asset_id,
            inital_price,
            reserve_price,
            time
        )
        .await;

        deploy_funds(&buyer1, &buyer1.wallet, 100).await;

        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(reserve_price + 1), Some(AssetId::from(*buy_asset_id)));

        assert!(
            buyer1
                .auction
                .buy_reserve()
                .tx_params(tx_params)
                .call_params(call_params)
                .append_variable_outputs(2)
                .call()
                .await
                .unwrap()
                .value
        );

        // Uncommment when https://github.com/FuelLabs/fuels-rs/issues/375 is resolved
        // assert_eq!(
        //     buyer1.auction.get_highest_bidder().call().await.unwrap().value,
        //     buyer1.wallet.address
        // );

        assert_eq!(
            buyer1
                .auction
                .deposits(englishauction_mod::Identity::Address(buyer1.wallet.address()))
                .call()
                .await
                .unwrap()
                .value,
            0
        );

        assert_eq!(
            buyer1.auction.current_bid().call().await.unwrap().value,
            reserve_price
        );

        assert_eq!(
            buyer1.auction.state().call().await.unwrap().value,
            2
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_not_initalized() {
        let (_deploy_wallet, _seller, buyer1, _buyer2, _sell_asset_id, buy_asset_id, _sell_amount, _inital_price, reserve_price, _time) = setup().await;

        deploy_funds(&buyer1, &buyer1.wallet, 100).await;
    
        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(reserve_price), Some(AssetId::from(*buy_asset_id)));

        assert!(
            buyer1
                .auction
                .buy_reserve()
                .tx_params(tx_params)
                .call_params(call_params)
                .append_variable_outputs(2)
                .call()
                .await
                .unwrap()
                .value
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_over_time() {
        let (deploy_wallet, seller, buyer1, _buyer2, sell_asset_id, buy_asset_id, sell_amount, inital_price, reserve_price, _time) = setup().await;

        init(&deploy_wallet,
            &seller,
            sell_asset_id,
            sell_amount,
            buy_asset_id,
            inital_price,
            reserve_price,
            1
        )
        .await;

        deploy_funds(&buyer1, &buyer1.wallet, 100).await;
    
        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(reserve_price), Some(AssetId::from(*buy_asset_id)));

        assert!(
            buyer1
                .auction
                .buy_reserve()
                .tx_params(tx_params)
                .call_params(call_params)
                .append_variable_outputs(2)
                .call()
                .await
                .unwrap()
                .value
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_no_reserve_set() {
        let (deploy_wallet, seller, buyer1, _buyer2, sell_asset_id, buy_asset_id, sell_amount, inital_price, reserve_price, time) = setup().await;

        init(&deploy_wallet,
            &seller,
            sell_asset_id,
            sell_amount,
            buy_asset_id,
            inital_price,
            0,
            time
        )
        .await;

        deploy_funds(&buyer1, &buyer1.wallet, 100).await;
    
        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(reserve_price), Some(AssetId::from(*buy_asset_id)));

        assert!(
            buyer1
                .auction
                .buy_reserve()
                .tx_params(tx_params)
                .call_params(call_params)
                .append_variable_outputs(2)
                .call()
                .await
                .unwrap()
                .value
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_bidder_is_seller() {
        let (deploy_wallet, seller, buyer1, _buyer2, sell_asset_id, buy_asset_id, sell_amount, inital_price, reserve_price, time) = setup().await;

        init(&deploy_wallet,
            &seller,
            sell_asset_id,
            sell_amount,
            buy_asset_id,
            inital_price,
            reserve_price,
            time
        )
        .await;

        deploy_funds(&buyer1, &seller.wallet, 100).await;
    
        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(inital_price), Some(AssetId::from(*buy_asset_id)));

        assert!(
            seller
                .auction
                .buy_reserve()
                .tx_params(tx_params)
                .call_params(call_params)
                .append_variable_outputs(2)
                .call()
                .await
                .unwrap()
                .value
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_reserve_not_met() {
        let (deploy_wallet, seller, buyer1, _buyer2, sell_asset_id, buy_asset_id, sell_amount, inital_price, reserve_price, time) = setup().await;

        init(&deploy_wallet,
            &seller,
            sell_asset_id,
            sell_amount,
            buy_asset_id,
            inital_price,
            reserve_price,
            time
        )
        .await;

        deploy_funds(&buyer1, &buyer1.wallet, 100).await;
    
        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(reserve_price - 1), Some(AssetId::from(*buy_asset_id)));

        assert!(
            buyer1
                .auction
                .buy_reserve()
                .tx_params(tx_params)
                .call_params(call_params)
                .append_variable_outputs(2)
                .call()
                .await
                .unwrap()
                .value
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_sending_incorrect_asset() {
        let (deploy_wallet, seller, buyer1, _buyer2, sell_asset_id, buy_asset_id, sell_amount, inital_price, reserve_price, time) = setup().await;

        init(&deploy_wallet,
            &seller,
            sell_asset_id,
            sell_amount,
            buy_asset_id,
            inital_price,
            reserve_price,
            time
        )
        .await;

        deploy_funds(&deploy_wallet, &buyer1.wallet, 100).await;
    
        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(reserve_price), Some(AssetId::from(*sell_asset_id)));

        assert!(
            buyer1
                .auction
                .buy_reserve()
                .tx_params(tx_params)
                .call_params(call_params)
                .append_variable_outputs(2)
                .call()
                .await
                .unwrap()
                .value
        );
    }
}

mod withdraw {

    use super::*;

    #[tokio::test]
    async fn withdraws_for_buyer() {
        let (deploy_wallet, seller, buyer1, _buyer2, sell_asset_id, buy_asset_id, sell_amount, inital_price, reserve_price, _time) = setup().await;

        init(&deploy_wallet,
            &seller,
            sell_asset_id,
            sell_amount,
            buy_asset_id,
            inital_price,
            reserve_price,
            3
        )
        .await;

        deploy_funds(&buyer1, &buyer1.wallet, 100).await;

        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(inital_price), Some(AssetId::from(*buy_asset_id)));

        let _bid1 = buyer1
            .auction
            .bid()
            .tx_params(tx_params)
            .call_params(call_params)
            .call()
            .await
            .unwrap()
            .value;

        // TODO: Speed up time

        assert!(
            buyer1
                .auction
                .withdraw()
                .append_variable_outputs(2)
                .call()
                .await
                .unwrap()
                .value
        );

        // TODO: Ensure the buyer has the seller assets

        assert_eq!(
            buyer1
                .auction
                .deposits(englishauction_mod::Identity::Address(buyer1.wallet.address()))
                .call()
                .await
                .unwrap()
                .value,
            0
        );

        assert_eq!(
            buyer1.auction.state().call().await.unwrap().value,
            2
        );
    }

    #[tokio::test]
    async fn withdraws_for_seller() {
        let (deploy_wallet, seller, buyer1, _buyer2, sell_asset_id, buy_asset_id, sell_amount, inital_price, reserve_price, _time) = setup().await;

        init(&deploy_wallet,
            &seller,
            sell_asset_id,
            sell_amount,
            buy_asset_id,
            inital_price,
            reserve_price,
            3
        )
        .await;

        deploy_funds(&buyer1, &buyer1.wallet, 100).await;

        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(inital_price), Some(AssetId::from(*buy_asset_id)));

        let _bid1 = buyer1
            .auction
            .bid()
            .tx_params(tx_params)
            .call_params(call_params)
            .call()
            .await
            .unwrap()
            .value;

        // TODO: Speed up time

        assert!(
            seller
                .auction
                .withdraw()
                .append_variable_outputs(2)
                .call()
                .await
                .unwrap()
                .value
        );

        // TODO: Ensure the seller has the buyer assets

        assert_eq!(
            buyer1.auction.state().call().await.unwrap().value,
            2
        );
    }

    #[tokio::test]
    async fn withdraws_for_failed_bids() {
        let (deploy_wallet, seller, buyer1, buyer2, sell_asset_id, buy_asset_id, sell_amount, inital_price, reserve_price, _time) = setup().await;

        init(&deploy_wallet,
            &seller,
            sell_asset_id,
            sell_amount,
            buy_asset_id,
            inital_price,
            reserve_price,
            5
        )
        .await;

        deploy_funds(&buyer1, &buyer1.wallet, 100).await;

        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(inital_price), Some(AssetId::from(*buy_asset_id)));

        let _bid1 = buyer1
            .auction
            .bid()
            .tx_params(tx_params)
            .call_params(call_params)
            .call()
            .await
            .unwrap()
            .value;

        deploy_funds(&buyer1, &buyer2.wallet, 100).await;

        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(inital_price + 1), Some(AssetId::from(*buy_asset_id)));

        let _bid2 = buyer2
            .auction
            .bid()
            .tx_params(tx_params)
            .call_params(call_params)
            .call()
            .await
            .unwrap()
            .value;

        // TODO: Speed up time

        assert!(
            buyer1
                .auction
                .withdraw()
                .append_variable_outputs(2)
                .call()
                .await
                .unwrap()
                .value
        );

        // TODO: Ensure the failed buyer has the inital price of assests again

        assert_eq!(
            buyer1
                .auction
                .deposits(englishauction_mod::Identity::Address(buyer1.wallet.address()))
                .call()
                .await
                .unwrap()
                .value,
            0
        );

        assert_eq!(
            buyer1.auction.state().call().await.unwrap().value,
            2
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_not_initalized() {
        let (_deploy_wallet, _seller, buyer1, _buyer2, _sell_asset_id, _buy_asset_id, _sell_amount, _inital_price, _reserve_price, _time) = setup().await;

        assert!(
            buyer1
                .auction
                .withdraw()
                .append_variable_outputs(2)
                .call()
                .await
                .unwrap()
                .value
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_not_over_time() {
        let (deploy_wallet, seller, buyer1, _buyer2, sell_asset_id, buy_asset_id, sell_amount, inital_price, reserve_price, time) = setup().await;

        init(&deploy_wallet,
            &seller,
            sell_asset_id,
            sell_amount,
            buy_asset_id,
            inital_price,
            reserve_price,
            time
        )
        .await;

        deploy_funds(&buyer1, &buyer1.wallet, 100).await;

        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(inital_price), Some(AssetId::from(*buy_asset_id)));

        let _bid1 = buyer1
            .auction
            .bid()
            .tx_params(tx_params)
            .call_params(call_params)
            .call()
            .await
            .unwrap()
            .value;

        assert!(
            buyer1
                .auction
                .withdraw()
                .append_variable_outputs(2)
                .call()
                .await
                .unwrap()
                .value
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_buyer_already_withdrawn() {
        let (deploy_wallet, seller, buyer1, _buyer2, sell_asset_id, buy_asset_id, sell_amount, inital_price, reserve_price, _time) = setup().await;

        init(&deploy_wallet,
            &seller,
            sell_asset_id,
            sell_amount,
            buy_asset_id,
            inital_price,
            reserve_price,
            3
        )
        .await;

        deploy_funds(&buyer1, &buyer1.wallet, 100).await;

        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(inital_price), Some(AssetId::from(*buy_asset_id)));

        let _bid1 = buyer1
            .auction
            .bid()
            .tx_params(tx_params)
            .call_params(call_params)
            .call()
            .await
            .unwrap()
            .value;

        // TODO: Speed up time

        let _widthdraw = buyer1
            .auction
            .withdraw()
            .append_variable_outputs(2)
            .call()
            .await
            .unwrap()
            .value;

        assert!(
            buyer1
                .auction
                .withdraw()
                .append_variable_outputs(2)
                .call()
                .await
                .unwrap()
                .value
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_seller_already_withdrawn() {
        let (deploy_wallet, seller, buyer1, _buyer2, sell_asset_id, buy_asset_id, sell_amount, inital_price, reserve_price, _time) = setup().await;

        init(&deploy_wallet,
            &seller,
            sell_asset_id,
            sell_amount,
            buy_asset_id,
            inital_price,
            reserve_price,
            3
        )
        .await;

        deploy_funds(&buyer1, &buyer1.wallet, 100).await;

        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(inital_price), Some(AssetId::from(*buy_asset_id)));

        let _bid1 = buyer1
            .auction
            .bid()
            .tx_params(tx_params)
            .call_params(call_params)
            .call()
            .await
            .unwrap()
            .value;

        // TODO: Speed up time

        let _withdrawn = seller
            .auction
            .withdraw()
            .append_variable_outputs(2)
            .call()
            .await
            .unwrap()
            .value;

        assert!(
            seller
                .auction
                .withdraw()
                .append_variable_outputs(2)
                .call()
                .await
                .unwrap()
                .value
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_failed_bid_withdrawn_twice() {
        let (deploy_wallet, seller, buyer1, buyer2, sell_asset_id, buy_asset_id, sell_amount, inital_price, reserve_price, _time) = setup().await;

        init(&deploy_wallet,
            &seller,
            sell_asset_id,
            sell_amount,
            buy_asset_id,
            inital_price,
            reserve_price,
            5
        )
        .await;

        deploy_funds(&buyer1, &buyer1.wallet, 100).await;

        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(inital_price), Some(AssetId::from(*buy_asset_id)));

        let _bid1 = buyer1
            .auction
            .bid()
            .tx_params(tx_params)
            .call_params(call_params)
            .call()
            .await
            .unwrap()
            .value;

        deploy_funds(&buyer1, &buyer2.wallet, 100).await;

        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(inital_price + 1), Some(AssetId::from(*buy_asset_id)));

        let _bid2 = buyer2
            .auction
            .bid()
            .tx_params(tx_params)
            .call_params(call_params)
            .call()
            .await
            .unwrap()
            .value;

        // TODO: Speed up time

        let _withdrawn = buyer1
            .auction
            .withdraw()
            .append_variable_outputs(2)
            .call()
            .await
            .unwrap()
            .value;

        assert!(
            buyer1
                .auction
                .withdraw()
                .append_variable_outputs(2)
                .call()
                .await
                .unwrap()
                .value
        );
    }
}


mod auction_end_block {

    use super::*;

    #[tokio::test]
    async fn gets_end_block() {
        let (deploy_wallet, seller, _buyer1, _buyer2, sell_asset_id, buy_asset_id, sell_amount, inital_price, reserve_price, time) = setup().await;

        init(&deploy_wallet,
            &seller,
            sell_asset_id,
            sell_amount,
            buy_asset_id,
            inital_price,
            reserve_price,
            time
        )
        .await;

        // TODO: This really shouldn't be hard coded. Need to add a get block function
        let block = 4;

        assert_eq!(
            deploy_wallet.auction.auction_end_block().call().await.unwrap().value,
            time + block
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_not_initalized() {
        let (_deploy_wallet, seller, _buyer1, _buyer2, _sell_asset_id, _buy_asset_id, _sell_amount, _inital_price, _reserve_price, time) = setup().await;

        let block = 0;

        assert_eq!(
            seller.auction.auction_end_block().call().await.unwrap().value,
            time + block
        );
    }
}

mod current_bid {

    use super::*;

    #[tokio::test]
    async fn gets_current_bid() {
        let (deploy_wallet, seller, buyer1, _buyer2, sell_asset_id, buy_asset_id, sell_amount, inital_price, reserve_price, time) = setup().await;

        init(&deploy_wallet,
            &seller,
            sell_asset_id,
            sell_amount,
            buy_asset_id,
            inital_price,
            reserve_price,
            time
        )
        .await;

        assert_eq!(
            deploy_wallet.auction.current_bid().call().await.unwrap().value,
            0
        );

        deploy_funds(&buyer1, &buyer1.wallet, 100).await;

        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(inital_price), Some(AssetId::from(*buy_asset_id)));

        let _bid1 = buyer1
            .auction
            .bid()
            .tx_params(tx_params)
            .call_params(call_params)
            .call()
            .await
            .unwrap()
            .value;

        assert_eq!(
            buyer1.auction.current_bid().call().await.unwrap().value,
            inital_price
        ); 
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_not_initalized() {
        let (deploy_wallet, _seller, _buyer1, _buyer2, _sell_asset_id, _buy_asset_id, _sell_amount, _inital_price, _reserve_price, _time) = setup().await;

        assert_eq!(
            deploy_wallet.auction.current_bid().call().await.unwrap().value,
            0
        );
    }
}

mod deposits {

    use super::*;

    #[tokio::test]
    async fn gets_deposit() {
        let (deploy_wallet, seller, buyer1, _buyer2, sell_asset_id, buy_asset_id, sell_amount, inital_price, reserve_price, time) = setup().await;

        init(&deploy_wallet,
            &seller,
            sell_asset_id,
            sell_amount,
            buy_asset_id,
            inital_price,
            reserve_price,
            time
        )
        .await;

        assert_eq!(
            buyer1
                .auction
                .deposits(englishauction_mod::Identity::Address(buyer1.wallet.address()))
                .call()
                .await
                .unwrap()
                .value,
            0
        );

        deploy_funds(&buyer1, &buyer1.wallet, 100).await;

        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(inital_price), Some(AssetId::from(*buy_asset_id)));

        let _bid1 = buyer1
            .auction
            .bid()
            .tx_params(tx_params)
            .call_params(call_params)
            .call()
            .await
            .unwrap()
            .value;

        assert_eq!(
            buyer1
                .auction
                .deposits(englishauction_mod::Identity::Address(buyer1.wallet.address()))
                .call()
                .await
                .unwrap()
                .value,
            inital_price
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_not_initalized() {
        let (_deploy_wallet, _seller, buyer1, _buyer2, _sell_asset_id, _buy_asset_id, _sell_amount, _inital_price, _reserve_price, _time) = setup().await;

        assert_eq!(
            buyer1
                .auction
                .deposits(englishauction_mod::Identity::Address(buyer1.wallet.address()))
                .call()
                .await
                .unwrap()
                .value,
            0
        );
    }
}

// Uncomment when https://github.com/FuelLabs/fuels-rs/issues/375 is resolved
// mod highest_bidder {

//     use super::*;

//     #[tokio::test]
//     async fn get_highest_bidder() {
//         let (deploy_wallet, seller, buyer1, buyer2, sell_asset_id, buy_asset_id, sell_amount, inital_price, reserve_price, time) = setup().await;

//         init(&deploy_wallet,
//             &seller,
//             sell_asset_id,
//             sell_amount,
//             buy_asset_id,
//             inital_price,
//             reserve_price,
//             time
//         )
//         .await;

//         deploy_funds(&buyer1, &buyer1.wallet, 100).await;

//         let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
//         let call_params = CallParameters::new(Some(inital_price), Some(AssetId::from(*buy_asset_id)));

//         let _bid1 = buyer1
//             .auction
//             .bid()
//             .tx_params(tx_params)
//             .call_params(call_params)
//             .call()
//             .await
//             .unwrap()
//             .value;

//         assert_eq!(
//             deploy_wallet.auction.highest_bidder().call().await.unwrap().value,
//             buyer1.wallet.address()
//         );

//         deploy_funds(&buyer, &buyer2.wallet, 100).await;

//         let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
//         let call_params = CallParameters::new(Some(inital_price + 1), Some(AssetId::from(*buy_asset_id)));

//         let _bid2 = buyer2
//             .auction
//             .bid()
//             .tx_params(tx_params)
//             .call_params(call_params)
//             .call()
//             .await
//             .unwrap()
//             .value;

//         assert_eq!(
//             deploy_wallet.auction.highest_bidder().call().await.unwrap().value,
//             buyer2.wallet.address()
//         );
//     }

//     #[tokio::test]
//     #[should_panic]
//     async fn panics_when_not_initalized() {
//         let (deploy_wallet, _seller, buyer1, _buyer2, _sell_asset_id, _buy_asset_id, _sell_amount, _inital_price, _reserve_price, _time) = setup().await;

//         assert_eq!(
//             deploy_wallet.auction.highest_bidder().call().await.unwrap().value,
//             buyer1.wallet.address()
//         );
//     }
// }

mod reserve {

    use super::*;

    #[tokio::test]
    async fn gets_reserve() {
        let (deploy_wallet, seller, _buyer1, _buyer2, sell_asset_id, buy_asset_id, sell_amount, inital_price, reserve_price, time) = setup().await;

        init(&deploy_wallet,
            &seller,
            sell_asset_id,
            sell_amount,
            buy_asset_id,
            inital_price,
            reserve_price,
            time
        )
        .await;

        assert_eq!(
            deploy_wallet.auction.reserve().call().await.unwrap().value,
            reserve_price
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_not_initalized() {
        let (deploy_wallet, _seller, _buyer1, _buyer2, _sell_asset_id, _buy_asset_id, _sell_amount, _inital_price, reserve_price, _time) = setup().await;

        assert_eq!(
            deploy_wallet.auction.reserve().call().await.unwrap().value,
            reserve_price
        );
    }
}

mod state {

    use super::*;

    #[tokio::test]
    async fn gets_state() {
        let (deploy_wallet, seller, _buyer1, _buyer2, sell_asset_id, buy_asset_id, sell_amount, inital_price, reserve_price, time) = setup().await;

        assert_eq!(
            deploy_wallet.auction.state().call().await.unwrap().value,
            0
        );

        init(&deploy_wallet,
            &seller,
            sell_asset_id,
            sell_amount,
            buy_asset_id,
            inital_price,
            reserve_price,
            time
        )
        .await;

        assert_eq!(
            deploy_wallet.auction.state().call().await.unwrap().value,
            1
        );
    }
}

mod sell_amount {

    use super::*;

    #[tokio::test]
    async fn gets_sell_amount() {
        let (deploy_wallet, seller, _buyer1, _buyer2, sell_asset_id, buy_asset_id, sell_amount, inital_price, reserve_price, time) = setup().await;

        init(&deploy_wallet,
            &seller,
            sell_asset_id,
            sell_amount,
            buy_asset_id,
            inital_price,
            reserve_price,
            time
        )
        .await;

        assert_eq!(
            deploy_wallet.auction.sell_amount().call().await.unwrap().value,
            sell_amount
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_not_initalized() {
        let (deploy_wallet, _seller, _buyer1, _buyer2, _sell_asset_id, _buy_asset_id, sell_amount, _inital_price, _reserve_price, _time) = setup().await;

        assert_eq!(
            deploy_wallet.auction.sell_amount().call().await.unwrap().value,
            sell_amount
        );
    }
}

mod sell_asset {

    use super::*;

    #[tokio::test]
    async fn gets_sell_asset() {
        let (deploy_wallet, seller, _buyer1, _buyer2, sell_asset_id, buy_asset_id, sell_amount, inital_price, reserve_price, time) = setup().await;

        init(&deploy_wallet,
            &seller,
            sell_asset_id,
            sell_amount,
            buy_asset_id,
            inital_price,
            reserve_price,
            time
        )
        .await;

        assert_eq!(
            deploy_wallet.auction.sell_asset().call().await.unwrap().value,
            sell_asset_id
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_not_initalized() {
        let (deploy_wallet, _seller, _buyer1, _buyer2, sell_asset_id, _buy_asset_id, _sell_amount, _inital_price, _reserve_price, _time) = setup().await;

        assert_eq!(
            deploy_wallet.auction.sell_asset().call().await.unwrap().value,
            sell_asset_id
        );
    }
}*/

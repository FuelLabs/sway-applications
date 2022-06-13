use fuels::{prelude::*, tx::ContractId, tx::Salt};
use fuels_abigen_macro::abigen;

// Load abi from json
abigen!(EnglishAuction, "out/debug/english-auction-abi.json");
abigen!(Asset, "tests/artifacts/asset/out/debug/asset-abi.json");

struct Metadata {
    asset: Option<Asset>,
    auction: EnglishAuction,
    wallet: LocalWallet,
}

async fn setup() -> (Metadata, Metadata, Metadata, Metadata, ContractId, ContractId, u64, u64, u64, u64) {
    // Setup 3 test wallets
    let mut wallets = launch_provider_and_get_wallets(WalletsConfig {
        num_wallets: 4,
        coins_per_wallet: 1,
        coin_amount: 1000000,
    })
    .await;

    // Get the wallets from that provider
    let wallet1 = wallets.pop().unwrap();
    let wallet2 = wallets.pop().unwrap();
    let wallet3 = wallets.pop().unwrap();
    let wallet4 = wallets.pop().unwrap();
    
    let auction_id = Contract::deploy(
        "./out/debug/english-auction.bin", 
        &wallet1, 
        TxParameters::default()
    )
    .await
    .unwrap();

    let sell_asset_id = Contract::deploy(
        "./tests/artifacts/asset/out/debug/asset.bin",
        &wallet1,
        TxParameters::default(),
    )
    .await
    .unwrap();

    let deploy_wallet = Metadata {
        asset: Some(Asset::new(sell_asset_id.to_string(), wallet1.clone())),
        auction: EnglishAuction::new(auction_id.to_string(), wallet1.clone()),
        wallet: wallet1.clone(),
    };

    let seller = Metadata {
        asset: Some(Asset::new(sell_asset_id.to_string(), wallet2.clone())),
        auction: EnglishAuction::new(auction_id.to_string(), wallet2.clone()),
        wallet: wallet2.clone(),
    };

    let buy_asset_id = Contract::deploy_with_salt(
        "./tests/artifacts/asset/out/debug/asset.bin",
        &wallet3,
        TxParameters::default(),
        Salt::from([1u8; 32]),
    )
    .await
    .unwrap();

    let buyer1 = Metadata {
        asset: Some(Asset::new(buy_asset_id.to_string(), wallet3.clone())),
        auction: EnglishAuction::new(auction_id.to_string(), wallet3.clone()),
        wallet: wallet3.clone(),
    };
    
    let buyer2 = Metadata {
        asset: Some(Asset::new(buy_asset_id.to_string(), wallet4.clone())),
        auction: EnglishAuction::new(auction_id.to_string(), wallet4.clone()),
        wallet: wallet4.clone(),
    };

    let sell_amount = 10;
    let inital_price = 1;
    let reserve_price = 3;
    let time = 10;

    (deploy_wallet, seller, buyer1, buyer2, sell_asset_id, buy_asset_id, sell_amount, inital_price, reserve_price, time)
}

async fn deploy_funds(
    deploy_wallet: &Metadata,
    wallet: &LocalWallet,
    asset_amount: u64
) {
    deploy_wallet
        .asset 
        .as_ref()
        .unwrap()
        .mint_and_send_to_address(asset_amount, wallet.address())
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap()
        .value;
}

async fn init(
    deploy_wallet: &Metadata,
    seller: &Metadata,
    sell_asset_id: ContractId,
    sell_amount: u64,
    buy_asset_id: ContractId,
    inital_price: u64,
    reserve_price: u64,
    time: u64
) -> bool {

    deploy_funds(&deploy_wallet, &seller.wallet, 100).await;
    
    let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
    let call_params = CallParameters::new(Some(sell_amount), Some(AssetId::from(*sell_asset_id)));

    seller
        .auction
        .constructor(
            englishauction_mod::Identity::Address(seller.wallet.address()), 
            buy_asset_id,
            inital_price,
            reserve_price,
            time)
        .tx_params(tx_params)
        .call_params(call_params)
        .call()
        .await
        .unwrap()
        .value
}

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

    #[tokio::test]
    async fn inits() {
        let (deploy_wallet, seller, _buyer1, _buyer2, sell_asset_id, buy_asset_id, sell_amount, inital_price, reserve_price, time) = setup().await;

        assert_eq!(
            deploy_wallet.auction.state().call().await.unwrap().value,
            0
        );

        assert!(
            init(
                &deploy_wallet,
                &seller,
                sell_asset_id,
                sell_amount,
                buy_asset_id,
                inital_price,
                reserve_price,
                time
            )
            .await
        );

        assert_eq!(
            deploy_wallet.auction.state().call().await.unwrap().value,
            1
        );
    }

    #[tokio::test]
    async fn sets_values() {
        let (deploy_wallet, seller, _buyer1, _buyer2, sell_asset_id, buy_asset_id, sell_amount, inital_price, reserve_price, time) = setup().await;

        init(
            &deploy_wallet,
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

        // TODO: Get the current block to test what block the auction should end
        // assert_eq!(
        //     deploy_wallet.auction.auction_end_block().call().await.unwrap().value,
        //     0
        // );

        assert_eq!(
            deploy_wallet.auction.sell_amount().call().await.unwrap().value,
            sell_amount
        );

        assert_eq!(
            deploy_wallet.auction.sell_asset().call().await.unwrap().value,
            sell_asset_id
        );

        assert_eq!(
            deploy_wallet.auction.reserve().call().await.unwrap().value,
            reserve_price
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_initalized_twice() {
        let (deploy_wallet, seller, _buyer1, _buyer2, sell_asset_id, buy_asset_id, sell_amount, inital_price, reserve_price, time) = setup().await;

        init(
            &deploy_wallet,
            &seller,
            sell_asset_id,
            sell_amount,
            buy_asset_id,
            inital_price,
            reserve_price,
            time
        )
        .await;

        assert!(
            init(
                &deploy_wallet,
                &seller,
                sell_asset_id,
                sell_amount,
                buy_asset_id,
                inital_price,
                reserve_price,
                time
            )
            .await
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_no_asset_provided() {
        let (deploy_wallet, seller, _buyer1, _buyer2, sell_asset_id, buy_asset_id, _sell_amount, inital_price, reserve_price, time) = setup().await;

        assert!(
            init(
                &deploy_wallet,
                &seller,
                sell_asset_id,
                0,
                buy_asset_id,
                inital_price,
                reserve_price,
                time
            )
            .await
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_inital_price_higher_than_reserve() {
        let (deploy_wallet, seller, _buyer1, _buyer2, sell_asset_id, buy_asset_id, sell_amount, _inital_price, _reserve_price, time) = setup().await;

        assert!(
            init(
                &deploy_wallet,
                &seller,
                sell_asset_id,
                sell_amount,
                buy_asset_id,
                2,
                1,
                time
            )
            .await
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_time_for_auction_is_zero() {
        let (deploy_wallet, seller, _buyer1, _buyer2, sell_asset_id, buy_asset_id, sell_amount, inital_price, reserve_price, _time) = setup().await;

        assert!(
            init(
                &deploy_wallet,
                &seller,
                sell_asset_id,
                sell_amount,
                buy_asset_id,
                inital_price,
                reserve_price,
                0
            )
            .await
        );
    }
}

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

    #[tokio::test]
    #[should_panic]
    async fn panics_when_over_reserve() {
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
}

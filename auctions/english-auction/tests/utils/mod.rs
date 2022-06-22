use fuels::{prelude::*, contract::contract::CallResponse, tx::ContractId, tx::Salt};
use fuels_abigen_macro::abigen;

// Load abi from json
abigen!(EnglishAuction, "out/debug/english-auction-abi.json");
abigen!(MyAsset, "tests/artifacts/asset/out/debug/asset-abi.json");

pub struct Metadata {
    asset: core::option::Option<MyAsset>,
    auction: EnglishAuction,
    wallet: LocalWallet,
}

pub mod test_helpers {

    use super::*;

    pub async fn setup() -> (Metadata, Metadata, Metadata, Metadata, ContractId, ContractId, u64, u64, u64, u64) {
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
            asset: Some(MyAsset::new(sell_asset_id.to_string(), wallet1.clone())),
            auction: EnglishAuction::new(auction_id.to_string(), wallet1.clone()),
            wallet: wallet1.clone(),
        };
    
        let seller = Metadata {
            asset: Some(MyAsset::new(sell_asset_id.to_string(), wallet2.clone())),
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
            asset: Some(MyAsset::new(buy_asset_id.to_string(), wallet3.clone())),
            auction: EnglishAuction::new(auction_id.to_string(), wallet3.clone()),
            wallet: wallet3.clone(),
        };
        
        let buyer2 = Metadata {
            asset: Some(MyAsset::new(buy_asset_id.to_string(), wallet4.clone())),
            auction: EnglishAuction::new(auction_id.to_string(), wallet4.clone()),
            wallet: wallet4.clone(),
        };
    
        let sell_amount = 10;
        let inital_price = 1;
        let reserve_price = 10;
        let time = 10;
    
        (deploy_wallet, seller, buyer1, buyer2, sell_asset_id, buy_asset_id, sell_amount, inital_price, reserve_price, time)
    }
}

pub mod abi_calls {

    use super::*;

    pub async fn init_token(
        deploy_wallet: &Metadata,
        seller: &Metadata,
        sell_asset_id: ContractId,
        sell_amount: u64,
        _buy_asset_id: ContractId,
        inital_price: u64,
        reserve_price: u64,
        time: u64,
        buy_asset_struct: Asset,
        sell_asset_struct: Asset
    ) -> u64 {
    
        deploy_funds(&deploy_wallet, &seller.wallet, 100).await;
        
        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(sell_amount), Some(AssetId::from(*sell_asset_id)), None);
    
        seller
            .auction
            .constructor(
                englishauction_mod::Identity::Address(seller.wallet.address()), 
                sell_asset_struct,
                buy_asset_struct,
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

    pub async fn init_nft(
        _deploy_wallet: &Metadata,
        seller: &Metadata,
        _sell_asset_id: ContractId,
        _sell_amount: u64,
        _sell_nft_id: u64,
        _buy_asset_id: ContractId,
        inital_price: u64,
        reserve_price: u64,
        time: u64,
        buy_asset_struct: Asset,
        sell_asset_struct: Asset
    ) -> u64 {
    
        seller
            .auction
            .constructor(
                englishauction_mod::Identity::Address(seller.wallet.address()), 
                sell_asset_struct,
                buy_asset_struct,
                inital_price,
                reserve_price,
                time)
            .call()
            .await
            .unwrap()
            .value
    }

    pub async fn bid_tokens(
        deploy_wallet: &Metadata, 
        bidder: &Metadata,
        auction_id: u64, 
        asset_id: ContractId, 
        amount: u64,
        asset: Asset
    ) -> CallResponse<()> {

        deploy_funds(&deploy_wallet, &bidder.wallet, 100).await;

        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(amount), Some(AssetId::from(*asset_id)), None);

        bidder
            .auction
            .bid(auction_id, asset)
            .tx_params(tx_params)
            .call_params(call_params)
            .append_variable_outputs(2)
            .call()
            .await
            .unwrap()
    }

    pub async fn bid_nft(
        bidder: &Metadata,
        auction_id: u64, 
        asset_id: ContractId, 
        amount: u64,
        nft_id: u64
    ) -> CallResponse<()> {

        let buy_asset_struct = Asset {
            contract_id: asset_id,
            amount,
            nft_id: Option::Some(nft_id)
        };

        bidder
            .auction
            .bid(auction_id, buy_asset_struct)
            .append_variable_outputs(2)
            .call()
            .await
            .unwrap()
    }

    pub async fn buy_reserve_tokens(
        deploy_wallet: &Metadata, 
        bidder: &Metadata,
        auction_id: u64, 
        asset_id: ContractId, 
        amount: u64,
        asset: Asset
    ) -> CallResponse<()> {

        deploy_funds(&deploy_wallet, &bidder.wallet, 100).await;

        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(amount), Some(AssetId::from(*asset_id)), None);

        bidder
            .auction
            .buy_reserve(auction_id, asset)
            .tx_params(tx_params)
            .call_params(call_params)
            .append_variable_outputs(2)
            .call()
            .await
            .unwrap()
    }

    pub async fn buy_reserve_nft(
        bidder: &Metadata,
        auction_id: u64, 
        asset: Asset
    ) -> CallResponse<()> {

        bidder
            .auction
            .buy_reserve(auction_id, asset)
            .append_variable_outputs(2)
            .call()
            .await
            .unwrap()
    }

    pub async fn withdraw(call_wallet: &Metadata, auction_id: u64) -> CallResponse<()> {
        call_wallet.auction.withdraw(auction_id).call().await.unwrap()
    }

    pub async fn auction_end_block(call_wallet: &Metadata, auction_id: u64) -> u64 {
        call_wallet.auction.auction_end_block(auction_id).call().await.unwrap().value
    }

    pub async fn current_bid(call_wallet: &Metadata, auction_id: u64) -> u64 {
        call_wallet.auction.current_bid(auction_id).call().await.unwrap().value
    }

    // Uncomment when https://github.com/FuelLabs/fuels-rs/issues/420 is resolved
    // pub async fn deposits(call_wallet: &Metadata, auction_id: u64) -> englishauction_mod::Option {
    //     call_wallet.auction.deposits(auction_id).call().await.unwrap().value
    // }

    // Uncomment when https://github.com/FuelLabs/fuels-rs/issues/421 is resolved
    // pub async fn highest_bidder(call_wallet: &Metadata, auction_id: u64) -> englishauction_mod::Option {
    //     call_wallet.auction.highest_bidder(auction_id).call().await.unwrap().value
    // }

    // Uncomment when https://github.com/FuelLabs/fuels-rs/issues/421 is resolved
    // pub async fn reserve(call_wallet: &Metadata, auction_id: u64) -> englishauction_mod::Option {
    //     call_wallet.auction.reserve(auction_id).call().await.unwrap().value
    // }

    pub async fn sell_amount(call_wallet: &Metadata, auction_id: u64) -> u64 {
        call_wallet.auction.sell_amount(auction_id).call().await.unwrap().value
    }

    pub async fn sell_asset(call_wallet: &Metadata, auction_id: u64) -> ContractId {
        call_wallet.auction.sell_asset(auction_id).call().await.unwrap().value
    }

    pub async fn state(call_wallet: &Metadata, auction_id: u64) -> u64 {
        call_wallet.auction.state(auction_id).call().await.unwrap().value
    }
}

pub async fn deploy_funds(
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
use fuels::{contract::contract::CallResponse, prelude::*, tx::ContractId, tx::Salt};
use rand::prelude::{Rng, SeedableRng, StdRng};

// Load abi from json
abigen!(EnglishAuction, "out/debug/english-auction-abi.json");
abigen!(MyAsset, "tests/artifacts/asset/out/debug/asset-abi.json");

pub struct Metadata {
    asset: Option<MyAsset>,
    auction: EnglishAuction,
    wallet: WalletUnlocked,
}

pub mod asset_abi_calls {

    use super::*;

    pub async fn mint_and_send_to_address(amount: u64, contract: &MyAsset, recipient: Address) -> CallResponse<()> {
        contract
            .methods()
            .mint_and_send_to_address(amount, recipient)
            .call()
            .await
            .unwrap()
    }
}

pub mod english_auction_abi_calls {

    use super::*;

    pub async fn auction_info(auction_id: u64, contract: &EnglishAuction) -> Option<Auction> {
        contract
            .methods()
            .auction_info(auction_id)
            .call()
            .await
            .unwrap()
            .value
    }

    pub async fn bid(auction_id: u64, bid_asset: Asset, contract: &EnglishAuction) -> CallResponse<()> {
        match bid_asset {
            Asset::NFTAsset(bid_asset) => {
                contract
                    .methods()
                    .bid(auction_id, Asset::NFTAsset(bid_asset.clone()))
                    .set_contracts(&[bid_asset.contract_id.into()])
                    .call()
                    .await
                    .unwrap()
            },
            Asset::TokenAsset(bid_asset) => {
                let tx_params = TxParameters::new(None, Some(1_000_000), None);
                let call_params = CallParameters::new(Some(bid_asset.amount), Some(AssetId::from(*bid_asset.contract_id)), None);
        
                contract
                    .methods()
                    .bid(auction_id, Asset::TokenAsset(bid_asset.clone()))
                    .tx_params(tx_params)
                    .call_params(call_params)
                    .call()
                    .await
                    .unwrap()
            }
        }
    }

    pub async fn cancel(auction_id: u64, contract: &EnglishAuction) -> CallResponse<()> {
        contract
            .methods()
            .cancel(auction_id)
            .call()
            .await
            .unwrap()
    }

    pub async fn create(
        bid_asset: Asset,
        contract: &EnglishAuction,
        duration: u64,
        inital_price: u64,
        reserve_price: Option<u64>,
        seller: Identity,
        sell_asset: Asset
    ) -> u64 {
        match sell_asset {
            Asset::NFTAsset(sell_asset) => {
                contract
                    .methods()
                    .create(bid_asset, duration, inital_price, reserve_price, seller, Asset::NFTAsset(sell_asset.clone()))
                    .set_contracts(&[sell_asset.contract_id.into()])
                    .call()
                    .await
                    .unwrap()
                    .value
            },
            Asset::TokenAsset(sell_asset) => {
                let tx_params = TxParameters::new(None, Some(1_000_000), None);
                let call_params = CallParameters::new(Some(sell_asset.amount), Some(AssetId::from(*sell_asset.contract_id)), None);
        
                contract
                    .methods()
                    .create(bid_asset, duration, inital_price, reserve_price, seller, Asset::TokenAsset(sell_asset.clone()))
                    .tx_params(tx_params)
                    .call_params(call_params)
                    .call()
                    .await
                    .unwrap()
                    .value
            }
        }
    }

    pub async fn deposit(auction_id: u64, contract: &EnglishAuction, identity: Identity) -> Option<Asset> {
        contract
            .methods()
            .deposit(auction_id, identity)
            .call()
            .await
            .unwrap()
            .value
    }

    pub async fn withdraw(auction_id: u64, contract: &EnglishAuction) -> CallResponse<()> {
        contract
            .methods()
            .withdraw(auction_id)
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }

    pub async fn total_auctions(contract: &EnglishAuction) -> u64 {
        contract
            .methods()
            .total_auctions()
            .call()
            .await
            .unwrap()
            .value
    }
}

pub mod test_helpers {

    use super::*;

    pub async fn setup() -> (
        Metadata,
        Metadata,
        Metadata,
        Metadata,
        ContractId,
        ContractId,
        u64,
        u64,
        u64,
        u64,
    ) {
        let num_wallets = 4;
        let coins_per_wallet = 1;
        let coin_amount = 1000000;
        let config = Config {
            manual_blocks_enabled: true, // Necessary so the `produce_blocks` API can be used locally
            ..Config::local_node()
        };
        let mut wallets = launch_custom_provider_and_get_wallets(
            WalletsConfig::new(Some(num_wallets), Some(coins_per_wallet), Some(coin_amount)),
            Some(config),
        )
        .await;

        // Get the wallets from that provider
        let wallet1 = wallets.pop().unwrap();
        let wallet2 = wallets.pop().unwrap();
        let wallet3 = wallets.pop().unwrap();
        let wallet4 = wallets.pop().unwrap();

        let auction_id = Contract::deploy(
            "./out/debug/english-auction.bin",
            &wallet1,
            TxParameters::default(),
            StorageConfiguration::with_storage_path(Some(
                "./out/debug/english-auction-storage_slots.json".to_string(),
            )),
        )
        .await
        .unwrap();

        let sell_asset_id = Contract::deploy(
            "./tests/artifacts/asset/out/debug/asset.bin",
            &wallet1,
            TxParameters::default(),
            StorageConfiguration::default(),
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

        let rng = &mut StdRng::seed_from_u64(2322u64);
        let salt: [u8; 32] = rng.gen();
        let buy_asset_id = Contract::deploy_with_parameters(
            "./tests/artifacts/asset/out/debug/asset.bin",
            &wallet3,
            TxParameters::default(),
            StorageConfiguration::default(),
            Salt::from(salt),
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

        (
            deploy_wallet,
            seller,
            buyer1,
            buyer2,
            sell_asset_id.into(),
            buy_asset_id.into(),
            sell_amount,
            inital_price,
            reserve_price,
            time,
        )
    }

    pub async fn token_asset(contract_id: ContractId, amount: u64) -> Asset {
        let token = TokenAsset {
            contract_id,
            amount,
        };

        Asset::TokenAsset(token)
    }

    pub async fn nft_asset(contract_id: ContractId, token_id: u64) -> Asset {
        let token = NFTAsset {
            contract_id,
            token_id,
        };

        Asset::NFTAsset(token)
    }
}

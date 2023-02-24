use fuels::{
    prelude::{
        abigen, launch_custom_provider_and_get_wallets, Config, Contract, StorageConfiguration,
        TxParameters, WalletUnlocked, WalletsConfig,
    },
    tx::{ContractId, Salt},
    types::Identity,
};

abigen!(
    Contract(
        name = "EnglishAuction",
        abi = "./contracts/auction-contract/out/debug/auction-contract-abi.json"
    ),
    Contract(
        name = "Nft",
        abi = "./contracts/auction-contract/tests/artifacts/NFT/out/debug/NFT-abi.json"
    ),
    Contract(
        name = "MyAsset",
        abi = "./contracts/auction-contract/tests/artifacts/asset/out/debug/asset-abi.json"
    ),
);

const AUCTION_CONTRACT_BINARY_PATH: &str = "./out/debug/auction-contract.bin";
const AUCTION_CONTRACT_STORAGE_PATH: &str = "./out/debug/auction-contract-storage_slots.json";
const NATIVE_ASSET_BINARY_PATH: &str = "./tests/artifacts/asset/out/debug/asset.bin";
const NFT_CONTRACT_BINARY_PATH: &str = "./tests/artifacts/NFT/out/debug/NFT.bin";
const NFT_CONTRACT_STORAGE_PATH: &str = "./tests/artifacts/NFT/out/debug/NFT-storage_slots.json";

pub(crate) struct Metadata {
    pub(crate) asset: MyAsset,
    pub(crate) auction: EnglishAuction,
    pub(crate) nft: Nft,
    pub(crate) wallet: WalletUnlocked,
}

pub(crate) async fn create_auction_copy(
    bid_asset: AuctionAsset,
    highest_bidder: Option<Identity>,
    end_block: u64,
    initial_price: u64,
    reserve_price: Option<u64>,
    sell_asset: AuctionAsset,
    seller: Identity,
    state: State,
) -> Auction {
    Auction {
        bid_asset,
        highest_bidder,
        end_block,
        initial_price,
        reserve_price,
        sell_asset,
        seller,
        state,
    }
}

pub(crate) async fn defaults_nft() -> (u64, u64, u64, u64) {
    let sell_count = 1;
    let inital_count = 1;
    let reserve_count = 1;
    let duration = 10;

    (sell_count, inital_count, reserve_count, duration)
}

pub(crate) async fn defaults_token() -> (u64, u64, u64, u64) {
    let sell_amount = 10;
    let initial_price = 1;
    let reserve_price = 10;
    let duration = 10;

    (sell_amount, initial_price, reserve_price, duration)
}

pub(crate) async fn nft_asset(asset_id: ContractId, token_id: u64) -> AuctionAsset {
    let token = NFTAsset { asset_id, token_id };

    AuctionAsset::NFTAsset(token)
}

pub(crate) async fn setup() -> (
    Metadata,
    Metadata,
    Metadata,
    Metadata,
    ContractId,
    ContractId,
    ContractId,
    ContractId,
    ContractId,
) {
    let number_of_wallets = 4;
    let coins_per_wallet = 1;
    let coin_amount = 1000000;
    let config = Config {
        manual_blocks_enabled: true, // Necessary so the `produce_blocks` API can be used locally
        ..Config::local_node()
    };
    let mut wallets = launch_custom_provider_and_get_wallets(
        WalletsConfig::new(
            Some(number_of_wallets),
            Some(coins_per_wallet),
            Some(coin_amount),
        ),
        Some(config),
        None,
    )
    .await;

    // Get the wallets from that provider
    let wallet1 = wallets.pop().unwrap();
    let wallet2 = wallets.pop().unwrap();
    let wallet3 = wallets.pop().unwrap();
    let wallet4 = wallets.pop().unwrap();

    let auction_id = Contract::deploy(
        AUCTION_CONTRACT_BINARY_PATH,
        &wallet1,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(AUCTION_CONTRACT_STORAGE_PATH.to_string())),
    )
    .await
    .unwrap();

    let sell_asset_id = Contract::deploy(
        NATIVE_ASSET_BINARY_PATH,
        &wallet1,
        TxParameters::default(),
        StorageConfiguration::default(),
    )
    .await
    .unwrap();

    let sell_nft_id = Contract::deploy(
        NFT_CONTRACT_BINARY_PATH,
        &wallet1,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(NFT_CONTRACT_STORAGE_PATH.to_string())),
    )
    .await
    .unwrap();

    let deploy_wallet = Metadata {
        asset: MyAsset::new(sell_asset_id.clone(), wallet1.clone()),
        auction: EnglishAuction::new(auction_id.clone(), wallet1.clone()),
        nft: Nft::new(sell_nft_id.clone(), wallet1.clone()),
        wallet: wallet1,
    };

    let seller = Metadata {
        asset: MyAsset::new(sell_asset_id.clone(), wallet2.clone()),
        auction: EnglishAuction::new(auction_id.clone(), wallet2.clone()),
        nft: Nft::new(sell_nft_id.clone(), wallet2.clone()),
        wallet: wallet2,
    };

    let buy_asset_id = Contract::deploy_with_parameters(
        NATIVE_ASSET_BINARY_PATH,
        &wallet3,
        TxParameters::default(),
        StorageConfiguration::default(),
        Salt::from([1u8; 32]),
    )
    .await
    .unwrap();

    let buy_nft_id = Contract::deploy_with_parameters(
        NFT_CONTRACT_BINARY_PATH,
        &wallet3,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(NFT_CONTRACT_STORAGE_PATH.to_string())),
        Salt::from([2u8; 32]),
    )
    .await
    .unwrap();

    let buyer1 = Metadata {
        asset: MyAsset::new(buy_asset_id.clone(), wallet3.clone()),
        auction: EnglishAuction::new(auction_id.clone(), wallet3.clone()),
        nft: Nft::new(buy_nft_id.clone(), wallet3.clone()),
        wallet: wallet3,
    };

    let buyer2 = Metadata {
        asset: MyAsset::new(buy_asset_id.clone(), wallet4.clone()),
        auction: EnglishAuction::new(auction_id.clone(), wallet4.clone()),
        nft: Nft::new(buy_nft_id.clone(), wallet4.clone()),
        wallet: wallet4,
    };

    (
        deploy_wallet,
        seller,
        buyer1,
        buyer2,
        auction_id.into(),
        sell_asset_id.into(),
        sell_nft_id.into(),
        buy_asset_id.into(),
        buy_nft_id.into(),
    )
}

pub(crate) async fn token_asset(asset_id: ContractId, amount: u64) -> AuctionAsset {
    let token = TokenAsset { asset_id, amount };

    AuctionAsset::TokenAsset(token)
}

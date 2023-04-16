use fuels::{
    prelude::{
        abigen, launch_custom_provider_and_get_wallets, Bech32ContractId, Config, Contract,
        LoadConfiguration, StorageConfiguration, TxParameters, WalletUnlocked, WalletsConfig,
    },
    tx::ContractId,
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
    pub(crate) asset: MyAsset<WalletUnlocked>,
    pub(crate) auction: EnglishAuction<WalletUnlocked>,
    pub(crate) nft: Nft<WalletUnlocked>,
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

    let wallet1 = wallets.pop().unwrap();
    let wallet2 = wallets.pop().unwrap();
    let wallet3 = wallets.pop().unwrap();
    let wallet4 = wallets.pop().unwrap();

    let auction_storage_configuration =
        StorageConfiguration::load_from(AUCTION_CONTRACT_STORAGE_PATH);
    let nft_storage_configuration = StorageConfiguration::load_from(NFT_CONTRACT_STORAGE_PATH);
    let buy_nft_storage_configuration = StorageConfiguration::load_from(NFT_CONTRACT_STORAGE_PATH);

    let auction_configuration = LoadConfiguration::default()
        .set_storage_configuration(auction_storage_configuration.unwrap());
    let nft_configuration =
        LoadConfiguration::default().set_storage_configuration(nft_storage_configuration.unwrap());
    let buy_nft_configuration = LoadConfiguration::default()
        .set_storage_configuration(buy_nft_storage_configuration.unwrap())
        .set_salt([2u8; 32]);

    let auction_id = Contract::load_from(AUCTION_CONTRACT_BINARY_PATH, auction_configuration)
        .unwrap()
        .deploy(&wallet1, TxParameters::default())
        .await
        .unwrap();
    let sell_asset_id = Contract::load_from(NATIVE_ASSET_BINARY_PATH, LoadConfiguration::default())
        .unwrap()
        .deploy(&wallet1, TxParameters::default())
        .await
        .unwrap();

    let sell_nft_id = Contract::load_from(NFT_CONTRACT_BINARY_PATH, nft_configuration)
        .unwrap()
        .deploy(&wallet1, TxParameters::default())
        .await
        .unwrap();
    let buy_asset_id = Contract::load_from(
        NATIVE_ASSET_BINARY_PATH,
        LoadConfiguration::default().set_salt([1u8; 32]),
    )
    .unwrap()
    .deploy(&wallet3, TxParameters::default())
    .await
    .unwrap();
    let buy_nft_id = Contract::load_from(NFT_CONTRACT_BINARY_PATH, buy_nft_configuration)
        .unwrap()
        .deploy(&wallet3, TxParameters::default())
        .await
        .unwrap();

    let deploy_wallet = user(
        wallet1,
        sell_asset_id.clone(),
        auction_id.clone(),
        sell_nft_id.clone(),
    )
    .await;
    let seller = user(
        wallet2,
        sell_asset_id.clone(),
        auction_id.clone(),
        sell_nft_id.clone(),
    )
    .await;
    let buyer1 = user(
        wallet3,
        buy_asset_id.clone(),
        auction_id.clone(),
        buy_nft_id.clone(),
    )
    .await;
    let buyer2 = user(
        wallet4,
        buy_asset_id.clone(),
        auction_id.clone(),
        buy_asset_id.clone(),
    )
    .await;

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

async fn user(
    user_wallet: WalletUnlocked,
    asset_id: Bech32ContractId,
    auction_id: Bech32ContractId,
    nft_id: Bech32ContractId,
) -> Metadata {
    Metadata {
        asset: MyAsset::new(asset_id.clone(), user_wallet.clone()),
        auction: EnglishAuction::new(auction_id.clone(), user_wallet.clone()),
        nft: Nft::new(nft_id.clone(), user_wallet.clone()),
        wallet: user_wallet,
    }
}

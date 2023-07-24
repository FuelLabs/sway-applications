use fuels::{
    prelude::{
        abigen, launch_custom_provider_and_get_wallets, AssetConfig, AssetId, Bech32ContractId,
        Config, Contract, ContractId, LoadConfiguration, StorageConfiguration, TxParameters,
        WalletUnlocked, WalletsConfig, BASE_ASSET_ID,
    },
    types::Identity,
};

abigen!(
    Contract(
        name = "EnglishAuction",
        abi = "./contracts/auction-contract/out/debug/auction-contract-abi.json"
    ),
    Contract(
        name = "Nft",
        abi = "./contracts/test-artifacts/NFT/out/debug/NFT-abi.json"
    ),
);

const AUCTION_CONTRACT_BINARY_PATH: &str = "./out/debug/auction-contract.bin";
const AUCTION_CONTRACT_STORAGE_PATH: &str = "./out/debug/auction-contract-storage_slots.json";
const NFT_CONTRACT_BINARY_PATH: &str = "../test-artifacts/NFT/out/debug/NFT.bin";
const NFT_CONTRACT_STORAGE_PATH: &str = "../test-artifacts/NFT/out/debug/NFT-storage_slots.json";

pub(crate) struct Metadata {
    pub(crate) auction: EnglishAuction<WalletUnlocked>,
    pub(crate) nft: Nft<WalletUnlocked>,
    pub(crate) wallet: WalletUnlocked,
}

#[allow(clippy::too_many_arguments)]
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
    let initial_count = 1;
    let reserve_count = 1;
    let duration = 10;

    (sell_count, initial_count, reserve_count, duration)
}

pub(crate) async fn defaults_token() -> (u64, u64, u64, u64, u64) {
    let sell_amount = 10;
    let initial_price = 1;
    let reserve_price = 10;
    let duration = 10;
    let initial_wallet_amount = 1_000_000;

    (
        sell_amount,
        initial_price,
        reserve_price,
        duration,
        initial_wallet_amount,
    )
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
    let number_of_coins = 1;
    let coin_amount = 1_000_000;
    let number_of_wallets = 4;

    let base_asset = AssetConfig {
        id: BASE_ASSET_ID,
        num_coins: number_of_coins,
        coin_amount,
    };
    let buy_asset_id = AssetId::new([1; 32]);
    let buy_asset = AssetConfig {
        id: buy_asset_id,
        num_coins: number_of_coins,
        coin_amount,
    };
    let sell_asset_id = AssetId::new([2; 32]);
    let sell_asset = AssetConfig {
        id: sell_asset_id,
        num_coins: number_of_coins,
        coin_amount,
    };
    let assets = vec![base_asset, buy_asset, sell_asset];

    let wallet_config = WalletsConfig::new_multiple_assets(number_of_wallets, assets);

    let provider_config = Config {
        manual_blocks_enabled: true, // Necessary so the `produce_blocks` API can be used locally
        ..Config::local_node()
    };
    let mut wallets =
        launch_custom_provider_and_get_wallets(wallet_config, Some(provider_config), None).await;

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

    let sell_nft_id = Contract::load_from(NFT_CONTRACT_BINARY_PATH, nft_configuration)
        .unwrap()
        .deploy(&wallet1, TxParameters::default())
        .await
        .unwrap();
    let buy_nft_id = Contract::load_from(NFT_CONTRACT_BINARY_PATH, buy_nft_configuration)
        .unwrap()
        .deploy(&wallet3, TxParameters::default())
        .await
        .unwrap();

    let deploy_wallet = user(wallet1, auction_id.clone(), sell_nft_id.clone()).await;
    let seller = user(wallet2, auction_id.clone(), sell_nft_id.clone()).await;
    let buyer1 = user(wallet3, auction_id.clone(), buy_nft_id.clone()).await;
    let buyer2 = user(
        wallet4,
        auction_id.clone(),
        ContractId::from(*buy_asset_id).into(),
    )
    .await;

    (
        deploy_wallet,
        seller,
        buyer1,
        buyer2,
        auction_id.into(),
        ContractId::from(*sell_asset_id),
        sell_nft_id.into(),
        ContractId::from(*buy_asset_id),
        buy_nft_id.into(),
    )
}

pub(crate) async fn token_asset(asset_id: ContractId, amount: u64) -> AuctionAsset {
    let token = TokenAsset { asset_id, amount };

    AuctionAsset::TokenAsset(token)
}

async fn user(
    user_wallet: WalletUnlocked,
    auction_id: Bech32ContractId,
    nft_id: Bech32ContractId,
) -> Metadata {
    Metadata {
        auction: EnglishAuction::new(auction_id, user_wallet.clone()),
        nft: Nft::new(nft_id, user_wallet.clone()),
        wallet: user_wallet,
    }
}

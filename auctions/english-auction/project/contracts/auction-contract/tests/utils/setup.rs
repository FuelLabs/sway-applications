use fuels::{
    prelude::{
        abigen, launch_custom_provider_and_get_wallets, AssetConfig, Bech32ContractId, Config,
        Contract, ContractId, LoadConfiguration, StorageConfiguration, TxPolicies, WalletUnlocked,
        WalletsConfig, BASE_ASSET_ID,
    },
    types::{AssetId, Identity},
};

abigen!(Contract(
    name = "EnglishAuction",
    abi = "./contracts/auction-contract/out/debug/auction-contract-abi.json"
),);

const AUCTION_CONTRACT_BINARY_PATH: &str = "./out/debug/auction-contract.bin";
const AUCTION_CONTRACT_STORAGE_PATH: &str = "./out/debug/auction-contract-storage_slots.json";

pub(crate) struct Metadata {
    pub(crate) auction: EnglishAuction<WalletUnlocked>,
    pub(crate) wallet: WalletUnlocked,
}

#[allow(clippy::too_many_arguments)]
pub(crate) async fn create_auction_copy(
    bid_asset: AssetId,
    highest_bid: u64,
    highest_bidder: Option<Identity>,
    end_block: u32,
    initial_price: u64,
    reserve_price: Option<u64>,
    sell_asset: AssetId,
    sell_asset_amount: u64,
    seller: Identity,
    state: State,
) -> Auction {
    Auction {
        bid_asset,
        end_block,
        highest_bid,
        highest_bidder,
        initial_price,
        reserve_price,
        sell_asset,
        sell_asset_amount,
        seller,
        state,
    }
}

pub(crate) async fn defaults() -> (u64, u64, u64, u32, u64) {
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

pub(crate) async fn setup() -> (
    Metadata,
    Metadata,
    Metadata,
    Metadata,
    ContractId,
    AssetId,
    AssetId,
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
    let mut wallets = launch_custom_provider_and_get_wallets(wallet_config, None, None)
        .await
        .unwrap();

    let wallet1 = wallets.pop().unwrap();
    let wallet2 = wallets.pop().unwrap();
    let wallet3 = wallets.pop().unwrap();
    let wallet4 = wallets.pop().unwrap();

    let auction_storage_configuration =
        StorageConfiguration::default().add_slot_overrides_from_file(AUCTION_CONTRACT_STORAGE_PATH);
    let auction_configuration = LoadConfiguration::default()
        .with_storage_configuration(auction_storage_configuration.unwrap());

    let auction_id = Contract::load_from(AUCTION_CONTRACT_BINARY_PATH, auction_configuration)
        .unwrap()
        .deploy(&wallet1, TxPolicies::default())
        .await
        .unwrap();

    let deploy_wallet = user(wallet1, auction_id.clone()).await;
    let seller = user(wallet2, auction_id.clone()).await;
    let buyer1 = user(wallet3, auction_id.clone()).await;
    let buyer2 = user(wallet4, auction_id.clone()).await;

    (
        deploy_wallet,
        seller,
        buyer1,
        buyer2,
        auction_id.into(),
        sell_asset_id,
        buy_asset_id,
    )
}

async fn user(user_wallet: WalletUnlocked, auction_id: Bech32ContractId) -> Metadata {
    Metadata {
        auction: EnglishAuction::new(auction_id, user_wallet.clone()),
        wallet: user_wallet,
    }
}

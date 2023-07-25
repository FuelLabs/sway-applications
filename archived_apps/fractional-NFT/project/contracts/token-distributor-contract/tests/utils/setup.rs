use fuels::{
    accounts::ViewOnlyAccount,
    prelude::{
        abigen, launch_custom_provider_and_get_wallets, Contract, ContractId, LoadConfiguration,
        StorageConfiguration, TxParameters, WalletUnlocked, WalletsConfig,
    },
    tx::AssetId,
};

abigen!(
    Contract(
        name = "TokenDistributor",
        abi =
            "./contracts/token-distributor-contract/out/debug/token-distributor-contract-abi.json"
    ),
    Contract(
        name = "FractionalNFT",
        abi = "./contracts/fractional-NFT-contract/out/debug/fractional-NFT-contract-abi.json"
    ),
    Contract(
        name = "Nft",
        abi = "./contracts/token-distributor-contract/tests/artifacts/NFT/out/debug/NFT-abi.json"
    ),
    Contract(
        name = "Asset",
        abi =
            "./contracts/token-distributor-contract/tests/artifacts/asset/out/debug/asset-abi.json"
    ),
);

const ASSET_CONTRACT_BINARY_PATH: &str = "./tests/artifacts/asset/out/debug/asset.bin";
const FRACTIONAL_NFT_CONTRACT_BINARY_PATH: &str =
    "../fractional-NFT-contract/out/debug/fractional-NFT-contract.bin";
const FRACTIONAL_NFT_CONTRACT_STORAGE_PATH: &str =
    "../fractional-NFT-contract/out/debug/fractional-NFT-contract-storage_slots.json";
const NFT_CONTRACT_BINARY_PATH: &str = "./tests/artifacts/NFT/out/debug/NFT.bin";
const NFT_CONTRACT_STORAGE_PATH: &str = "./tests/artifacts/NFT/out/debug/NFT-storage_slots.json";
const TOKEN_DISTRIBUTOR_CONTRACT_BINARY_PATH: &str = "./out/debug/token-distributor-contract.bin";
const TOKEN_DISTRIBUTOR_CONTRACT_STORAGE_PATH: &str =
    "./out/debug/token-distributor-contract-storage_slots.json";

pub(crate) struct Metadata {
    pub(crate) asset: Asset<WalletUnlocked>,
    pub(crate) f_nft: FractionalNFT<WalletUnlocked>,
    pub(crate) nft: Nft<WalletUnlocked>,
    pub(crate) token_distributor: TokenDistributor<WalletUnlocked>,
    pub(crate) wallet: WalletUnlocked,
}

pub(crate) async fn defaults() -> (u64, u64, u64, u64, u64) {
    let asset_supply = 100;
    let price = 1;
    let purchase_amount = 2;
    let supply = 10;
    let reserve = 10;
    (price, reserve, supply, purchase_amount, asset_supply)
}

pub(crate) async fn setup() -> (
    Metadata,
    Metadata,
    Metadata,
    ContractId,
    ContractId,
    ContractId,
    ContractId,
) {
    let number_of_wallets = 3;
    let coins_per_wallet = 1;
    let amount_per_coin = 1_000_000;

    let mut wallets = launch_custom_provider_and_get_wallets(
        WalletsConfig::new(
            Some(number_of_wallets),
            Some(coins_per_wallet),
            Some(amount_per_coin),
        ),
        None,
        None,
    )
    .await;

    let wallet1 = wallets.pop().unwrap();
    let wallet2 = wallets.pop().unwrap();
    let wallet3 = wallets.pop().unwrap();

    let distributor_storage_configuration =
        StorageConfiguration::load_from(TOKEN_DISTRIBUTOR_CONTRACT_STORAGE_PATH);
    let fractional_storage_configuration =
        StorageConfiguration::load_from(FRACTIONAL_NFT_CONTRACT_STORAGE_PATH);
    let nft_storage_configuration = StorageConfiguration::load_from(NFT_CONTRACT_STORAGE_PATH);

    let distributor_configuration = LoadConfiguration::default()
        .set_storage_configuration(distributor_storage_configuration.unwrap());
    let fractional_configuration = LoadConfiguration::default()
        .set_storage_configuration(fractional_storage_configuration.unwrap());
    let nft_configuration =
        LoadConfiguration::default().set_storage_configuration(nft_storage_configuration.unwrap());

    let token_distributor_id = Contract::load_from(
        TOKEN_DISTRIBUTOR_CONTRACT_BINARY_PATH,
        distributor_configuration,
    )
    .unwrap()
    .deploy(&wallet1, TxParameters::default())
    .await
    .unwrap();

    let f_nft_id = Contract::load_from(
        FRACTIONAL_NFT_CONTRACT_BINARY_PATH,
        fractional_configuration,
    )
    .unwrap()
    .deploy(&wallet1, TxParameters::default())
    .await
    .unwrap();

    let nft_id = Contract::load_from(NFT_CONTRACT_BINARY_PATH, nft_configuration)
        .unwrap()
        .deploy(&wallet1, TxParameters::default())
        .await
        .unwrap();

    let asset_id = Contract::load_from(ASSET_CONTRACT_BINARY_PATH, LoadConfiguration::default())
        .unwrap()
        .deploy(&wallet1, TxParameters::default())
        .await
        .unwrap();

    let deploy_wallet = Metadata {
        asset: Asset::new(asset_id.clone(), wallet1.clone()),
        f_nft: FractionalNFT::new(f_nft_id.clone(), wallet1.clone()),
        nft: Nft::new(nft_id.clone(), wallet1.clone()),
        token_distributor: TokenDistributor::new(token_distributor_id.clone(), wallet1.clone()),
        wallet: wallet1,
    };

    let owner1 = Metadata {
        asset: Asset::new(asset_id.clone(), wallet2.clone()),
        f_nft: FractionalNFT::new(f_nft_id.clone(), wallet2.clone()),
        nft: Nft::new(nft_id.clone(), wallet2.clone()),
        token_distributor: TokenDistributor::new(token_distributor_id.clone(), wallet2.clone()),
        wallet: wallet2,
    };

    let owner2 = Metadata {
        asset: Asset::new(asset_id.clone(), wallet3.clone()),
        f_nft: FractionalNFT::new(f_nft_id.clone(), wallet3.clone()),
        nft: Nft::new(nft_id.clone(), wallet3.clone()),
        token_distributor: TokenDistributor::new(token_distributor_id.clone(), wallet3.clone()),
        wallet: wallet3,
    };

    (
        deploy_wallet,
        owner1,
        owner2,
        token_distributor_id.into(),
        f_nft_id.into(),
        nft_id.into(),
        asset_id.into(),
    )
}

pub(crate) async fn wallet_balance(asset_contract: ContractId, wallet: &WalletUnlocked) -> u64 {
    wallet
        .get_asset_balance(&AssetId::new(*asset_contract))
        .await
        .unwrap()
}

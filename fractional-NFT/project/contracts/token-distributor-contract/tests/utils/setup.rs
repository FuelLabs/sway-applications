use fuels::{
    prelude::{
        abigen, launch_custom_provider_and_get_wallets, Contract, ContractId, StorageConfiguration,
        TxParameters, WalletUnlocked, WalletsConfig,
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
    pub(crate) asset: Asset,
    pub(crate) f_nft: FractionalNFT,
    pub(crate) nft: Nft,
    pub(crate) token_distributor: TokenDistributor,
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

    // Get the wallets from that provider
    let wallet1 = wallets.pop().unwrap();
    let wallet2 = wallets.pop().unwrap();
    let wallet3 = wallets.pop().unwrap();

    let token_distributor_id = Contract::deploy(
        TOKEN_DISTRIBUTOR_CONTRACT_BINARY_PATH,
        &wallet1,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(
            TOKEN_DISTRIBUTOR_CONTRACT_STORAGE_PATH.to_string(),
        )),
    )
    .await
    .unwrap();

    let f_nft_id = Contract::deploy(
        FRACTIONAL_NFT_CONTRACT_BINARY_PATH,
        &wallet1,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(
            FRACTIONAL_NFT_CONTRACT_STORAGE_PATH.to_string(),
        )),
    )
    .await
    .unwrap();

    let nft_id = Contract::deploy(
        NFT_CONTRACT_BINARY_PATH,
        &wallet1,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(NFT_CONTRACT_STORAGE_PATH.to_string())),
    )
    .await
    .unwrap();

    let asset_id = Contract::deploy(
        ASSET_CONTRACT_BINARY_PATH,
        &wallet1,
        TxParameters::default(),
        StorageConfiguration::default(),
    )
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

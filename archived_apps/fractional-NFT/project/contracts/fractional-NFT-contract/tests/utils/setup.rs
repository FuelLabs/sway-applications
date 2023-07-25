use fuels::prelude::{
    abigen, launch_custom_provider_and_get_wallets, Contract, ContractId, LoadConfiguration,
    StorageConfiguration, TxParameters, WalletUnlocked, WalletsConfig,
};

abigen!(
    Contract(
        name = "FractionalNFT",
        abi = "./contracts/fractional-NFT-contract/out/debug/fractional-NFT-contract-abi.json"
    ),
    Contract(
        name = "Nft",
        abi = "./contracts/fractional-NFT-contract/tests/artifacts/NFT/out/debug/NFT-1-abi.json"
    )
);

pub(crate) struct Metadata {
    pub(crate) f_nft: FractionalNFT<WalletUnlocked>,
    pub(crate) nft: Nft<WalletUnlocked>,
    pub(crate) wallet: WalletUnlocked,
}

const FRACTIONAL_NFT_CONTRACT_BINARY_PATH: &str = "./out/debug/fractional-NFT-contract.bin";
const FRACTIONAL_NFT_CONTRACT_STORAGE_PATH: &str =
    "./out/debug/fractional-NFT-contract-storage_slots.json";
const NFT_CONTRACT_BINARY_PATH: &str = "./tests/artifacts/NFT/out/debug/NFT-1.bin";
const NFT_CONTRACT_STORAGE_PATH: &str = "./tests/artifacts/NFT/out/debug/NFT-1-storage_slots.json";

pub(crate) async fn defaults() -> u64 {
    10 // supply
}

pub(crate) async fn setup() -> (Metadata, Metadata, Metadata, ContractId, ContractId) {
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

    let fractional_storage_configuration =
        StorageConfiguration::load_from(FRACTIONAL_NFT_CONTRACT_STORAGE_PATH);
    let nft_storage_configuration = StorageConfiguration::load_from(NFT_CONTRACT_STORAGE_PATH);

    let fractional_configuration = LoadConfiguration::default()
        .set_storage_configuration(fractional_storage_configuration.unwrap());
    let nft_configuration =
        LoadConfiguration::default().set_storage_configuration(nft_storage_configuration.unwrap());

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

    let deploy_wallet = Metadata {
        f_nft: FractionalNFT::new(f_nft_id.clone(), wallet1.clone()),
        nft: Nft::new(nft_id.clone(), wallet1.clone()),
        wallet: wallet1,
    };

    let owner1 = Metadata {
        f_nft: FractionalNFT::new(f_nft_id.clone(), wallet2.clone()),
        nft: Nft::new(nft_id.clone(), wallet2.clone()),
        wallet: wallet2,
    };

    let owner2 = Metadata {
        f_nft: FractionalNFT::new(f_nft_id.clone(), wallet3.clone()),
        nft: Nft::new(nft_id.clone(), wallet3.clone()),
        wallet: wallet3,
    };

    (
        deploy_wallet,
        owner1,
        owner2,
        f_nft_id.into(),
        nft_id.into(),
    )
}

use fuels::{
    prelude::{
        abigen, launch_custom_provider_and_get_wallets, AssetConfig, Bech32ContractId, Contract,
        ContractId, LoadConfiguration, StorageConfiguration, TxPolicies, WalletUnlocked,
        WalletsConfig, BASE_ASSET_ID,
    },
    programs::call_utils::TxDependencyExtension,
    types::{AssetId, Identity, Bits256},
    tx::Bytes32,
};
use sha2::{Digest, Sha256};

abigen!(Contract(
    name = "F_NFT",
    abi = "./contracts/f-NFT-contract/out/debug/f-NFT-contract-abi.json"
),
Contract(
    name = "NFT",
    abi = "../../NFT/project/contracts/NFT-contract/out/debug/NFT-contract-abi.json"
),
);

const F_NFT_CONTRACT_BINARY_PATH: &str = "./out/debug/f-NFT-contract.bin";
const NFT_CONTRACT_BINARY_PATH: &str = "../../NFT/project/contracts/NFT-contract/out/debug/NFT-contract.bin";
const F_NFT_CONTRACT_STORAGE_PATH: &str = "./out/debug/f-NFT-contract-storage_slots.json";
const NFT_CONTRACT_STORAGE_PATH: &str = "../../NFT/project/contracts/NFT-contract/out/debug/NFT-contract-storage_slots.json";

pub(crate) struct User {
    pub(crate) f_nft: F_NFT<WalletUnlocked>,
    pub(crate) nft: NFT<WalletUnlocked>,
    pub(crate) wallet: WalletUnlocked,
}

pub(crate) async fn setup_nft(wallet: WalletUnlocked, nft: &NFT<WalletUnlocked>, nft_id: ContractId) -> (AssetId, AssetId, AssetId) {
    let identity = Identity::Address(wallet.address().into());
    let b256_1 = Bytes32::new([1u8; 32]);
    let b256_2 = Bytes32::new([2u8; 32]);
    
    let _ = nft.methods().mint(identity.clone(), Bits256(*Bytes32::zeroed()), 1).append_variable_outputs(1).call().await;
    let _ = nft.methods().mint(identity.clone(), Bits256(*b256_1), 1).append_variable_outputs(1).call().await;
    let _ = nft.methods().mint(identity, Bits256(*b256_2), 1).append_variable_outputs(1).call().await;

    let mut hasher1 = Sha256::new();
    hasher1.update(*nft_id);
    hasher1.update(*Bytes32::zeroed());
    let nft_1 = AssetId::new(hasher1.finalize().try_into().unwrap());

    let mut hasher2 = Sha256::new();
    hasher2.update(*nft_id);
    hasher2.update(*b256_1);
    let nft_2 = AssetId::new(hasher2.finalize().try_into().unwrap());

    let mut hasher3 = Sha256::new();
    hasher3.update(*nft_id);
    hasher3.update(*b256_2);
    let nft_3 = AssetId::new(hasher3.finalize().try_into().unwrap());

    (nft_1, nft_2, nft_3)
}

pub(crate) async fn deploy() -> (
    User,
    User,
    User,
    ContractId,
    ContractId,
) {
    let number_of_coins = 1;
    let coin_amount = 1_000_000;
    let number_of_wallets = 3;

    let base_asset = AssetConfig {
        id: BASE_ASSET_ID,
        num_coins: number_of_coins,
        coin_amount,
    };
    let assets = vec![base_asset];

    let wallet_config = WalletsConfig::new_multiple_assets(number_of_wallets, assets);
    let mut wallets = launch_custom_provider_and_get_wallets(wallet_config, None, None)
        .await
        .unwrap();

    let wallet1 = wallets.pop().unwrap();
    let wallet2 = wallets.pop().unwrap();
    let wallet3 = wallets.pop().unwrap();

    let nft_storage_configuration =
        StorageConfiguration::default().add_slot_overrides_from_file(NFT_CONTRACT_STORAGE_PATH);
    let nft_configuration = LoadConfiguration::default()
        .with_storage_configuration(nft_storage_configuration.unwrap());

    let nft_id = Contract::load_from(NFT_CONTRACT_BINARY_PATH, nft_configuration)
        .unwrap()
        .deploy(&wallet1, TxPolicies::default())
        .await
        .unwrap();

    let f_nft_storage_configuration =
        StorageConfiguration::default().add_slot_overrides_from_file(F_NFT_CONTRACT_STORAGE_PATH);
    let f_nft_configuration = LoadConfiguration::default()
        .with_storage_configuration(f_nft_storage_configuration.unwrap());

    let f_nft_id = Contract::load_from(F_NFT_CONTRACT_BINARY_PATH, f_nft_configuration)
        .unwrap()
        .deploy(&wallet1, TxPolicies::default())
        .await
        .unwrap();

    let user1 = User {
        f_nft: F_NFT::new(
            f_nft_id.clone(),
            wallet1.clone(),
        ),
        nft: NFT::new(
            nft_id.clone(),
            wallet1.clone(),
        ),
        wallet: wallet1,
    };

    let user2 = User {
        f_nft: F_NFT::new(
            f_nft_id.clone(),
            wallet2.clone(),
        ),
        nft: NFT::new(
            nft_id.clone(),
            wallet2.clone(),
        ),
        wallet: wallet2,
    };

    let user3 = User {
        f_nft: F_NFT::new(
            f_nft_id.clone(),
            wallet3.clone(),
        ),
        nft: NFT::new(
            nft_id.clone(),
            wallet3.clone(),
        ),
        wallet: wallet3,
    };

    (
        user1,
        user2,
        user3,
        f_nft_id.into(),
        nft_id.into(),
    )
}

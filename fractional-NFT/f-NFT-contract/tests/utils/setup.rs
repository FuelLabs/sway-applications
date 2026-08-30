use fuels::{
    accounts::ViewOnlyAccount,
    prelude::{
        abigen, launch_custom_provider_and_get_wallets, AssetConfig, Contract, ContractId,
        LoadConfiguration, StorageConfiguration, TxPolicies, WalletUnlocked, WalletsConfig,
    },
    programs::call_utils::TxDependencyExtension,
    types::{AssetId, Bits256, Bytes32, Identity},
};
use sha2::{Digest, Sha256};

abigen!(
    Contract(
        name = "F_NFT",
        abi = "./f-NFT-contract/out/debug/f-NFT-contract-abi.json"
    ),
    Contract(
        name = "NFT",
        abi = "./test-artifacts/out/debug/NFT-contract-abi.json"
    ),
);

const F_NFT_CONTRACT_BINARY_PATH: &str = "./out/debug/f-NFT-contract.bin";
const NFT_CONTRACT_BINARY_PATH: &str = "../test-artifacts/out/debug/NFT-contract.bin";
const F_NFT_CONTRACT_STORAGE_PATH: &str = "./out/debug/f-NFT-contract-storage_slots.json";
const NFT_CONTRACT_STORAGE_PATH: &str =
    "../test-artifacts/out/debug/NFT-contract-storage_slots.json";

pub(crate) struct User {
    pub(crate) f_nft: F_NFT<WalletUnlocked>,
    pub(crate) nft: NFT<WalletUnlocked>,
    pub(crate) wallet: WalletUnlocked,
}

pub(crate) fn defaults(
    wallet: &WalletUnlocked,
    nft_1: AssetId,
    nft_2: AssetId,
    f_nft_id: ContractId,
) -> (Bytes32, Identity, AssetId, AssetId, u64) {
    let vault_sub_id = Bytes32::zeroed();
    let vault_admin = Identity::Address(wallet.address().into());

    let share_asset1_sub_id = get_share_sub_id(vault_sub_id, nft_1);
    let share_asset1 = get_asset_id(share_asset1_sub_id, f_nft_id);

    let share_asset2_sub_id = get_share_sub_id(vault_sub_id, nft_2);
    let share_asset2 = get_asset_id(share_asset2_sub_id, f_nft_id);

    let share_supply = 100_000_000;

    (
        vault_sub_id,
        vault_admin,
        share_asset1,
        share_asset2,
        share_supply,
    )
}

pub(crate) async fn deploy() -> (User, User, ContractId, ContractId) {
    let number_of_coins = 1;
    let coin_amount = 100_000_000;
    let number_of_wallets = 2;

    let base_asset = AssetConfig {
        id: AssetId::zeroed(),
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

    let nft_storage_configuration =
        StorageConfiguration::default().add_slot_overrides_from_file(NFT_CONTRACT_STORAGE_PATH);
    let nft_configuration =
        LoadConfiguration::default().with_storage_configuration(nft_storage_configuration.unwrap());

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
        f_nft: F_NFT::new(f_nft_id.clone(), wallet1.clone()),
        nft: NFT::new(nft_id.clone(), wallet1.clone()),
        wallet: wallet1,
    };

    let user2 = User {
        f_nft: F_NFT::new(f_nft_id.clone(), wallet2.clone()),
        nft: NFT::new(nft_id.clone(), wallet2.clone()),
        wallet: wallet2,
    };

    (user1, user2, f_nft_id.into(), nft_id.into())
}

pub(crate) fn get_asset_id(sub_id: Bytes32, contract: ContractId) -> AssetId {
    let mut hasher = Sha256::new();
    hasher.update(*contract);
    hasher.update(*sub_id);
    AssetId::new(*Bytes32::from(<[u8; 32]>::from(hasher.finalize())))
}

pub(crate) fn get_share_sub_id(vault_sub_id: Bytes32, nft: AssetId) -> Bytes32 {
    let mut hasher = Sha256::new();
    hasher.update(*nft);
    hasher.update(*vault_sub_id);
    Bytes32::from(<[u8; 32]>::from(hasher.finalize()))
}

pub(crate) async fn get_wallet_balance(wallet: &WalletUnlocked, asset: &AssetId) -> u64 {
    wallet.get_asset_balance(asset).await.unwrap()
}

pub(crate) async fn setup_nft(
    wallet: &WalletUnlocked,
    nft: &NFT<WalletUnlocked>,
    nft_id: ContractId,
) -> (AssetId, AssetId) {
    let identity = Identity::Address(wallet.address().into());
    let sub_id_1 = Bytes32::zeroed();
    let sub_id_2 = Bytes32::new([1u8; 32]);

    let _ = nft
        .methods()
        .mint(identity, Some(Bits256(*sub_id_1)), 1)
        .append_variable_outputs(1)
        .call()
        .await;
    let _ = nft
        .methods()
        .mint(identity, Some(Bits256(*sub_id_2)), 1)
        .append_variable_outputs(1)
        .call()
        .await;

    let nft_1 = get_asset_id(sub_id_1, nft_id);
    let nft_2 = get_asset_id(sub_id_2, nft_id);

    (nft_1, nft_2)
}

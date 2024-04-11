use fuels::{
    accounts::ViewOnlyAccount,
    prelude::{
        abigen, launch_custom_provider_and_get_wallets, AssetConfig, Contract, ContractId,
        LoadConfiguration, TxPolicies, WalletUnlocked, WalletsConfig, BASE_ASSET_ID,
    },
    tx::Bytes32,
    types::{Address, AssetId, Bits256, Identity},
};
use sha2::{Digest, Sha256};

abigen!(Contract(
    name = "NFT",
    abi = "./NFT-contract/out/debug/NFT-contract-abi.json"
),);

const NFT_CONTRACT_BINARY_PATH: &str = "./out/debug/NFT-contract.bin";

pub(crate) fn defaults(
    contract_id: ContractId,
    wallet_1: WalletUnlocked,
    wallet_2: WalletUnlocked,
) -> (
    AssetId,
    AssetId,
    AssetId,
    Bits256,
    Bits256,
    Bits256,
    Identity,
    Identity,
) {
    let sub_id_1 = Bytes32::from([1u8; 32]);
    let sub_id_2 = Bytes32::from([2u8; 32]);
    let sub_id_3 = Bytes32::from([3u8; 32]);
    let asset1 = get_asset_id(sub_id_1, contract_id);
    let asset2 = get_asset_id(sub_id_2, contract_id);
    let asset3 = get_asset_id(sub_id_3, contract_id);

    let identity_1 = Identity::Address(Address::from(wallet_1.address()));
    let identity_2 = Identity::Address(Address::from(wallet_2.address()));

    (
        asset1,
        asset2,
        asset3,
        Bits256(*sub_id_1),
        Bits256(*sub_id_2),
        Bits256(*sub_id_3),
        identity_1,
        identity_2,
    )
}

pub(crate) async fn setup() -> (
    WalletUnlocked,
    WalletUnlocked,
    ContractId,
    NFT<WalletUnlocked>,
    NFT<WalletUnlocked>,
) {
    let number_of_coins = 1;
    let coin_amount = 100_000_000;
    let number_of_wallets = 2;

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

    let id = Contract::load_from(NFT_CONTRACT_BINARY_PATH, LoadConfiguration::default())
        .unwrap()
        .deploy(&wallet1, TxPolicies::default())
        .await
        .unwrap();

    let instance_1 = NFT::new(id.clone(), wallet1.clone());
    let instance_2 = NFT::new(id.clone(), wallet2.clone());

    (wallet1, wallet2, id.into(), instance_1, instance_2)
}

pub(crate) fn get_asset_id(sub_id: Bytes32, contract: ContractId) -> AssetId {
    let mut hasher = Sha256::new();
    hasher.update(*contract);
    hasher.update(*sub_id);
    AssetId::new(*Bytes32::from(<[u8; 32]>::from(hasher.finalize())))
}

pub(crate) async fn get_wallet_balance(wallet: &WalletUnlocked, asset: &AssetId) -> u64 {
    wallet.get_asset_balance(asset).await.unwrap()
}

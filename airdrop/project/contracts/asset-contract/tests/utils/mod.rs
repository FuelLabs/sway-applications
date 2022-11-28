use fuels::{contract::contract::CallResponse, prelude::*};

abigen!(
    SimpleAsset,
    "./project/contracts/asset-contract/out/debug/asset-contract-abi.json"
);

pub struct Metadata {
    pub asset_id: ContractId,
    pub simple_asset: SimpleAsset,
    pub wallet: WalletUnlocked,
}

pub mod paths {
    pub const ASSET_CONTRACT_BINARY_PATH: &str = "./out/debug/asset-contract.bin";
    pub const ASSET_CONTRACT_STORAGE_PATH: &str = "./out/debug/asset-contract-storage_slots.json";
}

pub mod abi_calls {

    use super::*;

    pub async fn constructor(
        asset_supply: u64,
        contract: &SimpleAsset,
        minter: Identity,
    ) -> CallResponse<()> {
        contract
            .methods()
            .constructor(asset_supply, minter)
            .call()
            .await
            .unwrap()
    }

    pub async fn mint_to(amount: u64, contract: &SimpleAsset, to: Identity) -> CallResponse<()> {
        contract
            .methods()
            .mint_to(amount, to)
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }
}

pub mod test_helpers {

    use super::*;
    use paths::{ASSET_CONTRACT_BINARY_PATH, ASSET_CONTRACT_STORAGE_PATH};

    pub async fn setup() -> (Metadata, Metadata, u64) {
        let num_wallets = 2;
        let coins_per_wallet = 1;
        let coin_amount = 1000000;
        let mut wallets = launch_custom_provider_and_get_wallets(
            WalletsConfig::new(Some(num_wallets), Some(coins_per_wallet), Some(coin_amount)),
            None,
            None,
        )
        .await;

        let wallet1 = wallets.pop().unwrap();
        let wallet2 = wallets.pop().unwrap();

        let simple_asset_id = Contract::deploy(
            ASSET_CONTRACT_BINARY_PATH,
            &wallet1,
            TxParameters::default(),
            StorageConfiguration::with_storage_path(Some(ASSET_CONTRACT_STORAGE_PATH.to_string())),
        )
        .await
        .unwrap();

        let deployer = Metadata {
            asset_id: ContractId::new(*simple_asset_id.hash()),
            simple_asset: SimpleAsset::new(simple_asset_id.clone(), wallet1.clone()),
            wallet: wallet1,
        };

        let user = Metadata {
            asset_id: ContractId::new(*simple_asset_id.hash()),
            simple_asset: SimpleAsset::new(simple_asset_id, wallet2.clone()),
            wallet: wallet2,
        };

        let total_supply = 100;

        (deployer, user, total_supply)
    }
}

use fuels::{contract::contract::CallResponse, prelude::*};

abigen!(SimpleAsset, "out/debug/simpleasset-abi.json");

pub struct Metadata {
    pub asset_id: ContractId,
    pub simple_asset: SimpleAsset,
    pub wallet: WalletUnlocked,
}

pub mod abi_calls {

    use super::*;

    pub async fn constructor(
        minter: Identity,
        contract: &SimpleAsset,
        asset_supply: u64,
    ) -> CallResponse<()> {
        contract
            .constructor(minter, asset_supply)
            .call()
            .await
            .unwrap()
    }

    pub async fn mint_to(amount: u64, contract: &SimpleAsset, to: Identity) -> CallResponse<()> {
        contract
            .mint_to(amount, to)
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }
}

pub mod test_helpers {

    use super::*;

    pub async fn setup() -> (Metadata, Metadata, u64) {
        let num_wallets = 2;
        let coins_per_wallet = 1;
        let coin_amount = 1000000;
        let mut wallets = launch_custom_provider_and_get_wallets(
            WalletsConfig::new(Some(num_wallets), Some(coins_per_wallet), Some(coin_amount)),
            None,
        )
        .await;

        let wallet1 = wallets.pop().unwrap();
        let wallet2 = wallets.pop().unwrap();

        let simple_asset_id = Contract::deploy(
            "./out/debug/simpleasset.bin",
            &wallet1,
            TxParameters::default(),
            StorageConfiguration::with_storage_path(Some(
                "./out/debug/simpleasset-storage_slots.json".to_string(),
            )),
        )
        .await
        .unwrap();

        let deployer = Metadata {
            asset_id: ContractId::new(*simple_asset_id.hash()),
            simple_asset: SimpleAssetBuilder::new(simple_asset_id.to_string(), wallet1.clone())
                .build(),
            wallet: wallet1.clone(),
        };

        let user = Metadata {
            asset_id: ContractId::new(*simple_asset_id.hash()),
            simple_asset: SimpleAssetBuilder::new(simple_asset_id.to_string(), wallet2.clone())
                .build(),
            wallet: wallet2.clone(),
        };

        let total_supply = 100;

        (deployer, user, total_supply)
    }
}

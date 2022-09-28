use fuels::{contract::contract::CallResponse, prelude::*, tx::ContractId};

abigen!(Asset, "../asset/out/debug/asset-abi.json");

pub mod abi_calls {
    use super::*;

    pub async fn balance(contract: &Asset) -> u64 {
        contract.balance().call().await.unwrap().value
    }

    pub async fn burn_coins(contract: &Asset, amount: u64) -> CallResponse<()> {
        contract.burn_coins(amount).call().await.unwrap()
    }

    pub async fn initialize(contract: &Asset, identity: Identity, amount: u64) -> CallResponse<()> {
        contract.initialize(identity, amount).call().await.unwrap()
    }

    pub async fn mint(contract: &Asset) -> CallResponse<()> {
        contract
            .mint()
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }

    pub async fn mint_amount(contract: &Asset) -> u64 {
        contract.mint_amount().call().await.unwrap().value
    }

    pub async fn mint_coins(contract: &Asset, amount: u64) -> CallResponse<()> {
        contract.mint_coins(amount).call().await.unwrap()
    }

    pub async fn set_mint_amount(contract: &Asset, amount: u64) -> CallResponse<()> {
        contract.set_mint_amount(amount).call().await.unwrap()
    }

    pub async fn asset_balance(
        contract: &Asset,
        call_params: CallParameters,
        asset: ContractId,
    ) -> u64 {
        contract
            .asset_balance(asset)
            .call_params(call_params)
            .call()
            .await
            .unwrap()
            .value
    }

    pub async fn transfer_coins(
        contract: &Asset,
        coins: u64,
        identity: Identity,
    ) -> CallResponse<()> {
        contract
            .transfer_coins(coins, identity)
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }

    pub async fn transfer_asset_to_output(
        contract: &Asset,
        asset_id: ContractId,
        coins: u64,
        identity: Identity,
    ) -> CallResponse<()> {
        contract
            .transfer_asset_to_output(asset_id, coins, identity)
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }
}

pub mod test_helpers {
    use super::*;
    use abi_calls::initialize;

    pub async fn build_contract(contract_id: Bech32ContractId, wallet: WalletUnlocked) -> Asset {
        AssetBuilder::new(contract_id.to_string(), wallet).build()
    }

    pub async fn setup_and_initialize(
    ) -> (WalletUnlocked, WalletUnlocked, u64, Bech32ContractId, Asset) {
        let initial_amount = 1000000000;
        let num_wallets = 2;
        let num_coins = 1;
        let config = WalletsConfig::new(Some(num_wallets), Some(num_coins), Some(initial_amount));
        let mut wallets = launch_custom_provider_and_get_wallets(config, None).await;
        let owner = wallets.pop().unwrap();
        let minter = wallets.pop().unwrap();

        let asset_contract_id = Contract::deploy(
            "../asset/out/debug/asset.bin",
            &owner,
            TxParameters::default(),
            StorageConfiguration::default(),
        )
        .await
        .unwrap();
        let asset_instance =
            AssetBuilder::new(asset_contract_id.to_string(), owner.clone()).build();

        let mint_amount = 10000;
        initialize(
            &asset_instance,
            Identity::Address(Address::from(owner.address())),
            mint_amount,
        )
        .await;

        (
            owner.clone(),
            minter.clone(),
            mint_amount,
            asset_contract_id,
            asset_instance,
        )
    }
}

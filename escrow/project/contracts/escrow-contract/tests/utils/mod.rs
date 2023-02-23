use fuels::{
    prelude::{
        abigen, launch_custom_provider_and_get_wallets, AssetId, Bech32Address, CallParameters,
        Configurables, Contract, ContractId, Salt, StorageConfiguration, TxParameters,
        WalletUnlocked, WalletsConfig,
    },
    programs::call_response::FuelCallResponse,
    types::Identity,
};

abigen!(
    Contract(
        name = "Escrow",
        abi = "./contracts/escrow-contract/out/debug/escrow-contract-abi.json"
    ),
    Contract(
        name = "MyAsset",
        abi = "./contracts/escrow-contract/tests/artifacts/asset/out/debug/asset-abi.json"
    )
);

pub(crate) struct Defaults {
    pub(crate) asset: MyAsset,
    pub(crate) asset_amount: u64,
    pub(crate) asset_id: ContractId,
    pub(crate) deadline: u64,
}

pub(crate) struct User {
    pub(crate) contract: Escrow,
    pub(crate) wallet: WalletUnlocked,
}

pub(crate) mod paths {
    pub(crate) const ASSET_CONTRACT_BINARY_PATH: &str =
        "./tests/artifacts/asset/out/debug/asset.bin";
    pub(crate) const ASSET_CONTRACT_STORAGE_PATH: &str =
        "./tests/artifacts/asset/out/debug/asset-storage_slots.json";
    pub(crate) const ESCROW_CONTRACT_BINARY_PATH: &str = "./out/debug/escrow-contract.bin";
    pub(crate) const ESCROW_CONTRACT_STORAGE_PATH: &str =
        "./out/debug/escrow-contract-storage_slots.json";
}

pub(crate) mod abi_calls {

    use super::*;

    pub(crate) async fn accept_arbiter(contract: &Escrow, identifier: u64) -> FuelCallResponse<()> {
        contract
            .methods()
            .accept_arbiter(identifier)
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }

    pub(crate) async fn create_escrow(
        amount: u64,
        arbiter: &Arbiter,
        asset: &ContractId,
        assets: Vec<Asset>,
        buyer: &Bech32Address,
        contract: &Escrow,
        deadline: u64,
    ) -> FuelCallResponse<()> {
        let tx_params = TxParameters::new(None, Some(1_000_000), None);
        let call_params =
            CallParameters::new(Some(amount), Some(AssetId::from(**asset)), Some(1_000_000));

        contract
            .methods()
            .create_escrow(
                arbiter.clone(),
                assets,
                Identity::Address(buyer.into()),
                deadline,
            )
            .tx_params(tx_params)
            .call_params(call_params)
            .unwrap()
            .call()
            .await
            .unwrap()
    }

    pub(crate) async fn deposit(
        amount: u64,
        asset: &ContractId,
        contract: &Escrow,
        identifier: u64,
    ) -> FuelCallResponse<()> {
        let tx_params = TxParameters::new(None, Some(1_000_000), None);
        let call_params =
            CallParameters::new(Some(amount), Some(AssetId::from(**asset)), Some(1_000_000));

        contract
            .methods()
            .deposit(identifier)
            .tx_params(tx_params)
            .call_params(call_params)
            .unwrap()
            .call()
            .await
            .unwrap()
    }

    pub(crate) async fn dispute(contract: &Escrow, identifier: u64) -> FuelCallResponse<()> {
        contract.methods().dispute(identifier).call().await.unwrap()
    }

    pub(crate) async fn propose_arbiter(
        arbiter: Arbiter,
        contract: &Escrow,
        identifier: u64,
    ) -> FuelCallResponse<()> {
        let tx_params = TxParameters::new(None, Some(1_000_000), None);
        let call_params = CallParameters::new(
            Some(arbiter.fee_amount),
            Some(AssetId::from(*arbiter.asset)),
            Some(1_000_000),
        );

        contract
            .methods()
            .propose_arbiter(arbiter, identifier)
            .tx_params(tx_params)
            .call_params(call_params)
            .unwrap()
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }

    pub(crate) async fn resolve_dispute(
        contract: &Escrow,
        identifier: u64,
        payment_amount: u64,
        user: &Bech32Address,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .resolve_dispute(identifier, payment_amount, Identity::Address(user.into()))
            .append_variable_outputs(4)
            .call()
            .await
            .unwrap()
    }

    pub(crate) async fn return_deposit(contract: &Escrow, identifier: u64) -> FuelCallResponse<()> {
        contract
            .methods()
            .return_deposit(identifier)
            .append_variable_outputs(3)
            .call()
            .await
            .unwrap()
    }

    pub(crate) async fn take_payment(contract: &Escrow, identifier: u64) -> FuelCallResponse<()> {
        contract
            .methods()
            .take_payment(identifier)
            .append_variable_outputs(3)
            .call()
            .await
            .unwrap()
    }

    pub(crate) async fn transfer_to_seller(
        contract: &Escrow,
        identifier: u64,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .transfer_to_seller(identifier)
            .append_variable_outputs(3)
            .call()
            .await
            .unwrap()
    }

    pub(crate) async fn withdraw_collateral(
        contract: &Escrow,
        identifier: u64,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .withdraw_collateral(identifier)
            .append_variable_outputs(2)
            .call()
            .await
            .unwrap()
    }

    pub(crate) async fn arbiter_proposal(contract: &Escrow, identifier: u64) -> Option<Arbiter> {
        contract
            .methods()
            .arbiter_proposal(identifier)
            .call()
            .await
            .unwrap()
            .value
    }

    pub(crate) async fn assets(contract: &Escrow, identifier: u64) -> Option<Asset> {
        contract
            .methods()
            .assets(identifier)
            .call()
            .await
            .unwrap()
            .value
    }

    pub(crate) async fn escrows(contract: &Escrow, identifier: u64) -> Option<EscrowInfo> {
        contract
            .methods()
            .escrows(identifier)
            .call()
            .await
            .unwrap()
            .value
    }

    pub(crate) async fn escrow_count(contract: &Escrow) -> u64 {
        contract
            .methods()
            .escrow_count()
            .call()
            .await
            .unwrap()
            .value
    }
}

pub(crate) mod test_helpers {

    use super::*;
    use paths::{
        ASSET_CONTRACT_BINARY_PATH, ASSET_CONTRACT_STORAGE_PATH, ESCROW_CONTRACT_BINARY_PATH,
        ESCROW_CONTRACT_STORAGE_PATH,
    };

    pub(crate) async fn asset_amount(asset: &ContractId, wallet: &WalletUnlocked) -> u64 {
        wallet
            .clone()
            .get_asset_balance(&AssetId::from(**asset))
            .await
            .unwrap()
    }

    pub(crate) async fn create_arbiter(
        address: &Bech32Address,
        asset: ContractId,
        fee_amount: u64,
    ) -> Arbiter {
        Arbiter {
            address: Identity::Address(address.into()),
            asset,
            fee_amount,
        }
    }

    pub(crate) async fn create_asset(amount: u64, id: ContractId) -> Asset {
        Asset { amount, id }
    }

    pub(crate) async fn create_asset_with_salt(
        salt: [u8; 32],
        wallet: WalletUnlocked,
    ) -> (ContractId, MyAsset) {
        let asset_id = Contract::deploy_with_parameters(
            ASSET_CONTRACT_BINARY_PATH,
            &wallet,
            TxParameters::default(),
            StorageConfiguration::with_storage_path(Some(ASSET_CONTRACT_STORAGE_PATH.to_string())),
            Configurables::default(),
            Salt::from(salt),
        )
        .await
        .unwrap();

        (asset_id.clone().into(), MyAsset::new(asset_id, wallet))
    }

    pub(crate) async fn mint(address: &Bech32Address, amount: u64, contract: &MyAsset) {
        contract
            .methods()
            .mint_and_send_to_address(amount, address.into())
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap();
    }

    pub(crate) async fn setup() -> (User, User, User, Defaults) {
        let number_of_wallets = 4;
        let coins_per_wallet = 1;
        let amount_per_coin = 1_000_000;

        let config = WalletsConfig::new(
            Some(number_of_wallets),
            Some(coins_per_wallet),
            Some(amount_per_coin),
        );

        let mut wallets = launch_custom_provider_and_get_wallets(config, None, None).await;

        let deployer_wallet = wallets.pop().unwrap();
        let arbiter_wallet = wallets.pop().unwrap();
        let buyer_wallet = wallets.pop().unwrap();
        let seller_wallet = wallets.pop().unwrap();

        let escrow_id = Contract::deploy(
            ESCROW_CONTRACT_BINARY_PATH,
            &deployer_wallet,
            TxParameters::default(),
            StorageConfiguration::with_storage_path(Some(ESCROW_CONTRACT_STORAGE_PATH.to_string())),
        )
        .await
        .unwrap();

        let asset_id = Contract::deploy(
            ASSET_CONTRACT_BINARY_PATH,
            &deployer_wallet,
            TxParameters::default(),
            StorageConfiguration::with_storage_path(Some(ASSET_CONTRACT_STORAGE_PATH.to_string())),
        )
        .await
        .unwrap();

        let asset = MyAsset::new(asset_id.clone(), deployer_wallet);

        let arbiter = User {
            contract: Escrow::new(escrow_id.clone(), arbiter_wallet.clone()),
            wallet: arbiter_wallet,
        };

        let buyer = User {
            contract: Escrow::new(escrow_id.clone(), buyer_wallet.clone()),
            wallet: buyer_wallet,
        };

        let seller = User {
            contract: Escrow::new(escrow_id.clone(), seller_wallet.clone()),
            wallet: seller_wallet,
        };

        let defaults = Defaults {
            asset,
            asset_id: asset_id.into(),
            asset_amount: 100,
            deadline: 100,
        };

        (arbiter, buyer, seller, defaults)
    }
}

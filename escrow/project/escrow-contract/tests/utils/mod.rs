use fuels::{contract::contract::CallResponse, prelude::*};

abigen!(
    Escrow,
    "project/escrow-contract/out/debug/escrow-contract-abi.json"
);
abigen!(
    MyAsset,
    "project/escrow-contract/tests/artifacts/asset/out/debug/asset-abi.json"
);

pub struct Defaults {
    pub asset: MyAsset,
    pub asset_amount: u64,
    pub asset_id: ContractId,
    pub deadline: u64,
}

pub struct User {
    pub contract: Escrow,
    pub wallet: WalletUnlocked,
}

pub mod paths {
    pub const ASSET_BINARY: &str = "./tests/artifacts/asset/out/debug/asset.bin";
    pub const ASSET_STORAGE: &str = "./tests/artifacts/asset/out/debug/asset-storage_slots.json";
    pub const CONTRACT_BINARY: &str = "./out/debug/escrow-contract.bin";
    pub const CONTRACT_STORAGE: &str = "./out/debug/escrow-contract-storage_slots.json";
}

pub mod abi_calls {

    use super::*;

    pub async fn accept_arbiter(contract: &Escrow, identifier: u64) -> CallResponse<()> {
        contract
            .methods()
            .accept_arbiter(identifier)
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }

    pub async fn create_escrow(
        amount: u64,
        arbiter: &Arbiter,
        asset: &ContractId,
        assets: Vec<Asset>,
        buyer: &Bech32Address,
        contract: &Escrow,
        deadline: u64,
    ) -> CallResponse<()> {
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
            .call()
            .await
            .unwrap()
    }

    pub async fn deposit(
        amount: u64,
        asset: &ContractId,
        contract: &Escrow,
        identifier: u64,
    ) -> CallResponse<()> {
        let tx_params = TxParameters::new(None, Some(1_000_000), None);
        let call_params =
            CallParameters::new(Some(amount), Some(AssetId::from(**asset)), Some(1_000_000));

        contract
            .methods()
            .deposit(identifier)
            .tx_params(tx_params)
            .call_params(call_params)
            .call()
            .await
            .unwrap()
    }

    pub async fn dispute(contract: &Escrow, identifier: u64) -> CallResponse<()> {
        contract.methods().dispute(identifier).call().await.unwrap()
    }

    pub async fn propose_arbiter(
        arbiter: Arbiter,
        contract: &Escrow,
        identifier: u64,
    ) -> CallResponse<()> {
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
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }

    pub async fn resolve_dispute(
        contract: &Escrow,
        identifier: u64,
        payment_amount: u64,
        user: &Bech32Address,
    ) -> CallResponse<()> {
        contract
            .methods()
            .resolve_dispute(identifier, payment_amount, Identity::Address(user.into()))
            .append_variable_outputs(4)
            .call()
            .await
            .unwrap()
    }

    pub async fn return_deposit(contract: &Escrow, identifier: u64) -> CallResponse<()> {
        contract
            .methods()
            .return_deposit(identifier)
            .append_variable_outputs(3)
            .call()
            .await
            .unwrap()
    }

    pub async fn take_payment(contract: &Escrow, identifier: u64) -> CallResponse<()> {
        contract
            .methods()
            .take_payment(identifier)
            .append_variable_outputs(3)
            .call()
            .await
            .unwrap()
    }

    pub async fn transfer_to_seller(contract: &Escrow, identifier: u64) -> CallResponse<()> {
        contract
            .methods()
            .transfer_to_seller(identifier)
            .append_variable_outputs(3)
            .call()
            .await
            .unwrap()
    }

    pub async fn withdraw_collateral(contract: &Escrow, identifier: u64) -> CallResponse<()> {
        contract
            .methods()
            .withdraw_collateral(identifier)
            .append_variable_outputs(2)
            .call()
            .await
            .unwrap()
    }
}

pub mod test_helpers {

    use super::*;

    pub async fn asset_amount(asset: &ContractId, wallet: &WalletUnlocked) -> u64 {
        wallet
            .clone()
            .get_asset_balance(&AssetId::from(**asset))
            .await
            .unwrap()
    }

    pub async fn create_arbiter(
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

    pub async fn create_asset(amount: u64, id: ContractId) -> Asset {
        Asset { amount, id }
    }

    pub async fn create_asset_with_salt(
        salt: [u8; 32],
        wallet: WalletUnlocked,
    ) -> (ContractId, MyAsset) {
        let asset_id = Contract::deploy_with_parameters(
            paths::ASSET_BINARY,
            &wallet,
            TxParameters::default(),
            StorageConfiguration::with_storage_path(Some(paths::ASSET_STORAGE.to_string())),
            Salt::from(salt),
        )
        .await
        .unwrap();

        (
            asset_id.clone().into(),
            MyAsset::new(asset_id.clone(), wallet.clone()),
        )
    }

    pub async fn mint(address: &Bech32Address, amount: u64, contract: &MyAsset) {
        contract
            .methods()
            .mint_and_send_to_address(amount, address.into())
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap();
    }

    pub async fn setup() -> (User, User, User, Defaults) {
        let num_wallets = 4;
        let coins_per_wallet = 1;
        let amount_per_coin = 1_000_000;

        let config = WalletsConfig::new(
            Some(num_wallets),
            Some(coins_per_wallet),
            Some(amount_per_coin),
        );

        let mut wallets = launch_custom_provider_and_get_wallets(config, None, None).await;

        let deployer_wallet = wallets.pop().unwrap();
        let arbiter_wallet = wallets.pop().unwrap();
        let buyer_wallet = wallets.pop().unwrap();
        let seller_wallet = wallets.pop().unwrap();

        let escrow_id = Contract::deploy(
            paths::CONTRACT_BINARY,
            &deployer_wallet,
            TxParameters::default(),
            StorageConfiguration::with_storage_path(Some(paths::CONTRACT_STORAGE.to_string())),
        )
        .await
        .unwrap();

        let asset_id = Contract::deploy(
            paths::ASSET_BINARY,
            &deployer_wallet,
            TxParameters::default(),
            StorageConfiguration::with_storage_path(Some(paths::ASSET_STORAGE.to_string())),
        )
        .await
        .unwrap();

        let asset = MyAsset::new(asset_id.clone(), deployer_wallet.clone());

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
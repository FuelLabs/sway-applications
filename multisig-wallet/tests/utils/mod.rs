use fuels::{contract::contract::CallResponse, prelude::*};

// Load abi from json
abigen!(Multisig, "out/debug/multisig-wallet-abi.json");
abigen!(MyAsset, "tests/artifacts/asset/out/debug/asset-abi.json");

pub struct AssetWrapper {
    pub contract: MyAsset,
    pub id: ContractId,
}

pub struct MultisigWrapper {
    pub contract: Multisig,
    pub id: ContractId,
}

pub struct Wallets {
    pub users: [LocalWallet; 3],
}

pub mod abi_calls {

    use super::*;

    pub async fn balance(asset_id: ContractId, contract: &Multisig) -> u64 {
        contract.balance(asset_id).call().await.unwrap().value
    }

    pub async fn constructor(contract: &Multisig, users: Vec<User>, threshold: u64) {
        contract
            .constructor(users, threshold)
            .call()
            .await
            .unwrap();
    }

    pub async fn execute_transaction(
        contract: &Multisig,
        to: Identity,
        value: u64,
        data: Vec<u64>,
        signatures: Vec<B512>,
    ) -> CallResponse<()> {
        contract
            .execute_transaction(to, value, data, signatures)
            .call()
            .await
            .unwrap()
    }

    pub async fn transfer(
        contract: &Multisig,
        to: Identity,
        asset_id: ContractId,
        value: u64,
        data: Vec<u64>,
        signatures: Vec<B512>,
    ) -> CallResponse<()> {
        contract
            .transfer(to, asset_id, value, data, signatures)
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }

    pub async fn nonce(contract: &Multisig) -> CallResponse<u64> {
        contract.nonce().call().await.unwrap()
    }

    pub async fn owner(contract: &Multisig, user: Address) -> CallResponse<Owner> {
        contract.owner(user).call().await.unwrap()
    }

    pub async fn transaction_hash(
        contract: &Multisig,
        to: Identity,
        value: u64,
        data: Vec<u64>,
        nonce: u64,
    ) -> CallResponse<[u8; 32]> {
        contract
            .transaction_hash(to, value, data, nonce)
            .call()
            .await
            .unwrap()
    }
    
}

pub mod test_helpers {

    use super::*;

    pub async fn deposit(
        amount: u64,
        asset_id: ContractId,
        contract_id: ContractId,
        wallet: &LocalWallet,
    ) {
        wallet
            .force_transfer_to_contract(&contract_id.into(), amount, AssetId::new(*asset_id), TxParameters::default())
            .await
            .unwrap();
    }

    pub async fn mint(amount: u64, contract: &MyAsset, wallet: &LocalWallet) {
        contract
            .mint_and_send_to_address(amount, wallet.address().into())
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap();
    }

    pub async fn setup() -> (MultisigWrapper, Wallets, AssetWrapper) {
        let num_wallets = 4;
        let coins_per_wallet = 1;
        let amount_per_coin = 1_000_000;

        let mut wallets = launch_custom_provider_and_get_wallets(
            WalletsConfig::new(
                Some(num_wallets),
                Some(coins_per_wallet),
                Some(amount_per_coin),
            ),
            None,
        )
        .await;

        let wallet1 = wallets.pop().unwrap();
        let wallet2 = wallets.pop().unwrap();
        let wallet3 = wallets.pop().unwrap();
        let deployer = wallets.pop().unwrap();

        let id = Contract::deploy(
            "./out/debug/multisig-wallet.bin",
            &deployer,
            TxParameters::default(),
            StorageConfiguration::with_storage_path(Some(
                "./out/debug/multisig-wallet-storage_slots.json".to_string(),
            )),
        )
        .await
        .unwrap();

        let asset_id = Contract::deploy(
            "./tests/artifacts/asset/out/debug/asset.bin",
            &deployer,
            TxParameters::default(),
            StorageConfiguration::default(),
        )
        .await
        .unwrap();

        let asset = MyAssetBuilder::new(asset_id.to_string(), deployer.clone()).build();
        let multisig = MultisigBuilder::new(id.to_string(), wallet1.clone()).build();
        let users = [wallet1, wallet2, wallet3];

        (
            MultisigWrapper { contract: multisig, id: id.into() },
            Wallets { users },
            AssetWrapper { contract: asset, id: asset_id.into() },
        )
    }
}

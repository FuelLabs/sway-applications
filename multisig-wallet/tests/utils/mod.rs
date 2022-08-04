use fuels::{contract::contract::CallResponse, prelude::*};

// Load abi from json
abigen!(Multisig, "out/debug/multisig-wallet-abi.json");

pub mod abi_calls {

    use super::*;

    pub async fn balance(contract: &Multisig, asset_id: ContractId) -> CallResponse<u64> {
        contract.balance(asset_id).call().await.unwrap()
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

    pub async fn setup() -> (Multisig, LocalWallet, LocalWallet, LocalWallet) {
        let mut wallets = launch_custom_provider_and_get_wallets(
            WalletsConfig {
                num_wallets: 3,
                coins_per_wallet: 1,
                coin_amount: 1_000_000,
            },
            None,
        )
        .await;

        let wallet1 = wallets.pop().unwrap();
        let wallet2 = wallets.pop().unwrap();
        let wallet3 = wallets.pop().unwrap();

        let id = Contract::deploy(
            "./out/debug/multisig-wallet.bin",
            &wallet1,
            TxParameters::default(),
            StorageConfiguration::with_storage_path(Some(
                "./out/debug/multisig-wallet-storage_slots.json".to_string(),
            )),
        )
        .await
        .unwrap();

        (
            MultisigBuilder::new(id.to_string(), wallet1.clone()).build(),
            wallet1,
            wallet2,
            wallet3,
        )
    }
}

use fuels::prelude::*;
use fuels_abigen_macro::abigen;

// Load abi from json
abigen!(Multisig, "out/debug/multisig-wallet-abi.json");

pub mod test_helpers {

    use super::*;

    pub async fn setup() -> (Multisig, LocalWallet, LocalWallet, LocalWallet) {
        let num_wallets = 3;
        let coins_per_wallet = 1;
        let amount_per_coin = 1_000_000;

        let config = WalletsConfig::new(
            Some(num_wallets),
            Some(coins_per_wallet),
            Some(amount_per_coin),
        );

        let mut wallets = launch_provider_and_get_wallets(config).await;

        let wallet1 = wallets.pop().unwrap();
        let wallet2 = wallets.pop().unwrap();
        let wallet3 = wallets.pop().unwrap();

        let id = Contract::deploy(
            "./out/debug/multisig-wallet.bin",
            &wallet1,
            TxParameters::default(),
        )
        .await
        .unwrap();

        (
            Multisig::new(id.to_string(), wallet1.clone()),
            wallet1,
            wallet2,
            wallet3,
        )
    }
}

pub mod abi_calls {

    use super::*;

    pub async fn constructor(contract: &Multisig, users: &[User], threshold: u64) {
        contract
            .constructor(*users, threshold)
            .call()
            .await
            .unwrap();
    }

    pub async fn execute_transaction(
        contract: &Multisig,
        to: Identity,
        value: u64,
        data: &[u64],
        signatures: &[B512],
    ) -> CallRepsponse<()> {
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
        data: &[u64],
        signatures: &[B512],
    ) -> CallRepsponse<()> {
        contract
            .transfer(to, asset_id, value, data, signatures)
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }

    pub async fn owner(contract: &Multisig, user: Address) -> CallRepsponse<bool> {
        contract.owner(user).call().await.unwrap()
    }

    pub async fn balance(contract: &Multisig, asset_id: ContractId) -> CallRepsponse<u64> {
        contract.balance(asset_id).call().await.unwrap()
    }

    pub async fn transaction_hash(
        contract: &Multisig,
        to: Identity,
        value: u64,
        data: &[u64],
        nonce: u64,
    ) -> CallRepsponse<b256> {
        contract
            .transaction_hash(to, value, data, nonce)
            .call()
            .await
            .unwrap()
    }

    pub async fn nonce(contract: &Multisig) -> CallRepsponse<u64> {
        contract.nonce().call().await.unwrap()
    }
}

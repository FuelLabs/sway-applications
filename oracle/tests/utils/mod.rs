use fuels::{contract::contract::CallResponse, prelude::*, signers::wallet::Wallet};

abigen!(Oracle, "out/debug/oracle-abi.json");

pub struct Metadata {
    pub oracle: Oracle,
    pub wallet: Wallet,
}

pub mod abi_calls {
    use super::*;

    pub async fn constructor(contract: &Oracle, owner: Identity) -> CallResponse<()> {
        contract.constructor(owner).call().await.unwrap()
    }

    pub async fn price(contract: &Oracle) -> u64 {
        contract.price().call().await.unwrap().value
    }

    pub async fn owner(contract: &Oracle) -> Option {
        contract.owner().call().await.unwrap().value
    }

    pub async fn set_price(contract: &Oracle, new_price: u64) -> CallResponse<()> {
        contract.set_price(new_price).call().await.unwrap()
    }
}

pub mod test_helpers {
    use super::*;

    pub async fn setup() -> Metadata {
        let wallet = launch_provider_and_get_wallet().await;

        let oracle_id = Contract::deploy(
            "./out/debug/oracle.bin",
            &wallet,
            TxParameters::default(),
            StorageConfiguration::default(),
        )
        .await
        .unwrap();

        let user = Metadata {
            oracle: OracleBuilder::new(oracle_id.to_string(), wallet.clone()).build(),
            wallet,
        };

        user
    }
}

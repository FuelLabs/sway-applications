use fuels::{contract::contract::CallResponse, prelude::*, signers::wallet::Wallet};

abigen!(Oracle, "out/debug/oracle-abi.json");

pub struct Metadata {
    pub oracle: Oracle,
    pub wallet: Wallet,
}

pub mod abi_calls {
    use super::*;

    pub async fn price(contract: &Oracle) -> Option<u64> {
        contract.price().call().await.unwrap()
    }

    pub async fn owner(contract: &Oracle) -> Identity {
        contract.owner().call().await.unwrap().value
    }

    pub async fn set_price(contract: &Oracle, new_price: Option<u64>) -> CallResponse<()> {
        contract.set_price(new_price).call().await.unwrap()
    }
}

pub mod test_helpers {
    use super::*;

    pub async fn setup() -> (Metadata, Vec<Wallet>) {
        let wallets = launch_custom_provider_and_get_wallets(WalletsConfig::default(), None).await;

        let oracle_id = Contract::deploy(
            "./out/debug/oracle.bin",
            &wallets[0],
            TxParameters::default(),
            StorageConfiguration::default(),
        )
        .await
        .unwrap();

        let user = Metadata {
            oracle: OracleBuilder::new(oracle_id.to_string(), wallets[0].clone()).build(),
            wallet: wallets[0].clone(),
        };

        (user, wallets)
    }
}

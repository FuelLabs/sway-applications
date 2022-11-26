use fuels::{contract::contract::CallResponse, prelude::*, signers::wallet::Wallet};

abigen!(
    Oracle,
    "./project/oracle-contract/out/debug/oracle-contract-abi.json"
);

pub struct Metadata {
    pub oracle: Oracle,
    pub wallet: Wallet,
}

pub mod abi_calls {
    use super::*;

    pub async fn owner(contract: &Oracle) -> Identity {
        contract.methods().owner().call().await.unwrap().value
    }

    pub async fn price(contract: &Oracle) -> u64 {
        contract.methods().price().call().await.unwrap().value
    }

    pub async fn set_price(contract: &Oracle, new_price: u64) -> CallResponse<()> {
        contract
            .methods()
            .set_price(new_price)
            .call()
            .await
            .unwrap()
    }
}

pub mod paths {
    pub const ORACLE_CONTRACT_BINARY_PATH: &str =
        "../oracle-contract/out/debug/oracle-contract.bin";
}

pub mod test_helpers {

    use super::*;
    use paths::ORACLE_CONTRACT_BINARY_PATH;

    pub async fn setup() -> (Metadata, Vec<WalletUnlocked>) {
        let wallets =
            launch_custom_provider_and_get_wallets(WalletsConfig::default(), None, None).await;
        let oracle_id = Contract::deploy(
            ORACLE_CONTRACT_BINARY_PATH,
            &wallets[0],
            TxParameters::default(),
            StorageConfiguration::default(),
        )
        .await
        .unwrap();

        let user = Metadata {
            oracle: Oracle::new(oracle_id.clone(), wallets[0].clone()),
            wallet: wallets[0].clone().lock(),
        };

        (user, wallets)
    }
}

use fuels::{contract::contract::CallResponse, prelude::*};

abigen!(SimpleToken, "out/debug/simpletoken-abi.json");

pub struct Metadata {
    pub asset_id: ContractId,
    pub simple_token: SimpleToken,
    pub wallet: LocalWallet,
}

pub mod abi_calls {

    use super::*;

    pub async fn constructor(
        minter: Identity, 
        contract: &SimpleToken,
        token_supply: u64
    ) -> CallResponse<()> {
        contract.constructor(minter, token_supply).call().await.unwrap()
    }

    pub async fn mint_to(
        amount: u64, 
        contract: &SimpleToken,
        to: Identity
    ) -> CallResponse<()> {
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

    pub async fn setup() -> Metadata {
        let wallet = launch_provider_and_get_wallet().await;

        let simple_token_id = Contract::deploy(
            "./out/debug/simpletoken.bin",
            &wallet,
            TxParameters::default(),
            StorageConfiguration::with_storage_path(Some(
                "./out/debug/simpletoken-storage_slots.json".to_string(),
            )),
        )
        .await
        .unwrap();

        let deployer = Metadata {
            asset_id: ContractId::new(*simple_token_id.hash()),
            simple_token: SimpleTokenBuilder::new(simple_token_id.to_string(), wallet.clone()).build(),
            wallet: wallet.clone()
        };

        deployer
    }
}

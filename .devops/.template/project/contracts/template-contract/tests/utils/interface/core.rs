use fuels::{prelude::WalletUnlocked, programs::call_response::FuelCallResponse};

use crate::utils::setup::Template;

pub async fn template(contract: &Template<WalletUnlocked>) -> FuelCallResponse<()> {
    contract.methods().template().call().await.unwrap()
}

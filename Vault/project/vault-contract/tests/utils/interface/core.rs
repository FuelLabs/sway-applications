use fuels::contract::call_response::FuelCallResponse;

use crate::utils::setup::Vault;

pub async fn template(contract: &Vault) -> FuelCallResponse<()> {
    contract.methods().template().call().await.unwrap()
}

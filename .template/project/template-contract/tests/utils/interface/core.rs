use fuels::contract::call_response::FuelCallResponse;

use crate::utils::setup::Template;

pub async fn template(contract: &Template) -> FuelCallResponse<()> {
    contract.methods().template().call().await.unwrap()
}

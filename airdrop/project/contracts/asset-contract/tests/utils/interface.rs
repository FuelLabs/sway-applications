use crate::utils::setup::SimpleAsset;
use fuels::{programs::call_response::FuelCallResponse, types::Identity};

pub(crate) async fn constructor(
    asset_supply: u64,
    contract: &SimpleAsset,
    minter: Identity,
) -> FuelCallResponse<()> {
    contract
        .methods()
        .constructor(asset_supply, minter)
        .call()
        .await
        .unwrap()
}

pub(crate) async fn mint_to(
    amount: u64,
    contract: &SimpleAsset,
    to: Identity,
) -> FuelCallResponse<()> {
    contract
        .methods()
        .mint_to(amount, to)
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap()
}

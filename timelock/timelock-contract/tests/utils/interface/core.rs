use crate::utils::setup::{Asset, Timelock};
use fuels::{
    accounts::wallet::WalletUnlocked,
    prelude::Bytes,
    programs::call_response::FuelCallResponse,
    types::{Bits256, Identity},
};
use fuels::programs::call_utils::TxDependencyExtension;

pub async fn cancel(contract: &Timelock<WalletUnlocked>, id: Bits256) -> FuelCallResponse<()> {
    contract.methods().cancel(id).call().await.unwrap()
}

pub async fn execute(
    contract: &Timelock<WalletUnlocked>,
    recipient: &Identity,
    asset: Option<Asset>,
    data: Bytes,
    timestamp: u64,
) -> FuelCallResponse<()> {
    contract
        .methods()
        .execute(recipient.clone(), asset, Some(data), timestamp)
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap()
}

pub async fn queue(
    contract: &Timelock<WalletUnlocked>,
    recipient: &Identity,
    asset: Option<Asset>,
    data: Bytes,
    timestamp: u64,
) -> FuelCallResponse<()> {
    contract
        .methods()
        .queue(recipient.clone(), asset, Some(data), timestamp)
        .call()
        .await
        .unwrap()
}

use crate::utils::setup::{Asset, ExecutionRange, Timelock};
use fuels::{
    accounts::wallet::WalletUnlocked,
    prelude::{Bytes, AssetId},
    programs::call_response::FuelCallResponse,
    types::{Bits256, Identity},
};

pub async fn balance(
    contract: &Timelock<WalletUnlocked>,
    asset_id: AssetId,
) -> FuelCallResponse<u64> {
    contract.methods().balance(asset_id).call().await.unwrap()
}

pub async fn delays(contract: &Timelock<WalletUnlocked>) -> FuelCallResponse<(u64, u64)> {
    contract.methods().delays().call().await.unwrap()
}

pub async fn queued(
    contract: &Timelock<WalletUnlocked>,
    id: Bits256,
) -> FuelCallResponse<Option<ExecutionRange>> {
    contract.methods().queued(id).call().await.unwrap()
}

pub async fn transaction_hash(
    contract: &Timelock<WalletUnlocked>,
    recipient: &Identity,
    asset: Option<Asset>,
    data: Bytes,
    timestamp: u64,
) -> FuelCallResponse<Bits256> {
    contract
        .methods()
        .transaction_hash(recipient.clone(), asset, Some(data), timestamp)
        .call()
        .await
        .unwrap()
}

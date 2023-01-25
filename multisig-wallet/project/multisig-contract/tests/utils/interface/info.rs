use fuels::{contract::call_response::FuelCallResponse, prelude::*, tx::ContractId};

use crate::utils::setup::MultiSig;

pub async fn approval_weight(contract: &MultiSig, user: Bits256) -> FuelCallResponse<u64> {
    contract
        .methods()
        .approval_weight(user)
        .call()
        .await
        .unwrap()
}

pub async fn balance(contract: &MultiSig, asset_id: ContractId) -> FuelCallResponse<u64> {
    contract.methods().balance(asset_id).call().await.unwrap()
}

pub async fn nonce(contract: &MultiSig) -> FuelCallResponse<u64> {
    contract.methods().nonce().call().await.unwrap()
}

pub async fn threshold(contract: &MultiSig) -> FuelCallResponse<u64> {
    contract.methods().threshold().call().await.unwrap()
}

pub async fn transaction_hash(
    contract: &MultiSig,
    to: Identity,
    value: u64,
    data: Bits256,
    nonce: u64,
) -> FuelCallResponse<Bits256> {
    contract
        .methods()
        .transaction_hash(data, nonce, to, value)
        .call()
        .await
        .unwrap()
}

pub async fn update_hash(
    contract: &MultiSig,
    data: Bits256,
    nonce: u64,
) -> FuelCallResponse<Bits256> {
    contract
        .methods()
        .update_hash(data, nonce)
        .call()
        .await
        .unwrap()
}

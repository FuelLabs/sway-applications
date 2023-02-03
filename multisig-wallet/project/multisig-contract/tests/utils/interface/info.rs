use fuels::{programs::call_response::FuelCallResponse, tx::ContractId, types::Bits256};

use crate::utils::setup::{MultiSig, TypeToHash};

pub async fn balance(contract: &MultiSig, asset_id: ContractId) -> FuelCallResponse<u64> {
    contract.methods().balance(asset_id).call().await.unwrap()
}

pub async fn nonce(contract: &MultiSig) -> FuelCallResponse<u64> {
    contract.methods().nonce().call().await.unwrap()
}

pub async fn threshold(contract: &MultiSig) -> FuelCallResponse<u64> {
    contract.methods().threshold().call().await.unwrap()
}

pub async fn calculate_hash(
    contract: &MultiSig,
    type_to_hash: TypeToHash,
) -> FuelCallResponse<Bits256> {
    contract
        .methods()
        .calculate_hash(type_to_hash)
        .call()
        .await
        .unwrap()
}

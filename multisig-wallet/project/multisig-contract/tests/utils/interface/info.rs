use fuels::{
    programs::call_response::FuelCallResponse,
    tx::ContractId,
    types::{Bits256, Identity},
};

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

pub async fn compute_hash(
    contract: &MultiSig,
    type_to_hash: TypeToHash,
) -> FuelCallResponse<Bits256> {
    contract
        .methods()
        .compute_hash(type_to_hash)
        .call()
        .await
        .unwrap()
}

pub async fn compute_transaction_hash(
    contract: &MultiSig,
    asset_id: Option<ContractId>,
    calldata: Option<Vec<u8>>,
    contract_identifier: ContractId,
    forwarded_gas: Option<u64>,
    function_selector: Option<Vec<u8>>,
    nonce: u64,
    single_value_type_arg: Option<bool>,
    target: Identity,
    value: Option<u64>,
) -> FuelCallResponse<Bits256> {
    contract
        .methods()
        .compute_transaction_hash(
            asset_id,
            calldata,
            contract_identifier,
            forwarded_gas,
            function_selector,
            nonce,
            single_value_type_arg,
            target,
            value,
        )
        .call()
        .await
        .unwrap()
}
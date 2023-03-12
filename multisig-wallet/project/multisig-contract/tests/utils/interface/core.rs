use fuels::{
    programs::call_response::FuelCallResponse,
    types::{ContractId, Identity},
};

use crate::utils::setup::{MultiSig, SignatureInfo, User};

pub async fn cancel_transaction(contract: &MultiSig) -> FuelCallResponse<()> {
    contract
        .methods()
        .cancel_transaction()
        .call()
        .await
        .unwrap()
}

pub async fn constructor(contract: &MultiSig, users: Vec<User>) -> FuelCallResponse<()> {
    contract.methods().constructor(users).call().await.unwrap()
}

pub async fn execute_transaction(
    contract: &MultiSig,
    asset_id: Option<ContractId>,
    calldata: Option<Vec<u8>>,
    forwarded_gas: Option<u64>,
    function_selector: Option<Vec<u8>>,
    signatures: Vec<SignatureInfo>,
    single_value_type_arg: Option<bool>,
    target: Identity,
    value: Option<u64>,
) -> Result<FuelCallResponse<()>, ()> {
    if function_selector.is_none() {
        Ok(contract
            .methods()
            .execute_transaction(
                asset_id,
                calldata,
                forwarded_gas,
                function_selector,
                signatures,
                single_value_type_arg,
                target.clone(),
                value,
            )
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap())
    } else {
        Ok(contract
            .methods()
            .execute_transaction(
                asset_id,
                calldata,
                forwarded_gas,
                function_selector,
                signatures,
                single_value_type_arg,
                target.clone(),
                value,
            )
            .append_variable_outputs(1)
            .set_contract_ids(&[match target {
                Identity::ContractId(contract_identifier) => contract_identifier.into(),
                _ => {
                    return Err(());
                }
            }])
            .call()
            .await
            .unwrap())
    }
}

pub async fn set_threshold(
    contract: &MultiSig,
    signatures_data: Vec<SignatureInfo>,
    threshold: u64,
) -> FuelCallResponse<()> {
    contract
        .methods()
        .set_threshold(signatures_data, threshold)
        .call()
        .await
        .unwrap()
}

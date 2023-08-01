use fuels::{
    accounts::wallet::WalletUnlocked, programs::call_response::FuelCallResponse, types::Identity,
};

use crate::utils::setup::{ContractCallParams, MultiSig, SignatureInfo, TransferParams, User};

pub(crate) async fn constructor(
    contract: &MultiSig<WalletUnlocked>,
    users: Vec<User>,
) -> FuelCallResponse<()> {
    contract.methods().constructor(users).call().await.unwrap()
}

pub(crate) async fn execute_transaction(
    contract: &MultiSig<WalletUnlocked>,
    contract_call_params: Option<ContractCallParams>,
    signatures: Vec<SignatureInfo>,
    target: Identity,
    transfer_params: TransferParams,
) -> FuelCallResponse<()> {
    contract
        .methods()
        .execute_transaction(contract_call_params, signatures, target, transfer_params)
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap()
}

pub(crate) async fn set_threshold(
    contract: &MultiSig<WalletUnlocked>,
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

pub(crate) async fn set_weight(
    contract: &MultiSig<WalletUnlocked>,
    signatures_data: Vec<SignatureInfo>,
    user: User,
) -> FuelCallResponse<()> {
    contract
        .methods()
        .set_weight(signatures_data, user)
        .call()
        .await
        .unwrap()
}

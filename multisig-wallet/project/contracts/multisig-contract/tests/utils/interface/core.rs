use crate::utils::setup::{MultiSig, SignatureInfo, TransactionParameters, User};
use fuels::{
    accounts::wallet::WalletUnlocked,
    programs::{call_response::FuelCallResponse, call_utils::TxDependencyExtension},
    types::Identity,
};

pub(crate) async fn constructor(
    contract: &MultiSig<WalletUnlocked>,
    users: Vec<User>,
) -> FuelCallResponse<()> {
    contract.methods().constructor(users).call().await.unwrap()
}

pub(crate) async fn execute_transaction(
    contract: &MultiSig<WalletUnlocked>,
    signatures: Vec<SignatureInfo>,
    target: Identity,
    transaction_parameters: TransactionParameters,
) -> FuelCallResponse<()> {
    let contract_method_call = contract
        .methods()
        .execute_transaction(signatures, target.clone(), transaction_parameters.clone())
        .append_variable_outputs(1);

    match transaction_parameters {
        TransactionParameters::Call(_) => contract_method_call
            .with_contract_ids(&[match target {
                Identity::ContractId(contract_identifier) => contract_identifier.into(),
                _ => {
                    panic!("Target must be of type Identity::ContractId");
                }
            }])
            .call()
            .await
            .unwrap(),
        TransactionParameters::Transfer(_) => contract_method_call.call().await.unwrap(),
    }
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

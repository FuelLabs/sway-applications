use crate::utils::setup::{ContractCallParams, MultiSig, SignatureInfo, TransferParams, User};
use fuels::{
    accounts::wallet::WalletUnlocked,
    prelude::{Bech32Address, Bech32ContractId},
    programs::call_response::FuelCallResponse,
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
    contract_call_params: Option<ContractCallParams>,
    signatures: Vec<SignatureInfo>,
    target: Identity,
    transfer_params: TransferParams,
) -> FuelCallResponse<()> {
    let contract_method_call = contract
        .methods()
        .execute_transaction(
            contract_call_params.clone(),
            signatures,
            target.clone(),
            transfer_params,
        )
        .append_variable_outputs(1);

    if contract_call_params.is_none() {
        contract_method_call.call().await.unwrap()
    } else {
        contract_method_call
            .set_contract_ids(&[match target {
                Identity::ContractId(contract_identifier) => contract_identifier.into(),
                Identity::Address(address) => {
                    let address = Bech32Address::from(address);
                    Bech32ContractId::new(&address.hrp, address.hash)
                }
            }])
            .call()
            .await
            .unwrap()
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

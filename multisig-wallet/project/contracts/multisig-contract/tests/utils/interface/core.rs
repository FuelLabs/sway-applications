use fuels::{
    accounts::wallet::WalletUnlocked,
    prelude::ContractId,
    programs::call_response::FuelCallResponse,
    types::{Bits256, Identity},
};

use crate::utils::setup::{MultiSig, SignatureInfo, User};

pub(crate) async fn constructor(
    contract: &MultiSig<WalletUnlocked>,
    users: Vec<User>,
) -> FuelCallResponse<()> {
    contract.methods().constructor(users).call().await.unwrap()
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

use fuels::{
    accounts::wallet::WalletUnlocked,
    prelude::ContractId,
    programs::call_response::FuelCallResponse,
    types::{Bits256, Identity},
};

use crate::utils::setup::{MultiSig, User};

pub(crate) async fn approval_weight(
    contract: &MultiSig<WalletUnlocked>,
    user: Bits256,
) -> FuelCallResponse<u64> {
    contract
        .methods()
        .approval_weight(user)
        .call()
        .await
        .unwrap()
}

pub(crate) async fn balance(
    contract: &MultiSig<WalletUnlocked>,
    asset_id: ContractId,
) -> FuelCallResponse<u64> {
    contract.methods().balance(asset_id).call().await.unwrap()
}

pub(crate) async fn nonce(contract: &MultiSig<WalletUnlocked>) -> FuelCallResponse<u64> {
    contract.methods().nonce().call().await.unwrap()
}

pub(crate) async fn threshold(contract: &MultiSig<WalletUnlocked>) -> FuelCallResponse<u64> {
    contract.methods().threshold().call().await.unwrap()
}

pub(crate) async fn transaction_hash(
    contract: &MultiSig<WalletUnlocked>,
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

pub(crate) async fn threshold_hash(
    contract: &MultiSig<WalletUnlocked>,
    data: Option<Bits256>,
    nonce: u64,
    threshold: u64,
) -> FuelCallResponse<Bits256> {
    contract
        .methods()
        .threshold_hash(data, nonce, threshold)
        .call()
        .await
        .unwrap()
}

pub(crate) async fn weight_hash(
    contract: &MultiSig<WalletUnlocked>,
    data: Option<Bits256>,
    nonce: u64,
    user: User,
) -> FuelCallResponse<Bits256> {
    contract
        .methods()
        .weight_hash(data, nonce, user)
        .call()
        .await
        .unwrap()
}

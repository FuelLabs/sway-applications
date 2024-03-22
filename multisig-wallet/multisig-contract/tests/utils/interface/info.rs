use fuels::{
    accounts::wallet::WalletUnlocked, prelude::AssetId, programs::call_response::FuelCallResponse,
    types::Bits256,
};

use crate::utils::setup::{MultiSig, TypeToHash};

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
    asset_id: AssetId,
) -> FuelCallResponse<u64> {
    contract.methods().balance(asset_id).call().await.unwrap()
}

pub(crate) async fn nonce(contract: &MultiSig<WalletUnlocked>) -> FuelCallResponse<u64> {
    contract.methods().nonce().call().await.unwrap()
}

pub(crate) async fn threshold(contract: &MultiSig<WalletUnlocked>) -> FuelCallResponse<u64> {
    contract.methods().threshold().call().await.unwrap()
}

pub(crate) async fn compute_hash(
    contract: &MultiSig<WalletUnlocked>,
    type_to_hash: TypeToHash,
) -> FuelCallResponse<Bits256> {
    contract
        .methods()
        .compute_hash(type_to_hash)
        .call()
        .await
        .unwrap()
}

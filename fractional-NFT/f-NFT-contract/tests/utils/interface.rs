use crate::utils::setup::F_NFT;
use fuels::{
    prelude::{AssetId, CallParameters, TxPolicies, WalletUnlocked},
    programs::{call_response::FuelCallResponse, call_utils::TxDependencyExtension},
    types::{Bits256, Bytes32, Identity},
};

pub(crate) async fn deposit(
    contract: &F_NFT<WalletUnlocked>,
    nft: AssetId,
    reciever: Identity,
    vault_sub_id: Bytes32,
) -> FuelCallResponse<u64> {
    let call_params = CallParameters::new(1, nft, 1_000_000);

    contract
        .methods()
        .deposit(reciever, Bits256(*vault_sub_id))
        .with_tx_policies(TxPolicies::default().with_script_gas_limit(2_000_000))
        .call_params(call_params)
        .unwrap()
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap()
}

pub(crate) async fn withdraw(
    contract: &F_NFT<WalletUnlocked>,
    share_asset: AssetId,
    amount: u64,
    reciever: Identity,
    underlying_asset: AssetId,
    vault_sub_id: Bytes32,
) -> FuelCallResponse<u64> {
    let call_params = CallParameters::new(amount, share_asset, 1_000_000);

    contract
        .methods()
        .withdraw(reciever, underlying_asset, Bits256(*vault_sub_id))
        .with_tx_policies(TxPolicies::default().with_script_gas_limit(2_000_000))
        .call_params(call_params)
        .unwrap()
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap()
}

pub(crate) async fn managed_assets(
    contract: &F_NFT<WalletUnlocked>,
    underlying_asset: AssetId,
    vault_sub_id: Bytes32,
) -> u64 {
    contract
        .methods()
        .managed_assets(underlying_asset, Bits256(*vault_sub_id))
        .call()
        .await
        .unwrap()
        .value
}

pub(crate) async fn max_depositable(
    contract: &F_NFT<WalletUnlocked>,
    reciever: Identity,
    underlying_asset: AssetId,
    vault_sub_id: Bytes32,
) -> Option<u64> {
    contract
        .methods()
        .max_depositable(reciever, underlying_asset, Bits256(*vault_sub_id))
        .call()
        .await
        .unwrap()
        .value
}

pub(crate) async fn max_withdrawable(
    contract: &F_NFT<WalletUnlocked>,
    underlying_asset: AssetId,
    vault_sub_id: Bytes32,
) -> Option<u64> {
    contract
        .methods()
        .max_withdrawable(underlying_asset, Bits256(*vault_sub_id))
        .call()
        .await
        .unwrap()
        .value
}

pub(crate) async fn total_assets(contract: &F_NFT<WalletUnlocked>) -> u64 {
    contract
        .methods()
        .total_assets()
        .call()
        .await
        .unwrap()
        .value
}

pub(crate) async fn total_supply(
    contract: &F_NFT<WalletUnlocked>,
    share_asset: AssetId,
) -> Option<u64> {
    contract
        .methods()
        .total_supply(share_asset)
        .call()
        .await
        .unwrap()
        .value
}

use crate::utils::setup::AirdropDistributor;
use fuels::{
    prelude::{AssetId, CallParameters, TxPolicies, WalletUnlocked},
    programs::{call_response::FuelCallResponse, call_utils::TxDependencyExtension},
    types::{Bits256, Identity},
};

pub(crate) async fn claim(
    amount: u64,
    contract: &AirdropDistributor<WalletUnlocked>,
    key: u64,
    proof: Vec<Bits256>,
    to: Identity,
) -> FuelCallResponse<()> {
    contract
        .methods()
        .claim(amount, key, proof, to)
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap()
}

pub(crate) async fn clawback(
    contract: &AirdropDistributor<WalletUnlocked>,
) -> FuelCallResponse<()> {
    contract
        .methods()
        .clawback()
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap()
}

pub(crate) async fn airdrop_constructor(
    admin: Identity,
    amount: u64,
    asset: AssetId,
    claim_time: u32,
    contract: &AirdropDistributor<WalletUnlocked>,
    merkle_root: Bits256,
    num_leaves: u64,
) -> FuelCallResponse<()> {
    let call_params = CallParameters::new(amount, asset, 1_000_000);

    contract
        .methods()
        .constructor(admin, claim_time, merkle_root, num_leaves)
        .with_tx_policies(TxPolicies::default().with_script_gas_limit(2_000_000))
        .call_params(call_params)
        .unwrap()
        .call()
        .await
        .unwrap()
}

use crate::utils::setup::{AirdropDistributor, SimpleAsset};
use fuels::{
    prelude::{AssetId, CallParameters, TxParameters, WalletUnlocked},
    programs::call_response::FuelCallResponse,
    types::{Bits256, ContractId, Identity},
};

pub(crate) async fn asset_constructor(
    asset_supply: u64,
    contract: &SimpleAsset<WalletUnlocked>,
    minter: Identity,
) -> FuelCallResponse<()> {
    contract
        .methods()
        .constructor(asset_supply, minter)
        .call()
        .await
        .unwrap()
}

pub(crate) async fn mint_to(
    amount: u64,
    contract: &SimpleAsset<WalletUnlocked>,
    to: Identity,
) -> FuelCallResponse<()> {
    contract
        .methods()
        .mint_to(amount, to)
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap()
}

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
    asset: ContractId,
    claim_time: u64,
    contract: &AirdropDistributor<WalletUnlocked>,
    merkle_root: Bits256,
    num_leaves: u64,
) -> FuelCallResponse<()> {
    let tx_parameters = TxParameters::default().set_gas_limit(2_000_000);
    let call_params = CallParameters::new(amount, AssetId::from(*asset), 1_000_000);

    contract
        .methods()
        .constructor(admin, claim_time, merkle_root, num_leaves)
        .tx_params(tx_parameters)
        .call_params(call_params)
        .unwrap()
        .call()
        .await
        .unwrap()
}

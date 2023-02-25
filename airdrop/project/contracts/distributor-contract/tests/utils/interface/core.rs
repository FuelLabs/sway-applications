use crate::utils::setup::{AirdropDistributor, SimpleAsset};
use fuels::{
    programs::call_response::FuelCallResponse,
    types::{Bits256, ContractId, Identity},
};

pub(crate) async fn asset_constructor(
    asset_supply: u64,
    contract: &SimpleAsset,
    minter: Identity,
) -> FuelCallResponse<()> {
    contract
        .methods()
        .constructor(asset_supply, minter)
        .call()
        .await
        .unwrap()
}

pub(crate) async fn claim(
    amount: u64,
    asset_id: ContractId,
    contract: &AirdropDistributor,
    key: u64,
    num_leaves: u64,
    proof: Vec<Bits256>,
    to: Identity,
) -> FuelCallResponse<()> {
    contract
        .methods()
        .claim(amount, key, num_leaves, proof, to)
        .append_variable_outputs(1)
        .set_contract_ids(&[asset_id.into()])
        .call()
        .await
        .unwrap()
}

pub(crate) async fn airdrop_constructor(
    asset: ContractId,
    claim_time: u64,
    contract: &AirdropDistributor,
    merkle_root: Bits256,
) -> FuelCallResponse<()> {
    contract
        .methods()
        .constructor(asset, claim_time, merkle_root)
        .call()
        .await
        .unwrap()
}

use fuels::{contract::call_response::FuelCallResponse, prelude::*, tx::ContractId};

use crate::utils::setup::Vault;

pub async fn asset(contract: &Vault) -> FuelCallResponse<ContractId> {
    contract.methods().asset().call().await.unwrap()
}

pub async fn convert_to_assets(contract: &Vault, shares: u64) -> FuelCallResponse<u64> {
    contract
        .methods()
        .convert_to_assets(shares)
        .call()
        .await
        .unwrap()
}

pub async fn convert_to_shares(contract: &Vault, assets: u64) -> FuelCallResponse<u64> {
    contract
        .methods()
        .convert_to_shares(assets)
        .call()
        .await
        .unwrap()
}

pub async fn max_deposit(contract: &Vault, receiver: Identity) -> FuelCallResponse<u64> {
    contract
        .methods()
        .max_deposit(receiver)
        .call()
        .await
        .unwrap()
}

pub async fn max_mint(contract: &Vault, receiver: Identity) -> FuelCallResponse<u64> {
    contract.methods().max_mint(receiver).call().await.unwrap()
}

pub async fn max_redeem(contract: &Vault, owner: Identity) -> FuelCallResponse<u64> {
    contract.methods().max_redeem(owner).call().await.unwrap()
}

pub async fn max_withdraw(contract: &Vault, owner: Identity) -> FuelCallResponse<u64> {
    contract.methods().max_withdraw(owner).call().await.unwrap()
}

pub async fn preview_deposit(contract: &Vault, assets: u64) -> FuelCallResponse<u64> {
    contract
        .methods()
        .preview_deposit(assets)
        .call()
        .await
        .unwrap()
}

pub async fn preview_mint(contract: &Vault, shares: u64) -> FuelCallResponse<u64> {
    contract
        .methods()
        .preview_mint(shares)
        .call()
        .await
        .unwrap()
}

pub async fn preview_redeem(contract: &Vault, shares: u64) -> FuelCallResponse<u64> {
    contract
        .methods()
        .preview_redeem(shares)
        .call()
        .await
        .unwrap()
}

pub async fn preview_withdraw(contract: &Vault, assets: u64) -> FuelCallResponse<u64> {
    contract
        .methods()
        .preview_withdraw(assets)
        .call()
        .await
        .unwrap()
}

pub async fn total_assets(contract: &Vault) -> FuelCallResponse<u64> {
    contract.methods().total_assets().call().await.unwrap()
}

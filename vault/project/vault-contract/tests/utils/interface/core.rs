use fuels::{contract::call_response::FuelCallResponse, prelude::*};

use crate::utils::setup::Vault;

pub async fn deposit(contract: &Vault, assets: u64, receiver: Identity) -> FuelCallResponse<u64> {
    contract
        .methods()
        .deposit(assets, receiver)
        .call()
        .await
        .unwrap()
}

pub async fn mint(contract: &Vault, shares: u64, receiver: Identity) -> FuelCallResponse<u64> {
    contract
        .methods()
        .mint(shares, receiver)
        .call()
        .await
        .unwrap()
}

pub async fn redeem(
    contract: &Vault,
    shares: u64,
    receiver: Identity,
    owner: Identity,
) -> FuelCallResponse<u64> {
    contract
        .methods()
        .redeem(shares, receiver, owner)
        .call()
        .await
        .unwrap()
}

pub async fn withdraw(
    contract: &Vault,
    assets: u64,
    receiver: Identity,
    owner: Identity,
) -> FuelCallResponse<u64> {
    contract
        .methods()
        .withdraw(assets, receiver, owner)
        .call()
        .await
        .unwrap()
}

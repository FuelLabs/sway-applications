use fuels::{
    accounts::wallet::WalletUnlocked, programs::call_response::FuelCallResponse, types::Address,
};

use crate::utils::setup::TargetContract;

pub(crate) async fn count(
    contract: &TargetContract<WalletUnlocked>,
    address: Address,
) -> FuelCallResponse<u64> {
    contract.methods().count(address).call().await.unwrap()
}

pub(crate) async fn deposit(
    contract: &TargetContract<WalletUnlocked>,
    address: Address,
) -> FuelCallResponse<u64> {
    contract.methods().deposit(address).call().await.unwrap()
}

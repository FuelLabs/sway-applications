use fuels::{
    accounts::wallet::WalletUnlocked, programs::call_response::FuelCallResponse, types::Address,
};

use crate::utils::setup::CallableContract;

pub async fn check_counter_map(
    contract: &CallableContract<WalletUnlocked>,
    address: Address,
) -> FuelCallResponse<u64> {
    contract
        .methods()
        .check_counter_map(address)
        .call()
        .await
        .unwrap()
}

pub async fn check_deposit_map(
    contract: &CallableContract<WalletUnlocked>,
    address: Address,
) -> FuelCallResponse<u64> {
    contract
        .methods()
        .check_deposit_map(address)
        .call()
        .await
        .unwrap()
}

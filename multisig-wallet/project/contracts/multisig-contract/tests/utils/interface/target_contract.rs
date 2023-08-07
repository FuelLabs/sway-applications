use fuels::{
    accounts::wallet::WalletUnlocked, programs::call_response::FuelCallResponse, types::Address,
};

use crate::utils::setup::TargetContract;

pub(crate) async fn check_counter(
    contract: &TargetContract<WalletUnlocked>,
    address: Address,
) -> FuelCallResponse<u64> {
    contract
        .methods()
        .check_counter(address)
        .call()
        .await
        .unwrap()
}

pub(crate) async fn check_deposit(
    contract: &TargetContract<WalletUnlocked>,
    address: Address,
) -> FuelCallResponse<u64> {
    contract
        .methods()
        .check_deposit(address)
        .call()
        .await
        .unwrap()
}

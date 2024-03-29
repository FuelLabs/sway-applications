use crate::utils::setup::TicTacToe;
use fuels::{
    accounts::wallet::WalletUnlocked, prelude::TxPolicies,
    programs::call_response::FuelCallResponse, types::Identity,
};

pub(crate) async fn new_game(
    contract: &TicTacToe<WalletUnlocked>,
    player_one: &Identity,
    player_two: &Identity,
) -> FuelCallResponse<()> {
    contract
        .methods()
        .new_game(player_one.clone(), player_two.clone())
        .with_tx_policies(TxPolicies::default().with_script_gas_limit(2_000_000))
        .call()
        .await
        .unwrap()
}

pub(crate) async fn make_move(
    contract: &TicTacToe<WalletUnlocked>,
    position: u64,
) -> FuelCallResponse<()> {
    contract
        .methods()
        .make_move(position)
        .with_tx_policies(TxPolicies::default().with_script_gas_limit(2_000_000))
        .call()
        .await
        .unwrap()
}

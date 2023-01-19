use crate::utils::setup::TicTacToe;
use fuels::{contract::call_response::FuelCallResponse, prelude::*};

pub async fn new_game(
    contract: &TicTacToe,
    player_one: &Identity,
    player_two: &Identity,
) -> FuelCallResponse<()> {
    contract
        .methods()
        .new_game(player_one.clone(), player_two.clone())
        .call()
        .await
        .unwrap()
}

pub async fn make_move(contract: &TicTacToe, position: u64) -> FuelCallResponse<()> {
    contract.methods().make_move(position).call().await.unwrap()
}

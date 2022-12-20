library utils;

dep data_structures;

use data_structures::GameState;

// This file is unused because I didn't figure out how to refactor main.sw without the compiler complaining about various things.
// Will delete after getting feedback.

#[storage(read)]
pub fn computeGameState(board: StorageMap<u64, u64>) -> GameState {
    let winning_combinations = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];

    let mut combo_index = 0;
    while combo_index < 8 {
        let combination = winning_combinations[combo_index];
        let mut player_one_won = true;
        let mut player_two_won = true;

        let mut position_index = 0;
        while position_index < 3 {
            let position = combination[position_index];
            let player = board.get(position);
            if player != 1 {
                player_one_won = false;
            }
            if player != 2 {
                player_two_won = false;
            }

            position_index += 1;
        }

        if player_one_won {
            return GameState::PlayerOneWon;
        }
        if player_two_won {
            return GameState::PlayerTwoWon;
        }

        combo_index += 1;
    }

    // Check if game is a draw
    let mut draw = true;
    let position_index = 0;
    while position_index < 9 {
        if board.get(position_index) == 0 {
            draw = false;
        }
    }
    if draw {
        return GameState::Draw;
    }

    return GameState::InProgress;
}

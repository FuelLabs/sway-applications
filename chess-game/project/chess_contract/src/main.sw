contract;

use chess_abi::Chess;
use lib_chess::{board::Board, game::{Game, Status}, move::Move,};
use std::{
    call_frames::contract_id,
    constants::ZERO_B256,
    hash::keccak256
};

storage {
    // should bond be a config time const ?
    bond: u64 = 42,
    // mapping of game_id => Game. game_ids are globally unique
    games: StorageMap<b256, Game> = StorageMap {},
    // mapping of Player1 => Player2 => match_number
    matches: StorageMap<(Identity, Identity), u64> = StorageMap {},
}

impl Chess for Contract {
    #[storage(read, write)]
    fn start_new_game(player1: Identity, player2: Identity, bond: Option<u64>) -> b256 {
        // increment the previous game number
        let match_number = storage.matches.get((player1, player2)) + 1;
        storage.matches.insert((player1, player2), match_number);

        let mut game = Game {
            player_1: player1,
            player_2: player2,
            match_number,
            board: Board::new(),
            status: Status::Active,
            statehash: ZERO_B256,
        };
        game.statehash = hash_state(game.board.piecemap, game.board.metadata);
        let game_id = generate_game_id(player1, player2, match_number);
        storage.games.insert(game_id, game);

        game_id

    }

    // #[storage(write)]
    // fn post_bond(game_id: b256);

    // #[storage(read)]
    // fn move(move: Move);

    // #[storage(read, write)]
    // fn move_from_state(nonce: u64, sig: B512);

    // #[storage(read)]
    // fn game(game_id: b256) -> Game;

    // fn game_id(player1: Identity, player2: Identity, nonce: u64) -> b256;

    // #[storage(read, write)]
    // fn claim_winnings(game_id: b256);
}

// Private
fn generate_game_id(player1: Identity, player2: Identity, game_number: u64) -> b256 {
    keccak256((player1, player2, game_number, contract_id()))
}

fn hash_state(piecemap: b256, metadata: u64,) -> b256 {
    keccak256((piecemap, metadata))
}

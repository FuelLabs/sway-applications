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
    salts: StorageMap<(Identity, Identity), u64> = StorageMap {},
}

impl Chess for Contract {
    #[storage(read, write)]
    fn start_new_game(player1: Identity, player2: Identity, bond: Option<u64>) -> b256 {
        // increment the previous game salt
        let salt = storage.salts.get((player1, player2)) + 1;
        storage.salts.insert((player1, player2), salt);

        let status = match bond {
            // free play, no bond required.
            Option::None => Status::Active,
            Option::Some(v) => Status::Standby,
        };
        // TODO: add winner, maybe to status?
        // TODO: track bonds payed by each player
        let mut game = Game {
            players: [player2, player1],
            bonds_payed: [false, false],
            salt,
            board: Board::new(),
            status: status,
            winner: Option::None,
            statehash: ZERO_B256,
        };
        game.statehash = game.hash_state();
        let game_id = game.id();
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
// fn generate_game_id(player1: Identity, player2: Identity, game_number: u64) -> b256 {
//     keccak256((player1, player2, game_number, contract_id()))
// }

// // TODO: decide if this should include game.status by testing adversarially
// fn hash_state(piecemap: b256, metadata: u64,) -> b256 {
//     keccak256((piecemap, metadata))
// }

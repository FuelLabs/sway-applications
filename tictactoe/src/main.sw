contract;

use core::*;
use std::*;
use std::{
    address::Address,
    assert::assert,
    chain::auth::{AuthError, msg_sender},
    identity::Identity,
    result::*,
    revert::revert,
};

pub struct Address {
    value: b256,
}

storage {
    games_played: u64,
}

enum Players {
    None: (),
    PlayerOne: Address,
    PlayerTwo: Address,
}

enum Winners {
    Players: Players,
    None: (),
    Draw: (),
}

struct Game {
    gameId: u64,
    PlayerOne: Players,
    PlayerTwo: Players,
    winner: Winners,
    playerTurn: Players,
}

abi TicTacToe {
    fn new_game(player_one: Players, player_two: Players) -> Game;
    fn make_move(game: Game, position: u64) -> (bool, str[20]);
    fn save_player_position(player: Players, position: u64);
    fn get_player_position_filled(gameID: u64, position: u64) -> Players;
    fn save_winner(gameID: u64, winner: Winners);
    fn get_winner(gameID: u64) -> Winners;
    fn calculate_winner(game: Game) -> Winners;
    //fn next_player(game: Game);
    // fn horizontal_alignment(gameID: u64) -> Winners;
    // fn vertical_alignment(gameID: u64) -> Winners;
    // fn diagonal_alignment(gameID: u64) -> Winners;
    // fn is_board_full(gameID: u64) -> bool;
}

impl TicTacToe for Contract {
    fn new_game(player_one:Players, player_two:Players) -> Game {
        let gameID = storage.games_played;
        
        let mut game = Game {
            gameID: gameID,
            PlayerOne: player_one,
            PlayerTwo: player_two,
            winner: Winners::None,
            // let error1 = Error::StateError(StateError::Void);
            playerTurn: player_one,
        };
        storage.games_played = storage.games_played + 1;
        return game;
    }

    fn make_move(game: Game, position: u64) -> (bool, str[20]) {
        let gameID = game.gameID;
        if (gameID > storage.games_played) {
            return(false, "No such game exists.");
        }
        // Any winner other than `None` means that no more moves are allowed.
        if (game.winner != Winners::None) {
            return(false, "The game has ended. ");
        }
        // Only the player whose turn it is may make a move.
        if (msg_sender != getCurrentPlayer(gameID)) {
            // TODO: what if the player is not present in the game at all?
            return(false, "It is not your turn.");
        }
        // Players can only make moves in cells on the board that have not been played before.
        if (get_player_position_filled(gameID, position) != Players::None) {
            return(false, "This pos is taken.  ");
        }
        // Now the move is recorded and the according event emitted.
        save_player_position(game.playerTurn, position);

        // Check if there is a winner now that we have a new move.
        let winner = calculateWinner(gameID);
        if (winner != Winners::None) {
            // If there is a winner (can be a `Draw`) it must be recorded in the game and
            // the corresponding event must be emitted.
            game.winner = winner;
            return(true, "The game is over.");
        }
        // A move was made and there is no winner yet.
        // The next player should make her move.
        //nextPlayer(game);
        return(true, "");
    }

    // player 1 or 2, position is 1-9 (123/456/789)
    fn save_player_position(player: Players, position: u64) {
        store(sha256(("player_pos", position)), player);
    }
    // get the players position on the board, returns the player or empty
    // if the player returned value is None, that means it's empty
    fn get_player_position_filled(gameID: u64, position: u64) -> Players {
        get(sha256(("player_pos", gameID, position)));
    }
    fn save_winner(gameID: u64, winner: Winners) {
        store(sha256(("winner", gameID)), winner);
    }
    fn get_winner(gameID: u64) -> Winners {
        get(sha256(("winner", gameID, winner)));
    }
    fn calculate_winner(game: Game) -> Winners {
        let gameID = game.gameID;
        let player = horizontal_alignment(gameID);

        if (player == Players::PlayerOne) {
            save_winner(gameID, winner::Players(Players::PlayerOne));
            return winner::Players(Players::PlayerOne);
        }
        if (player == Players::PlayerTwo) {
            save_winner(gameID, winner::Players(Players::PlayerTwo));
            return winner::Players(Players::PlayerTwo);
        }

        player = vertical_alignment(gameID);
        if (player == Players::PlayerOne) {
            save_winner(gameID, winner::Players(Players::PlayerOne));
            return winner::Players(Players::PlayerOne);
        }
        if (player == Players::PlayerTwo) {
            save_winner(gameID, winner::Players(Players::PlayerTwo));
            return winner::Players(Players::PlayerTwo);
        }
        player = diagonal_alignment(gameID);
        if (player == Players::PlayerOne) {
            save_winner(gameID, winner::Players(Players::PlayerOne));
            return winner::Players(Players::PlayerOne);
        }
        if (player == Players::PlayerTwo) {
            game.winner = Winner::Player:Players(Players::PlayerTwo);
            save_winner(gameID, game::winner);
            
            return game.winner;
        }
        // If there is no winner and no more space on the board,
        // then it is a draw.
        if (is_board_full(gameID)) {
            
            game.winner = Winner::Draw;
            save_winner(gameID, winner);
            return winner;
        }
        else {
            save_winner(gameID, winner);
            return winner;
        }
    }
}
fn test(gameID: u64) -> Winners {
    let mut counter = 1;
    let mut break_early = false;
    let mut i = 1;
    while counter < 3 {
        while get_player_position_filled(gameID, i) != Players::None {
            if get_player_position_filled(gameID, i) == get_player_position_filled(gameID, i + 1) {
                counter = counter + 1;
                i = i + 1;
            } else {
                counter = 1;
                if i < 4 && i < 7 {
                    i = 4;
                } else {
                    i = 7;
                }
            }
        }
    }
}

// nextPlayer changes whose turn it is for the given `_game`.
// fn next_player(game: Game) {
//     if (game.playerTurn == game:Players::PlayerOne) {
//         match game {
//             playerTurn = Players::PlayerTwo;
//         }
//     } else {
//         match game {
//             playerTurn = Players::PlayerOne;
//         }

//     }
// }

fn horizontal_alignment(gameID: u64) -> Players {
    if (get_player_position_filled(gameID, 1) == get_player_position_filled(gameID, 2)) {
        if (get_player_position_filled(gameID, 2) == get_player_position_filled(gameID, 3)) {
            if (get_player_position_filled(gameID, 1) != Players::None) {
                return get_player_position_filled(gameID, 1);
            }
        }
    }

    else if (get_player_position_filled(gameID, 4) == get_player_position_filled(gameID, 5)) {
        if (get_player_position_filled(gameID, 5) == get_player_position_filled(6)) {
            if (get_player_position_filled(4) != Players::None) {
                return get_player_position_filled(gameID, 4);
            }
        }
    }

    else if (get_player_position_filled(gameID, 7) == get_player_position_filled(gameID, 8)) {
        if (get_player_position_filled(gameID, 8) == get_player_position_filled(gameID, 9)) {
            if (get_player_position_filled(gameID, 7) != Players::None) {
                return get_player_position_filled(gameID, 7);
            }
        }
    }
    return Players::None;
}

fn vertical_alignment(gameID: u64) -> Players {
    if (get_player_position_filled(gameID, 1) == get_player_position_filled(gameID, 4)) {
        if (get_player_position_filled(gameID, 4) == get_player_position_filled(gameID, 7)) {
            if (get_player_position_filled(gameID, 1) != Players::None) {
                return get_player_position_filled(gameID, 1);
            }
        }
    }

    else if (get_player_position_filled(gameID, 2) == get_player_position_filled(gameID, 5)) {
        if (get_player_position_filled(gameID, 5) == get_player_position_filled(gameID, 8)) {
            if (get_player_position_filled(gameID, 2) != Players::None) {
                return get_player_position_filled(gameID, 2);
            }
        }
    }

    else if (get_player_position_filled(gameID, 3) == get_player_position_filled(gameID, 6)) {
        if (get_player_position_filled(gameID, 6) == get_player_position_filled(gameID, 9)) {
            if (get_player_position_filled(gameID, 3) != Players::None) {
                return get_player_position_filled(gameID, 3);
            }
        }
    }
}

fn diagonal_alignment(gameID: u64) -> Players {
    if (get_player_position_filled(gameID, 1) == get_player_position_filled(gameID, 5)) {
        if (get_player_position_filled(gameID, 5) == get_player_position_filled(gameID, 9)) {
            if (get_player_position_filled(gameID, 1) != Players::None) {
                return get_player_position_filled(gameID, 1);
            }
        }
    } else if (get_player_position_filled(gameID, 3) == get_player_position_filled(gameID, 5)) {
        if (get_player_position_filled(gameID, 5) == get_player_position_filled(gameID, 7)) {
            if (get_player_position_filled(gameID, 3) != Players::None) {
                return get_player_position_filled(gameID, 2);
            }
        }
    }
    return Players::None;
}

fn is_board_full(gameID: u64) -> bool {
    let mut x = 0;
    while x < 9 {
        if (get_player_position_filled(gameID, x) == Players::None) {
            return false;
        }
        x = x + 1;
    }
    return true;
}



enum Enum_a {
    abc: Enum_b,
    var1: (),
    var2, (),
}

enum Enum_b {
    abc: (),
    def: (),
}

struct S {
    abc:
}
library events;

dep data_structures;

use data_structures::GameState;

pub struct NewGameEvent {
    player_one: Identity,
    player_two: Identity,
}

pub struct MoveEvent {
    player: u64,
    player_id: Identity,
    position: u64,
}

pub struct GameOverEvent {
    game_state: GameState,
}
library game;

dep players;

use players::Players;

/// A game is defined by the Players and the winner.
pub struct Game {
    players: Players,
    winner: Option<Identity>,
    player_turn: u64,
    move_counter: u64,
}

impl Game {
    pub fn default() -> Self {
        Self {
            players: Players::default(),
            winner: Option::None,
            player_turn: 1,
            move_counter: 0,
        }
    }

    pub fn new(player_one: Identity, player_two: Identity) -> Self {
        Self {
            players: Players::new(player_one, player_two),
            winner: Option::None,
            player_turn: 1,
            move_counter: 0,
        }
    }

    pub fn increment_move(mut self) {
        self.move_counter += 1;
    }

    pub fn switch_turn(mut self) {
        self.player_turn = if self.player_turn == 2 {1} else {2};
    }
}

library data_structures;

pub struct Game {
    player_one: Player,
    player_two: Player,
}

impl Game {
    pub fn new(player_one: Player, player_two: Player) -> Self {
        Self {
            player_one,
            player_two,
        }
    }
}

pub struct Player {
    score: u64,
}

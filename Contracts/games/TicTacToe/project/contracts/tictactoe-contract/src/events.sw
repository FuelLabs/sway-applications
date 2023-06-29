library;

pub struct GameDrawnEvent {
    player_one: Identity,
    player_two: Identity,
}

pub struct GameWonEvent {
    player: Identity,
}

pub struct NewGameEvent {
    player_one: Identity,
    player_two: Identity,
}

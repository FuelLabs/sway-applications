library;

// A list of all the winning combinations.
const MATCHES = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];

/// Checks if a player has won.
///
/// # Arguments
///
/// * `board`: [Vec<Option<Identity>>] - A vector of all the markers on the board.
/// * `player`: [Identity] - The player to check for.
///
/// # Returns
///
/// * [bool] - True if the player has won, false otherwise.
pub fn win_check(board: Vec<Option<Identity>>, player: Identity) -> bool {
    let mut i = 0;
    while (i < 8) {
        let marker_one = board.get(MATCHES[i][0]).unwrap();
        let marker_two = board.get(MATCHES[i][1]).unwrap();
        let marker_three = board.get(MATCHES[i][2]).unwrap();

        if (marker_one.is_some()
            && marker_one.unwrap() == player)
            && (marker_two.is_some()
            && marker_two.unwrap() == player)
            && (marker_three.is_some()
            && marker_three.unwrap() == player)
        {
            return true;
        }
        i += 1;
    }
    return false;
}

/// Checks if the game ends up in a draw.
///
/// # Arguments
///
/// * `board`: [Vec<Option<Identity>>] - A vector of all the markers on the board.
/// * `player_one`: [Identity] - The first player.
/// * `player_two`: [Identity] - The second player.
/// * `move_counter`: [u64] - The number of moves made.
///
/// # Returns
///
/// * [bool] - True if the game has ended in a draw, false otherwise.
pub fn draw(
    board: Vec<Option<Identity>>,
    player_one: Identity,
    player_two: Identity,
    move_counter: u64,
) -> bool {
    if move_counter != 9 {
        return false;
    }

    let mut i = 0;
    while (i < 8) {
        let marker_one = board.get(MATCHES[i][0]).unwrap();
        let marker_two = board.get(MATCHES[i][1]).unwrap();
        let marker_three = board.get(MATCHES[i][2]).unwrap();

        if ((marker_one.is_some()
            && marker_one.unwrap() == player_one)
            && (marker_two.is_some()
            && marker_two.unwrap() == player_one)
            && (marker_three.is_some()
            && marker_three.unwrap() == player_one)
            || (marker_one.is_some()
            && marker_one.unwrap() == player_two)
            && (marker_two.is_some()
            && marker_two.unwrap() == player_two)
            && (marker_three.is_some()
            && marker_three.unwrap() == player_two))
        {
            return false;
        }
        i += 1;
    }
    return true;
}

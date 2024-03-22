library;

// A list of all the winning combinations.
const MATCHES = [
    [0, 1, 2], [3, 4, 5], [6, 7, 8], [0, 3, 6], [1, 4, 7], [2, 5, 8], [0, 4, 8], [2, 4, 6],
];

/// Checks if a player has won.
///
/// # Arguments
///
/// * `board`: [Vec<Option<Identity>>] - A vector of all the markers on the board.
/// * `is_player1`: [bool] - The boolean corresponding to if the player is player 1 or 2.
///
/// # Returns
///
/// * [bool] - True if the player has won, false otherwise.
pub fn win_check(board: Vec<Option<bool>>, is_player1: bool) -> bool {
    let mut i = 0;
    while (i < 8) {
        let marker_one = board.get(MATCHES[i][0]).unwrap();
        let marker_two = board.get(MATCHES[i][1]).unwrap();
        let marker_three = board.get(MATCHES[i][2]).unwrap();

        if (marker_one.is_some()
            && marker_one.unwrap() == is_player1)
            && (marker_two.is_some()
            && marker_two.unwrap() == is_player1)
            && (marker_three.is_some()
            && marker_three.unwrap() == is_player1)
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
/// * `move_counter`: [u64] - The number of moves made.
///
/// # Returns
///
/// * [bool] - True if the game has ended in a draw, false otherwise.
pub fn draw(board: Vec<Option<bool>>, move_counter: u64) -> bool {
    if move_counter != 9 {
        return false;
    }

    let mut i = 0;
    while (i < 8) {
        let marker_one = board.get(MATCHES[i][0]).unwrap();
        let marker_two = board.get(MATCHES[i][1]).unwrap();
        let marker_three = board.get(MATCHES[i][2]).unwrap();

        if ((marker_one
                    .is_some()
                && marker_one
                    .unwrap())
            && (marker_two
                    .is_some()
                && marker_two
                    .unwrap())
            && (marker_three
                    .is_some()
                && marker_three
                    .unwrap())
                || (marker_one
                        .is_some()
                    && !marker_one
                        .unwrap())
                && (marker_two
                        .is_some()
                    && !marker_two
                        .unwrap())
                && (marker_three
                        .is_some()
                    && !marker_three
                        .unwrap()))
        {
            return false;
        }
        i += 1;
    }
    return true;
}

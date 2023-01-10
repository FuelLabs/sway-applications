library utils;

use core::ops::Eq;

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

impl<T> Eq for Option<T> {
    fn eq(self, other: Self) -> bool {
        match (self, other) {
            (Option::None, Option::None) => true,
            (Option::Some(T), Option::Some(T)) => true,
            _ => false,
        }
    }
}

pub fn win_check(board: Vec<Option<Identity>>, player: Option<Identity>) -> bool {
    let mut i = 0;
    while (i < 9) {
        if (board.get(MATCHES[i][0]).unwrap() == player)
            && (board.get(MATCHES[i][1]).unwrap() == player)
            && (board.get(MATCHES[i][2]).unwrap() == player)
        {
            return true;
        }
        i += 1;
    }
    return false;
}

pub fn draw(
    board: Vec<Option<Identity>>,
    player_one: Option<Identity>,
    player_two: Option<Identity>,
) -> bool {
    let mut i = 0;
    while (i < 9) {
        if ((board.get(MATCHES[i][0]).unwrap() == player_one)
            && (board.get(MATCHES[i][1]).unwrap() == player_one)
            && (board.get(MATCHES[i][2]).unwrap() == player_one)
            || (board.get(MATCHES[i][0]).unwrap() == player_two)
            && (board.get(MATCHES[i][1]).unwrap() == player_two)
            && (board.get(MATCHES[i][2]).unwrap() == player_two))
        {
            return false;
        }
        i += 1;
    }
    return true;
}

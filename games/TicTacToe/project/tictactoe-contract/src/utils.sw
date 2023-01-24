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

pub fn win_check(board: Vec<Option<Identity>>, player: Identity) -> bool {
    let mut i = 0;
    while (i < 9) {
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

pub fn draw(
    board: Vec<Option<Identity>>,
    player_one: Identity,
    player_two: Identity,
) -> bool {
    let mut i = 0;
    while (i < 9) {
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

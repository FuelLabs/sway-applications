// ANCHOR: interface
library interface;

dep data_structures;

use data_structures::{Game, Player};

abi ConnectFour {
    /// Creates a new game
    ///
    /// Creating a game allows players to sequentially take turns placing their marker in an empty
    /// spot until a player reaches four in a row or the board is filled and a draw is declared
    ///
    /// # Arguments
    ///
    /// - `player_one` - The first player to make a move
    /// - `player_two` - The second player to make a move
    ///
    /// # Reverts
    ///
    /// - When a player has been blacklisted for cheating
    fn create_game(player_one: Player, player_two: Player) -> Game;

    /// Places a marker from the next player in the game in the specified column
    ///
    /// # Arguments
    ///
    /// - `column` - The column to place a marker in, range 0 <= column < 8
    /// - `game` - The game to make a move in
    ///
    /// # Reverts
    ///
    /// - When a game has ended in a player winning or a draw
    /// - When a marker is placed into a `column` that is full
    fn move(column: u64, game: Game) -> Game;
}
// ANCHOR_END: interface

library;

abi Game {
    /// Starts a new game.
    ///
    /// # Arguments
    ///
    /// * `player_one`: [Identity] - The first player to make a move.
    /// * `player_two`: [Identity] - The second player to make a move.
    ///
    /// # Reverts
    ///
    /// * When there is a game playing.
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads - `1`
    /// * Writes - `14`
    #[storage(read, write)]
    fn new_game(player_one: Identity, player_two: Identity);

    /// Allows a player to make a move at a `position`.
    ///
    /// # Additional Information
    ///
    /// It also determines if the game has been won or drawn.
    ///
    /// # Arguments
    ///
    /// * `position`: [u64] - The position where the player wants to move.
    ///
    /// # Reverts
    ///
    /// * When the game has ended.
    /// * When the wrong player is trying to make a move.
    /// * When a player makes a move out of bounds.
    /// * When a player tries to make a move in an occupied cell.
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads - `8`
    /// * Writes - `3`
    #[storage(read, write)]
    fn make_move(position: u64);

    /// Returns the player positions of the current game as a vector.
    ///
    /// # Returns
    ///
    /// * [Vec<Option<Identity>>] - The current positions of all players on the board.
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads - `1`
    #[storage(read)]
    fn get_board() -> Vec<Option<Identity>>;
}

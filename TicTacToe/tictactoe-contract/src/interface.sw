library;

use ::data_structures::State;

abi Game {
    /// Starts a new game and returns the game id.
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
    /// * Writes - `6`
    #[storage(read, write)]
    fn new_game(player_one: Identity, player_two: Identity) -> u64;

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
    fn make_move(position: u64, game_id: u64);

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
    fn get_board(game_id: u64) -> Vec<Option<bool>>;

    /// Returns the current state of the game.
    ///
    /// # Returns
    ///
    /// * [State] - The current states of the game.
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads - `1`
    #[storage(read)]
    fn get_game_state(game_id: u64) -> State;

    /// Returns the player who's turn it is to make a move.
    ///
    /// # Returns
    ///
    /// * [Identity] - The current player.
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads - `2`
    #[storage(read)]
    fn get_current_player(game_id: u64) -> Option<Identity>;

    /// Returns the players of the current game.
    ///
    /// # Returns
    ///
    /// * [(Identity, Identity)] - A tuple of player 1 and player 2.
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads - `3`
    #[storage(read)]
    fn get_players(game_id: u64) -> Option<(Identity, Identity)>;

    /// Returns the number of moves made in the current game.
    ///
    /// # Returns
    ///
    /// * [u64] - The number of moves in the game
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads - `1`
    #[storage(read)]
    fn get_move_counter(game_id: u64) -> u64;
}

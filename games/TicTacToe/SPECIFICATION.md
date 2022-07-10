# Summary
The game is simple in its original state which consists of
* 3x3 grid
* 2 players
* Each player makes a move in a turn based fashion
* First player to reach 3 in a row wins otherwise it’s a draw
Expansions to game logic (more players, larger boards, more / less markers required to win etc.) and contract data handling (multiple games at once, allowing contracts to interact with the game etc.) can be added later.
# Rough Contract Design
* The contract should support 2 players and they should not be contracts
	* The creator of the game can specify themeselves as both players
* After a player makes a move the next move is automatically set to be made by the next player unless the game has ended
* The game ends in any of the following states
	* Either player has reached 3 in a row
		* This can be in any direction (horizontal, vertical or diagonal)
	* The grid is full and no more moves can be made
* A player cannot place their marker in a position that is already filled
* A player cannot make a move outside of the designated 3x3 grid
* There can only be 1 game played at a time
	* This means that the game itself is stored in storage
	* No other storage is required
* Starting a new game overwrites any previous game even if that game has not ended
* Events should be emitted when
	* A new game is created
		* Who the players are
	* A move is made
		* Which player has made a move and at which position
	* The game ends (draw or if someone won then who won)






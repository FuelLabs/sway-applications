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




# Overview

This document provides an overview of the application.

It outline the use cases, i.e. desirable functionality, in addition to requirements for the smart contract and the user interface.


## Actions that users are able to perform

Users are able to choose the cell where they want to move when their turn comes.

### Game Creation

A user should be able to create a game, and play either alone against himself or with someone else.

1. 1 or 2 players can join the game
2. An empty board is initialized

### End of the game

#### Context

If a player managed to align 3 of a kind horizontaly, verticaly or diagonaly, then he is defined as the winner of the game. Else, if the game board is full but none of the players managed to align 3 of a kind (X or O), then the game ends up in a draw.
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

If a player managed to align 3 of a kind horizontaly, verticaly or diagonaly, then he is defined as the winner of the game. Else, is the game board is full but no one aligned 3 of a kind, then the game ends up in a draw.



### Cancelling a campaign

#### Context

Once a user creates a campaign they may decide that the campaign no longer needs to run therefore they may wish to cancel it

#### Requirements

- The following parameters are required
  - Campaign identifier
    - `type:` u64
    - `restrictions:` 
      - Must be a valid ID (_the identifier uniquely identifies an existing campaign_)
- Only the author of the campaign can attempt to cancel
- Can only cancel a campaign if it is in the Funding phase and before the deadline
- Campaign should be updated to no longer be active
- Log the campaign id

### Helper functions for the user interface

#### Campaign Info

##### Context

The user interface will need to be able to retrieve information about the campaign data structure in order to display it to the users

##### Requirements

- The following parameters are required
  - Campaign identifier
    - `type:` u64
    - `restrictions:` 
      - Must be a valid ID (_the identifier uniquely identifies an existing campaign_)
- The following values should be returned
  - Campaign information
    - `type`: struct

#### Amount pledged by a user

##### Context

The user interface needs to be able to show the user how much they have pledged for each campaign

##### Requirements

- The following parameters are required
  - Campaign identifier
    - `type:` u64
    - `restrictions:` 
      - Must be a valid ID (_the identifier uniquely identifies an existing campaign_)
- The following values should be returned
  - Amount pledged
    - `type`: u64


#### Campaigns created by author

##### Context

The user interface needs to be able to display the currently active campaigns that a user has created and campaigns that are no longer active (completed successfully, failed or cancelled)

##### Requirements

- The following values should be returned
  - Campaigns created by the author (active / completed)
    - `type`: struct

#### Campaign count

##### Context

The user interface needs to know how many campaigns exist in order to aid the queries

##### Requirements

- The following values should be returned
  - Total number of campaigns created
    - `type`: u64

#### Updating the campaign state

##### Context

In order to keep the rest of the functions clean there needs to be another function which checks the deadline and updates the state for the specified campaign.

This is a problem because there needs to be some incentive for this function to be called by someone - anyone.

##### Requirements

- The following parameters are required
  - Campaign identifier
    - `type:` u64
    - `restrictions:` 
      - Must be a valid ID (_the identifier uniquely identifies an existing campaign_)
- If the deadline has been passed and the campaign is in the Funding phase then the state should be updated to either Failed or Successful based on whether the target amount has been reached

## User interface



### Cancelling a campaign

#### Context

Once a user creates a campaign they may decide that the campaign no longer needs to run therefore they may wish to cancel it

#### Requirements

- The following parameters are required
  - Campaign identifier
    - `type:` u64
    - `restrictions:` 
      - Must be a valid ID (_the identifier uniquely identifies an existing campaign_)
- Only the author of the campaign can attempt to cancel
- Can only cancel a campaign if it is in the Funding phase and before the deadline
- Campaign should be updated to no longer be active
- Log the campaign id

### Helper functions for the user interface

#### Campaign Info

##### Context

The user interface will need to be able to retrieve information about the campaign data structure in order to display it to the users

##### Requirements

- The following parameters are required
  - Campaign identifier
    - `type:` u64
    - `restrictions:` 
      - Must be a valid ID (_the identifier uniquely identifies an existing campaign_)
- The following values should be returned
  - Campaign information
    - `type`: struct

#### Amount pledged by a user

##### Context

The user interface needs to be able to show the user how much they have pledged for each campaign

##### Requirements

- The following parameters are required
  - Campaign identifier
    - `type:` u64
    - `restrictions:` 
      - Must be a valid ID (_the identifier uniquely identifies an existing campaign_)
- The following values should be returned
  - Amount pledged
    - `type`: u64


#### Campaigns created by author

##### Context

The user interface needs to be able to display the currently active campaigns that a user has created and campaigns that are no longer active (completed successfully, failed or cancelled)

##### Requirements

- The following values should be returned
  - Campaigns created by the author (active / completed)
    - `type`: struct

#### Campaign count

##### Context

The user interface needs to know how many campaigns exist in order to aid the queries

##### Requirements

- The following values should be returned
  - Total number of campaigns created
    - `type`: u64

#### Updating the campaign state

##### Context

In order to keep the rest of the functions clean there needs to be another function which checks the deadline and updates the state for the specified campaign.

This is a problem because there needs to be some incentive for this function to be called by someone - anyone.

##### Requirements

- The following parameters are required
  - Campaign identifier
    - `type:` u64
    - `restrictions:` 
      - Must be a valid ID (_the identifier uniquely identifies an existing campaign_)
- If the deadline has been passed and the campaign is in the Funding phase then the state should be updated to either Failed or Successful based on whether the target amount has been reached

## User interface



### Cancelling a campaign

#### Context

Once a user creates a campaign they may decide that the campaign no longer needs to run therefore they may wish to cancel it

#### Requirements

- The following parameters are required
  - Campaign identifier
    - `type:` u64
    - `restrictions:` 
      - Must be a valid ID (_the identifier uniquely identifies an existing campaign_)
- Only the author of the campaign can attempt to cancel
- Can only cancel a campaign if it is in the Funding phase and before the deadline
- Campaign should be updated to no longer be active
- Log the campaign id

### Helper functions for the user interface

#### Campaign Info

##### Context

The user interface will need to be able to retrieve information about the campaign data structure in order to display it to the users

##### Requirements

- The following parameters are required
  - Campaign identifier
    - `type:` u64
    - `restrictions:` 
      - Must be a valid ID (_the identifier uniquely identifies an existing campaign_)
- The following values should be returned
  - Campaign information
    - `type`: struct

#### Amount pledged by a user

##### Context

The user interface needs to be able to show the user how much they have pledged for each campaign

##### Requirements

- The following parameters are required
  - Campaign identifier
    - `type:` u64
    - `restrictions:` 
      - Must be a valid ID (_the identifier uniquely identifies an existing campaign_)
- The following values should be returned
  - Amount pledged
    - `type`: u64


#### Campaigns created by author

##### Context

The user interface needs to be able to display the currently active campaigns that a user has created and campaigns that are no longer active (completed successfully, failed or cancelled)

##### Requirements

- The following values should be returned
  - Campaigns created by the author (active / completed)
    - `type`: struct

#### Campaign count

##### Context

The user interface needs to know how many campaigns exist in order to aid the queries

##### Requirements

- The following values should be returned
  - Total number of campaigns created
    - `type`: u64

#### Updating the campaign state

##### Context

In order to keep the rest of the functions clean there needs to be another function which checks the deadline and updates the state for the specified campaign.

This is a problem because there needs to be some incentive for this function to be called by someone - anyone.

##### Requirements

- The following parameters are required
  - Campaign identifier
    - `type:` u64
    - `restrictions:` 
      - Must be a valid ID (_the identifier uniquely identifies an existing campaign_)
- If the deadline has been passed and the campaign is in the Funding phase then the state should be updated to either Failed or Successful based on whether the target amount has been reached

## User interface



### Cancelling a campaign

#### Context

Once a user creates a campaign they may decide that the campaign no longer needs to run therefore they may wish to cancel it

#### Requirements

- The following parameters are required
  - Campaign identifier
    - `type:` u64
    - `restrictions:` 
      - Must be a valid ID (_the identifier uniquely identifies an existing campaign_)
- Only the author of the campaign can attempt to cancel
- Can only cancel a campaign if it is in the Funding phase and before the deadline
- Campaign should be updated to no longer be active
- Log the campaign id

### Helper functions for the user interface

#### Campaign Info

##### Context

The user interface will need to be able to retrieve information about the campaign data structure in order to display it to the users

##### Requirements

- The following parameters are required
  - Campaign identifier
    - `type:` u64
    - `restrictions:` 
      - Must be a valid ID (_the identifier uniquely identifies an existing campaign_)
- The following values should be returned
  - Campaign information
    - `type`: struct

#### Amount pledged by a user

##### Context

The user interface needs to be able to show the user how much they have pledged for each campaign

##### Requirements

- The following parameters are required
  - Campaign identifier
    - `type:` u64
    - `restrictions:` 
      - Must be a valid ID (_the identifier uniquely identifies an existing campaign_)
- The following values should be returned
  - Amount pledged
    - `type`: u64


#### Campaigns created by author

##### Context

The user interface needs to be able to display the currently active campaigns that a user has created and campaigns that are no longer active (completed successfully, failed or cancelled)

##### Requirements

- The following values should be returned
  - Campaigns created by the author (active / completed)
    - `type`: struct

#### Campaign count

##### Context

The user interface needs to know how many campaigns exist in order to aid the queries

##### Requirements

- The following values should be returned
  - Total number of campaigns created
    - `type`: u64

#### Updating the campaign state

##### Context

In order to keep the rest of the functions clean there needs to be another function which checks the deadline and updates the state for the specified campaign.

This is a problem because there needs to be some incentive for this function to be called by someone - anyone.

##### Requirements

- The following parameters are required
  - Campaign identifier
    - `type:` u64
    - `restrictions:` 
      - Must be a valid ID (_the identifier uniquely identifies an existing campaign_)
- If the deadline has been passed and the campaign is in the Funding phase then the state should be updated to either Failed or Successful based on whether the target amount has been reached

## User interface



### Cancelling a campaign

#### Context

Once a user creates a campaign they may decide that the campaign no longer needs to run therefore they may wish to cancel it

#### Requirements

- The following parameters are required
  - Campaign identifier
    - `type:` u64
    - `restrictions:` 
      - Must be a valid ID (_the identifier uniquely identifies an existing campaign_)
- Only the author of the campaign can attempt to cancel
- Can only cancel a campaign if it is in the Funding phase and before the deadline
- Campaign should be updated to no longer be active
- Log the campaign id

### Helper functions for the user interface

#### Campaign Info

##### Context

The user interface will need to be able to retrieve information about the campaign data structure in order to display it to the users

##### Requirements

- The following parameters are required
  - Campaign identifier
    - `type:` u64
    - `restrictions:` 
      - Must be a valid ID (_the identifier uniquely identifies an existing campaign_)
- The following values should be returned
  - Campaign information
    - `type`: struct

#### Amount pledged by a user

##### Context

The user interface needs to be able to show the user how much they have pledged for each campaign

##### Requirements

- The following parameters are required
  - Campaign identifier
    - `type:` u64
    - `restrictions:` 
      - Must be a valid ID (_the identifier uniquely identifies an existing campaign_)
- The following values should be returned
  - Amount pledged
    - `type`: u64


#### Campaigns created by author

##### Context

The user interface needs to be able to display the currently active campaigns that a user has created and campaigns that are no longer active (completed successfully, failed or cancelled)

##### Requirements

- The following values should be returned
  - Campaigns created by the author (active / completed)
    - `type`: struct

#### Campaign count

##### Context

The user interface needs to know how many campaigns exist in order to aid the queries

##### Requirements

- The following values should be returned
  - Total number of campaigns created
    - `type`: u64

#### Updating the campaign state

##### Context

In order to keep the rest of the functions clean there needs to be another function which checks the deadline and updates the state for the specified campaign.

This is a problem because there needs to be some incentive for this function to be called by someone - anyone.

##### Requirements

- The following parameters are required
  - Campaign identifier
    - `type:` u64
    - `restrictions:` 
      - Must be a valid ID (_the identifier uniquely identifies an existing campaign_)
- If the deadline has been passed and the campaign is in the Funding phase then the state should be updated to either Failed or Successful based on whether the target amount has been reached

## User interface



### Cancelling a campaign

#### Context

Once a user creates a campaign they may decide that the campaign no longer needs to run therefore they may wish to cancel it

#### Requirements

- The following parameters are required
  - Campaign identifier
    - `type:` u64
    - `restrictions:` 
      - Must be a valid ID (_the identifier uniquely identifies an existing campaign_)
- Only the author of the campaign can attempt to cancel
- Can only cancel a campaign if it is in the Funding phase and before the deadline
- Campaign should be updated to no longer be active
- Log the campaign id

### Helper functions for the user interface

#### Campaign Info

##### Context

The user interface will need to be able to retrieve information about the campaign data structure in order to display it to the users

##### Requirements

- The following parameters are required
  - Campaign identifier
    - `type:` u64
    - `restrictions:` 
      - Must be a valid ID (_the identifier uniquely identifies an existing campaign_)
- The following values should be returned
  - Campaign information
    - `type`: struct

#### Amount pledged by a user

##### Context

The user interface needs to be able to show the user how much they have pledged for each campaign

##### Requirements

- The following parameters are required
  - Campaign identifier
    - `type:` u64
    - `restrictions:` 
      - Must be a valid ID (_the identifier uniquely identifies an existing campaign_)
- The following values should be returned
  - Amount pledged
    - `type`: u64


#### Campaigns created by author

##### Context

The user interface needs to be able to display the currently active campaigns that a user has created and campaigns that are no longer active (completed successfully, failed or cancelled)

##### Requirements

- The following values should be returned
  - Campaigns created by the author (active / completed)
    - `type`: struct

#### Campaign count

##### Context

The user interface needs to know how many campaigns exist in order to aid the queries

##### Requirements

- The following values should be returned
  - Total number of campaigns created
    - `type`: u64

#### Updating the campaign state

##### Context

In order to keep the rest of the functions clean there needs to be another function which checks the deadline and updates the state for the specified campaign.

This is a problem because there needs to be some incentive for this function to be called by someone - anyone.

##### Requirements

- The following parameters are required
  - Campaign identifier
    - `type:` u64
    - `restrictions:` 
      - Must be a valid ID (_the identifier uniquely identifies an existing campaign_)
- If the deadline has been passed and the campaign is in the Funding phase then the state should be updated to either Failed or Successful based on whether the target amount has been reached

## User interface
TODO


<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset=".docs/escrow-logo-dark-theme.png">
        <img alt="escrow logo" width="400px" src=".docs/escrow-logo-light-theme.png">
    </picture>
</p>

## Overview

This is an on-chain implementation of the classic game of chess.
While a chess engine could possibly be added in the future, the game is currently meant for two players. However, there's nothing stopping one or both of the "players" from being a multisig contract, an off-chain chess engine, or even a fully on-chain chess engine in a contract.

The game can be played either fully on-chain, one move at a time, or "optimistically" via a fraud-provable [state-channel](https://www.jeffcoleman.ca/state-channels/) mechanism.

>State channels are a very broad and simple way to think about blockchain interactions which could occur on the blockchain, but instead get conducted off of the blockchain, without significantly increasing the risk of any participant.
>
>–Jeff Coleman, 2015


Where the firt mode requires a transaction for each move made, optimistic mode allows moves to be
made by exchanging signed states between the players via an off-chain communication channel.
At any point, the current signed game-state and move can be submitted to the contract for validation. A successfully validated move will cause the contract to update the stored state for the current game.
Taken to the extreme, with the exception of game initialization and verification of the final move, the entire game can be played off-chain while still remaining trustless and verifiable.

## Project Structure
The project consists of both a smart contract and a predicate.

<!--Only show most important files e.g. script to run, build etc.-->

```
chess-game/
├── Forc.toml
├── README.md
├── SPECIFICATION.md
├── game_flow.mmd
└── project
    ├── arbiter_predicate
    │   ├── Forc.toml
    │   └── src
    │       └── main.sw
    └── chess_contract
        ├── Forc.toml
        └── src
            └── main.sw
```

## Running the Project
___
### User Interface

TODO: UI does not currently exist


### Tests

In order to run the tests make sure that you are in the root of this project i.e. `/path/to/chess-game/<you are here>`

Build the contracts:

```bash
forc build
```

Run the tests:

```bash
cargo test
```

## Specification

The specification contains a non-technical overview of the contract indicating the flow of information from the start to the end of a game.

Check [SPECIFICATION.md](./SPECIFICATION.md) for more info!
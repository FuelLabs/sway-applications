<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset=".docs/chess_logo-dark-theme.png">
        <img alt="chess logo" width="400px" src=".docs/chess_logo-light-theme.png">
    </picture>
</p>

## Overview

This is an on-chain implementation of the classic game of chess for two players.

The game can be played either fully on-chain, one move at a time, or "optimistically" via a fraud-provable [state-channel](https://www.jeffcoleman.ca/state-channels/) mechanism.

>State channels are a very broad and simple way to think about blockchain interactions which could occur on the blockchain, but instead get conducted off of the blockchain, without significantly increasing the risk of any participant.
>
>–Jeff Coleman, 2015


Where the first mode requires a transaction for each move made, optimistic mode allows moves to be
made by exchanging signed states between the players via an off-chain communication channel.
At any point, the current signed game-state and move can be submitted to the contract for validation. A successfully validated move will cause the contract to update the stored state for the current game.
Taken to the extreme, with the exception of game initialization and verification of the final move, the entire game can be played off-chain while still remaining trustless and verifiable.

This game is currently meant for two players. However, there's nothing stopping one or both of the "players" from being a multisig contract, an off-chain chess engine, or even a fully on-chain chess engine in a contract.

## Project Structure

WIP: The project consists of a library and a contract.

<!--Only show most important files e.g. script to run, build etc.-->

```
chess_game
├── DATA_STRUCTURES.mmd
├── Forc.lock
├── Forc.toml
├── README.md
├── SEQUENCE.mmd
├── SPECIFICATION.md
└── project
    ├── chess
    │   ├── Forc.toml
    │   ├── out
    │   │   └── debug
    │   │       └── chess.bin
    │   └── src
    │       ├── bitboard.sw
    │       ├── bitmaps.sw
    │       ├── board.sw
    │       ├── errors.sw
    │       ├── events.sw
    │       ├── game.sw
    │       ├── lib.sw
    │       ├── move.sw
    │       ├── piece.sw
    │       ├── special.sw
    │       ├── square.sw
    │       └── utils.sw
    └── chess_contract
        ├── Forc.toml
        └── src
            └── main.sw
```

## Running the Project

TODO: WIP
___
### User Interface

TODO: UI does not currently exist


### Tests

In order to run the tests make sure that you are in the root of this project i.e. `/path/to/chess-game/<you are here>`

Build the workspace members:

```bash
forc build
```

Run the tests:

```bash
forc test && cargo test
```

## Specification

The specification contains a non-technical overview of the chess game.

Check [SPECIFICATION.md](./SPECIFICATION.md) for more info!
/* Autogenerated file. Do not edit manually. */

/* tslint:disable */
/* eslint-disable */

/*
  Fuels version: 0.76.0
  Forc version: 0.51.1
  Fuel-Core version: 0.22.1
*/

import type {
  BigNumberish,
  BN,
  BytesLike,
  Contract,
  DecodedValue,
  FunctionFragment,
  Interface,
  InvokeFunction,
} from 'fuels';

import type { Option, Enum, Vec } from "./common";

export enum GameStateErrorInput { GameHasEnded = 'GameHasEnded', GameHasNotEnded = 'GameHasNotEnded' };
export enum GameStateErrorOutput { GameHasEnded = 'GameHasEnded', GameHasNotEnded = 'GameHasNotEnded' };
export type IdentityInput = Enum<{ Address: AddressInput, ContractId: ContractIdInput }>;
export type IdentityOutput = Enum<{ Address: AddressOutput, ContractId: ContractIdOutput }>;
export enum PlayerErrorInput { IncorrectPlayerTurn = 'IncorrectPlayerTurn' };
export enum PlayerErrorOutput { IncorrectPlayerTurn = 'IncorrectPlayerTurn' };
export enum PositionErrorInput { CellIsNotEmpty = 'CellIsNotEmpty', InvalidPosition = 'InvalidPosition' };
export enum PositionErrorOutput { CellIsNotEmpty = 'CellIsNotEmpty', InvalidPosition = 'InvalidPosition' };
export enum StateInput { Playing = 'Playing', Ended = 'Ended' };
export enum StateOutput { Playing = 'Playing', Ended = 'Ended' };

export type AddressInput = { value: string };
export type AddressOutput = AddressInput;
export type ContractIdInput = { value: string };
export type ContractIdOutput = ContractIdInput;
export type GameDrawnEventInput = { player_one: IdentityInput, player_two: IdentityInput };
export type GameDrawnEventOutput = { player_one: IdentityOutput, player_two: IdentityOutput };
export type GameWonEventInput = { player: IdentityInput };
export type GameWonEventOutput = { player: IdentityOutput };
export type NewGameEventInput = { player_one: IdentityInput, player_two: IdentityInput };
export type NewGameEventOutput = { player_one: IdentityOutput, player_two: IdentityOutput };

interface TictactoeContractAbiInterface extends Interface {
  functions: {
    get_board: FunctionFragment;
    get_current_player: FunctionFragment;
    get_game_state: FunctionFragment;
    get_players: FunctionFragment;
    make_move: FunctionFragment;
    new_game: FunctionFragment;
  };

  encodeFunctionData(functionFragment: 'get_board', values: []): Uint8Array;
  encodeFunctionData(functionFragment: 'get_current_player', values: []): Uint8Array;
  encodeFunctionData(functionFragment: 'get_game_state', values: []): Uint8Array;
  encodeFunctionData(functionFragment: 'get_players', values: []): Uint8Array;
  encodeFunctionData(functionFragment: 'make_move', values: [BigNumberish]): Uint8Array;
  encodeFunctionData(functionFragment: 'new_game', values: [IdentityInput, IdentityInput]): Uint8Array;

  decodeFunctionData(functionFragment: 'get_board', data: BytesLike): DecodedValue;
  decodeFunctionData(functionFragment: 'get_current_player', data: BytesLike): DecodedValue;
  decodeFunctionData(functionFragment: 'get_game_state', data: BytesLike): DecodedValue;
  decodeFunctionData(functionFragment: 'get_players', data: BytesLike): DecodedValue;
  decodeFunctionData(functionFragment: 'make_move', data: BytesLike): DecodedValue;
  decodeFunctionData(functionFragment: 'new_game', data: BytesLike): DecodedValue;
}

export class TictactoeContractAbi extends Contract {
  interface: TictactoeContractAbiInterface;
  functions: {
    get_board: InvokeFunction<[], Vec<Option<IdentityOutput>>>;
    get_current_player: InvokeFunction<[], Option<IdentityOutput>>;
    get_game_state: InvokeFunction<[], StateOutput>;
    get_players: InvokeFunction<[], Option<[IdentityOutput, IdentityOutput]>>;
    make_move: InvokeFunction<[position: BigNumberish], void>;
    new_game: InvokeFunction<[player_one: IdentityInput, player_two: IdentityInput], void>;
  };
}
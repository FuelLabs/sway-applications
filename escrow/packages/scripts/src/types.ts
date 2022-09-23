import type { BytesLike, CreateTransactionRequestLike, StorageSlot } from 'fuels';

export type DeployContractOptions = {
  salt?: BytesLike;
  storageSlots?: StorageSlot[];
  stateRoot?: BytesLike;
} & CreateTransactionRequestLike;

export enum Commands {
  'build' = 'build',
  'deploy' = 'deploy',
  'types' = 'types',
  'run' = 'run',
}

export type BuildDeploy = {
  name: string;
  contractId: string;
};

export type Event =
  | {
      type: Commands.build;
      data: unknown;
    }
  | {
      type: Commands.deploy;
      data: Array<BuildDeploy>;
    }
  | {
      type: Commands.run;
      data: Array<BuildDeploy>;
    };

export type OptionsFunction = (contracts: Array<ContractDeployed>) => DeployContractOptions;

export type ContractConfig = {
  name: string;
  path: string;
  options?: DeployContractOptions | OptionsFunction;
};

export type ContractDeployed = {
  name: string;
  contractId: string;
};

export type Config = {
  onSuccess?: (event: Event) => void;
  onFailure?: (err: unknown) => void;
  env?: {
    [key: string]: string;
  };
  types: {
    artifacts: string;
    output: string;
  };
  contracts: Array<ContractConfig>;
};

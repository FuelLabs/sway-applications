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
    contracts: {
      name: string;
      path: string;
    }[];
  };
  
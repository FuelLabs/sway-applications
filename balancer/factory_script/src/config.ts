export type Config = {
  types: {
    artifacts: string;
    output: string;
  };
  contracts: {
    name: string;
    path: string;
  }[];
};

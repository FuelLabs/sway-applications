## Egnlish Auction Scripts

This package has scripts to help integrate Sway contracts
with the English Auction application. This makes it easier for developers to
change contracts while building a nice UI

### Execute

```sh
pnpm exec english-auction-scripts [command]
```

### Options

```sh
Usage: English-Auction Scripts [options] [command]

Utility to build, deploy and generate types for Sway Contracts

Options:
  -h, --help      display help for command

Commands:
  build           Build sway contracts and generate type
  deploy          deploy contract to fuel network
  run             build and deploy contracts to fuel network
  help [command]  display help for command
```

### Config

```
{
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
}
```

See complete [types here](./src/types.ts).

## English Auction Scripts

This package has scripts to help integrate Sway contracts with the English Auction application. This makes it easier for developers to change contracts while building a user interface.

To use this package you must install the dependencies from the root of the monorepo i.e `/path/to/english-auction/`.

```bash
pnpm install
```

Also you must be in the repo root to run the package commands.

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

See complete [types here](./src/types.ts).
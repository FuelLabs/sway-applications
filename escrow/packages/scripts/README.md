## Escrow Scripts

This package helps integrate Sway contracts with the Escrow frontend application.  It makes it easier for developers to change the smart contracts while building a nice UI.

### Execute Commands

```bash
pnpm exec escrow-scripts [command]
```

### Command Options

```bash
Usage: Escrow Scripts [options] [command]

Utility to build, deploy, and generate types for Sway Contracts

Options:
    -h, --help

Commands:
    build           build sway contract(s) and generate types
    deploy          deploy contract(s) to the fuel network
    run             build and deploy contracts to the fuel network
    help [command]  display help for command
```

### Environment Variables
| name           | default          | description                                                                                                                                            |
| -------------- | ---------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------ |
| WALLET_SECRET  | empty            | Wallet secret used to deploy contracts                                                                                                                 |
| GENESIS_SECRET | empty            | Genesis secret used when WALLET_SECRET is not present it creates a new wallet and seeds values from genesis to the new wallet and deploys the contract |
| PROVIDER_URL   | fuels-ts default | Fuel network url                                                                                                                                       |
| GAS_PRICE      | 0                | Specified the gas price used to send the tx                                                                                                            |

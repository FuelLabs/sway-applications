## Start the user interface

### Build the contracts

Change into `/multisig-wallet/project` and run

```sh
forc build
```

### Start a local node

Run the script `start_node.sh` which is inside `/multisig-wallet/ui/scripts`

### Deploy contracts to a local node

Change into `/multisig-wallet/project` and run

```sh
forc deploy --unsigned
```

### Install dependencies

Change into `/multisig-wallet/ui/app` and run

```sh
pnpm install
```

### Start the user interface in your default browser

Inside `/multisig-wallet/ui/app` run

```sh
pnpm run dev --open
```

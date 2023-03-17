## Start the user interface

### Contracts

Run the following commands from the following directory: `/multisig-wallet/project`

#### Build the contracts

```sh
forc build
```

#### Run a local client

To fund your local browser wallet copy the address and put it into the `chainConfig.json`

```sh
fuel-core run --ip 127.0.0.1 --port 4000 --chain ../ui/config/chainConfig.json --db-type in-memory
```

#### Deploy the contract

```sh
forc deploy --unsigned
```

### User interface

The user interface is run inside of `docker`.

Change into `/multisig-wallet/ui/config`

#### Build the image

```sh
docker-compose build user-interface
```

#### Start the user interface

```sh
docker-compose run user-interface
```

The terminal will present a `Network` URL. Copy that into your browser to open the UI.

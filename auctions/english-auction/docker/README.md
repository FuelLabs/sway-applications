## Fuel network environment

We enable developers to locally run an entire environment with a
`fuel-core` network

### Environment variables

| name           | default     | description                                                                                                          |
| -------------- | ----------- | -------------------------------------------------------------------------------------------------------------------- |
| ENVIRONMENT    | development | This is used to append on volume and container name to enable multiple envs like test                                |
| WALLET_SECRET  |             | Private key which corresponds to the first address initialized with assets in the `chainConfig.json`. This key is used in `english-auction-scripts` to deploy contracts. |
| WALLET_MNEMONIC | | Mnemonic which corresponds to the second and third addresses initialized with assets in the `chainConfig.jsion`.  This mnemonic is used in `app/scripts` to initialize the NFT contract.
| FUEL_CORE_PORT | 4001        | Fuel network PORT                                                                                                    |
| GAS_PRICE      | 1           | Set Fuel Core `--min-gas-price`                                                                                      |

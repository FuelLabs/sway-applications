## Fuel network environment with a Faucet API

We enable developers to run locally a entire env with a
`fuel-core` network and `faucet` api running together

### Environment variables

| name             | default     | description                                                                                                          |
| ---------------- | ----------- | -------------------------------------------------------------------------------------------------------------------- |
| ENVIRONMENT      | development | This is used to append on volume and container name to enable multiple envs like test                                |
| WALLET_SECRET    |             | Secret used on the faucet API, by default we use the same `privateKey` used on the genesis config `chainConfig.json` |
| FUEL_CORE_PORT   | 4000        | Fuel network PORT                                                                                                    |
| FUEL_FAUCET_PORT | 4040        | Faucet API PORT                                                                                                      |
| DISPENSE_AMOUNT  | 50000000    | Faucet dispense amount                                                                                               |
| GAS_PRICE        | 1           | Set Fuel Core `--min-gas-price`                                                                                      |

# fuel-ts factory

fuel-ts factory for building, deploying and calling the contract's abi functions.

## Steps to run the factory script

### Local fuel node

Start a local fuel node by running `fuel-core run --chain ./chainConfig.json`.
This step will start and initialize the local fuel node instance with a wallet having some balance.
If you want to use different wallet instead then you can create a new wallet. 
Add the public key in [initial_state](https://github.com/Rapid-Innovation/blockchain-sway-contract-fuel/blob/477ad0173c881636c404364931ea7701def3483e/factory_script/chainConfig.json#L7) - 

```
    "initial_state": {
      "coins": [
        {
          "owner": "0x54944e5b8189827e470e5a8bacfc6c3667397dc4e1eef7ef3519d16d6d6c6610",
          "amount": "0x000000000000FFFF",
          "asset_id": "0x0000000000000000000000000000000000000000000000000000000000000000"
        }
      ]
    },
```

Change the owner to your new generated wallet's pubkey. Also you can add other assets and their amount as well in your wallet.
Use the private key in [TypeScript](https://github.com/Rapid-Innovation/blockchain-sway-contract-fuel/blob/83e96845e0a3f1bec3608d114da68463dd27cdf7/factory_script/src/main.ts#L25) to create a wallet instance.


### Adding contracts

You need to add the paths to the contracts that you want to build and deploy.
A config type has been created where you can configure the [contracts](https://github.com/Rapid-Innovation/blockchain-sway-contract-fuel/blob/83e96845e0a3f1bec3608d114da68463dd27cdf7/factory_script/src/main.ts#L7), their names and paths.


### Types

Types will be generated for the contracts that has been built. [Config for types](https://github.com/Rapid-Innovation/blockchain-sway-contract-fuel/blob/83e96845e0a3f1bec3608d114da68463dd27cdf7/factory_script/src/main.ts#L8). 
*For now the factory generates types for all the contracts from the parent directory where all the contracts reside*. 

### Contract ABI Function calls

After the contracts are built and deployed we can call the [contract's abi functions](https://github.com/Rapid-Innovation/blockchain-sway-contract-fuel/blob/one_contract_architecture/factory_script/src/calls/setAuthorizerCall.ts), you need to pass the contractId and wallet or any sort of assets while calling the contract. New functions can be added as soon as more contracts are deployed.

### Run the script

1. You need to get all the required node modules to run the script. Do that using -

`npm install`

2. To run the script - 

`npm run start`


## Known issue

While generating types for few contracts there is this error that occurs from the `typechain-target-fuels` library.

```
Error: Can't guess class name, please rename file: 
    at d (/home/sahil/work/projects/fuel/contracts-sway-blockchain-fuel/factory_script/node_modules/typechain-target-fuels/src/parser/parseSvmTypes.ts:128:11)
    at S (/home/sahil/work/projects/fuel/contracts-sway-blockchain-fuel/factory_script/node_modules/typechain-target-fuels/src/parser/parseSvmTypes.ts:192:19)
    at W (/home/sahil/work/projects/fuel/contracts-sway-blockchain-fuel/factory_script/node_modules/typechain-target-fuels/src/parser/abiParser.ts:124:18)
    at V (/home/sahil/work/projects/fuel/contracts-sway-blockchain-fuel/factory_script/node_modules/typechain-target-fuels/src/parser/abiParser.ts:106:11)
    at /home/sahil/work/projects/fuel/contracts-sway-blockchain-fuel/factory_script/node_modules/typechain-target-fuels/src/parser/abiParser.ts:163:60
    at Array.map (<anonymous>)
    at dt (/home/sahil/work/projects/fuel/contracts-sway-blockchain-fuel/factory_script/node_modules/typechain-target-fuels/src/parser/abiParser.ts:163:49)
    at gt (/home/sahil/work/projects/fuel/contracts-sway-blockchain-fuel/factory_script/node_modules/typechain-target-fuels/src/parser/abiParser.ts:92:14)
    at /home/sahil/work/projects/fuel/contracts-sway-blockchain-fuel/factory_script/node_modules/typechain-target-fuels/src/parser/abiParser.ts:195:22
```

Currently looking for a fix.

Full issue - https://discord.com/channels/732892373507375164/734213700835082330/1013737638701842432

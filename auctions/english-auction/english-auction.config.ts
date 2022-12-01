import dotenv from 'dotenv';
import { createConfig, replaceEventOnEnv } from 'english-auction-scripts';

const { NODE_ENV, OUTPUT_ENV } = process.env;

function getEnvName() {
  if (NODE_ENV === 'test') {
    return '.env.test';
  } else if (NODE_ENV === 'testnet') {
    return '.env.testnet';
  }
  return '.env';
}

dotenv.config({
  path: `./docker/${getEnvName()}`,
});

const getDeployOptions = () => ({
  gasPrice: Number(process.env.GAS_PRICE || 0),
});

export default createConfig({
  types: {
    artifacts: './packages/contracts/**/out/debug/**-abi.json',
    output: './packages/app/src/types/contracts',
  },
  contracts: [
    {
      name: 'VITE_TOKEN_ID',
      buildPath: './packages/contracts',
      deployPath: './packages/contracts/english-auction/project/auction-contract/tests/artifacts/asset',
      options: getDeployOptions(),
    },
    {
      name: 'VITE_CONTRACT_ID',
      buildPath: './packages/contracts',
      deployPath: './packages/contracts/english-auction/project/auction-contract',
      options: getDeployOptions(),
    },
    {
      name: 'VITE_NFT_ID',
      buildPath: './packages/contracts',
      deployPath: '../../NFT/project/NFT-contract',
      options: getDeployOptions(),
    },
  ],
  onSuccess: (event) => {
    replaceEventOnEnv(`./packages/app/${OUTPUT_ENV || getEnvName()}`, event);
  },
});

import dotenv from 'dotenv';
import { createConfig, replaceEventOnEnv } from 'escrow-scripts';

const { NODE_ENV, OUTPUT_ENV } = process.env;

function getEnvName() {
  return NODE_ENV === 'test' ? '.env.test' : '.env';
}

dotenv.config({
  path: `./docker/${getEnvName()}`,
});

export default createConfig({
  types: {
    artifacts: './packages/contracts/**/out/debug/**-abi.json',
    output: './packages/frontend/src/types/contracts',
  },
  contracts: [
    {
      name: 'VITE_CONTRACT_ID',
      path: './packages/contracts/escrow',
    },
    {
      name: 'VITE_TOKEN_ID',
      path: './packages/contracts/escrow/tests/artifacts/asset',
    },
  ],
  onSuccess: (event) => {
    replaceEventOnEnv(`./packages/frontend/${OUTPUT_ENV || getEnvName()}`, event);
  },
});

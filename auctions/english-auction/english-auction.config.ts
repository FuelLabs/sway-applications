import dotenv from 'dotenv';
import { createConfig, replaceEventOnEnv } from 'swayswap-scripts';

const { NODE_ENV, OUTPUT_ENV } = process.env;

function getEnvName() {
  return NODE_ENV === 'test' ? '.env.test' : '.env';
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
      path: './packages/contracts/token_contract',
      options: getDeployOptions(),
    },
    {
      name: 'VITE_CONTRACT_ID',
      path: './packages/contracts/exchange_contract',
      options: (contracts) => {
        const contractDeployed = contracts.find((c) => c.name === 'VITE_TOKEN_ID')!;
        return {
          ...getDeployOptions(),
          storageSlots: [
            {
              key: '0x0000000000000000000000000000000000000000000000000000000000000001',
              value: contractDeployed.contractId,
            },
          ],
        };
      },
    },
  ],
  onSuccess: (event) => {
    replaceEventOnEnv(`./packages/app/${OUTPUT_ENV || getEnvName()}`, event);
  },
});

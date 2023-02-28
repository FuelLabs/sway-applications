import dotenv from 'dotenv';
import { createConfig, Commands, log } from 'english-auction-scripts';
import type { Event } from 'english-auction-scripts';
import { readFile, writeFile, access, copyFile } from 'fs/promises';

const { NODE_ENV, OUTPUT_ENV } = process.env;

function getEnvName() {
  if (NODE_ENV === 'test') {
    return '.env.test';
  }
  if (NODE_ENV === 'testnet') {
    return '.env.testnet';
  }
  return '.env';
}

/**
 * Use event output data to replace
 * on the provide path env the new
 * contract ids.
 *
 * It uses the name inform on the config.contracts.name
 * as a key to the new value. If it didn't found the key
 * on the provide path nothing happens
 */
export async function replaceEventOnEnv(path: string, event: Event) {
  if (event.type === Commands.deploy || event.type === Commands.run) {
    log(`Reading file from ${path}`);
    try {
      await access(path);
    } catch (e: unknown) {
      // If the env file does not exist yet
      // Create it by copying the example env
      if (path.slice(-4) === 'test') {
        await copyFile('./packages/app/.env.example.test', path);
      } else {
        await copyFile('./packages/app/.env.example', path);
      }
    }
    const fileEnv = (await readFile(path)).toString();
    // Replace new ides on .env file
    const newEnvFile = event.data.reduce((file, { name, contractId }) => {
      log(`Replace env ${name} with ${contractId}`);
      // Replace key with new value
      return file.replace(new RegExp(`(${name}=).*`), `$1${contractId}`);
    }, fileEnv);
    log(`Updating ${path} with new contract ids`);
    await writeFile(path, newEnvFile);
    log(`${path} contract updates!`);
  }
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
      deployPath:
        './packages/contracts/english-auction/project/auction-contract/tests/artifacts/asset',
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
      deployPath:
        './packages/contracts/english-auction/project/auction-contract/tests/artifacts/NFT',
      options: getDeployOptions(),
    },
  ],
  onSuccess: (event) => {
    replaceEventOnEnv(`./packages/app/${OUTPUT_ENV || getEnvName()}`, event);
  },
  isWorkspace: true,
});

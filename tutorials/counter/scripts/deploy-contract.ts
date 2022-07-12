import { TestUtils } from '@fuel-ts/wallet';
import { config } from 'dotenv';
import fs from 'fs';
import { ContractFactory, NativeAssetId, Provider, Wallet } from 'fuels';
import path from 'path';

import { CounterContractAbi__factory as Factory } from '../src/counter-contract-types';

config({ path: path.resolve(__dirname, '../.env') });

async function main(): Promise<string> {
  const provider = new Provider(process.env.FUEL_PROVIDER_URL as string);
  const wallet = new Wallet(process.env.PRIVATE_KEY as string, provider);
  await TestUtils.seedWallet(wallet, [[5_000, NativeAssetId]]);

  const bytecode = fs.readFileSync(path.join(__dirname, '../out/debug/counter-contract.bin'));
  const factory = new ContractFactory(bytecode, Factory.abi, wallet);
  const contract = await factory.deployContract();
  return contract.id;
}

main().then((contractId) =>
  // eslint-disable-next-line no-console
  console.log(
    `Deployed contract details, please add to your '.env' file:
CONTRACT_ID="${contractId}"
    `
  )
);

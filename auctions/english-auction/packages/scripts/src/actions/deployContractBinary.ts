import { readFileSync } from 'fs';
import type { JsonAbi, Wallet } from 'fuels';
import { ContractFactory } from 'fuels';
import path from 'path';
import { log } from 'src/log';
import type { DeployContractOptions } from 'src/types';

function getBinaryName(contractPath: string) {
  const fileName = contractPath.split('/').slice(-1);
  return `/out/debug/${fileName}.bin`;
}

function getABIName(contractPath: string) {
  const fileName = contractPath.split('/').slice(-1);
  return `/out/debug/${fileName}-abi.json`;
}

export async function deployContractBinary(
  wallet: Wallet,
  binaryPath: string,
  options?: DeployContractOptions
) {
  if (!wallet) {
    throw new Error('Cannot deploy without wallet');
  }
  const binaryFilePath = path.join(binaryPath, getBinaryName(binaryPath));
  const abiFilePath = path.join(binaryPath, getABIName(binaryPath));
  log('read binary file from: ', binaryFilePath);
  const bytecode = readFileSync(binaryFilePath);
  const abiJSON = JSON.parse(readFileSync(abiFilePath).toString()) as JsonAbi;
  const contractFactory = new ContractFactory(bytecode, abiJSON, wallet);

  log('deploy contract');
  const contract = await contractFactory.deployContract({
    gasLimit: 1_000_000,
    storageSlots: [],
    ...options,
  });
  log('contract successful deployed');
  return contract.id.toB256();
}

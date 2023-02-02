import { readFileSync } from 'fs';
import { ContractFactory } from 'fuels';
import type { JsonAbi, WalletUnlocked } from 'fuels';
import path from 'path';
import { getBinaryName, getABIName } from 'src/helpers/fileNames';
import { log } from 'src/log';
import type { DeployContractOptions } from 'src/types';

export async function deployContractBinary(
  wallet: WalletUnlocked,
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
    storageSlots: [],
    ...options,
  });
  log('contract successful deployed');
  return contract.id.toB256();
}

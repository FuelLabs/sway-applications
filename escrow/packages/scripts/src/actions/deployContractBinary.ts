import { readFileSync } from 'fs';
import type { Wallet } from 'fuels';
import { CreateTransactionRequest, ZeroBytes32, ContractUtils } from 'fuels';
import path from 'path';
import { log } from 'src/log';

function getBinaryName(contractPath: string) {
  const fileName = contractPath.split('/').slice(-1);
  return `/out/debug/${fileName}.bin`;
}

export async function deployContractBinary(wallet: Wallet, binaryPath: string) {
  const { GAS_PRICE, BYTE_PRICE } = process.env;
  if (!wallet) {
    throw new Error('Cannot deploy without wallet');
  }
  const binaryFilePath = path.join(binaryPath, getBinaryName(binaryPath));
  log('read binary file from: ', binaryFilePath);
  const bytecode = readFileSync(binaryFilePath);
  // Calculate contractId
  const stateRoot = ZeroBytes32;
  const contractId = ContractUtils.getContractId(bytecode, ZeroBytes32, stateRoot);
  const request = new CreateTransactionRequest({
    gasPrice: GAS_PRICE || 0,
    bytePrice: BYTE_PRICE || 0,
    gasLimit: 1_000_000,
    bytecodeWitnessIndex: 0,
    witnesses: [bytecode],
  });
  // Deploy contract using wallet
  log('deploy contract');
  request.addContractCreatedOutput(contractId, stateRoot);
  // Add input coins on the wallet to deploy contract
  await wallet.fund(request);
  // Send deploy transaction
  log('send [CREATE CONTRACT] transaction to', wallet.provider.url);
  const response = await wallet.sendTransaction(request);
  // Await contract to be fully deployed
  log('Waiting contract to be fully deployed');
  await response.wait();
  log('contract successful deployed');
  return contractId;
}

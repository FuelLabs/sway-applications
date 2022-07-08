import fs from 'fs';
import {
  ContractFactory,
  NativeAssetId,
  ScriptTransactionRequest,
  Wallet,
  ZeroBytes32,
} from 'fuels';
import type { Interface, JsonAbi } from 'fuels';

export const deployContractBinary = async (contextLog: string, binaryPath: string, abi: JsonAbi | Interface, wallet: Wallet) => {
    // Deploy
    console.log(contextLog, 'Load contract binary...');
    const bytecode = fs.readFileSync(binaryPath);
    console.log(contextLog, 'Deploy contract...');
    const factory = new ContractFactory(bytecode, abi, wallet);
    const contract = await factory.deployContract({
      salt: ZeroBytes32,
      stateRoot: ZeroBytes32,
    });
  
    console.log(contextLog, 'Contract deployed...');
    return contract;
}
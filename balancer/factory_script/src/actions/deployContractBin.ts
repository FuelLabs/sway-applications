import {
  Wallet,
  CreateTransactionRequest,
  ZeroBytes32,
  ContractUtils,
} from "fuels";
import path from "path";
import { readFileSync } from "fs";

const getBinName = (contractPath: string) => {
  const fileName = contractPath.split("/").slice(-1);
  return `/out/debug/${fileName}.bin`;
};

export const deployContractBin = async (wallet: Wallet, binPath: string) => {
  if (!wallet) {
    throw new Error("Cannot deploy without wallet");
  }

  const binFilePath = path.join(binPath, getBinName(binPath));
  console.log("Deploying", binFilePath.toString());
  const bytecode = readFileSync(binFilePath);

  const stateRoot = ZeroBytes32;
  const contractId = ContractUtils.getContractId(
    bytecode,
    ZeroBytes32,
    stateRoot
  );

  const request = new CreateTransactionRequest({
    gasPrice: 1_000_000,
    bytePrice: 1_000_000,
    gasLimit: 1_000_000,
    bytecodeWitnessIndex: 0,
    witnesses: [bytecode],
  });
  
  await wallet.fund(request);
  request.addContractCreatedOutput(contractId, stateRoot);
  const response = await wallet.sendTransaction(request);
  await response.wait();

  return contractId;
};

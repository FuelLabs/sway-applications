import { Wallet } from "fuels";
import type { Config } from "../config";
import { deployContractBin } from "./deployContractBin";

export async function deployContracts(config: Config, wallet: Wallet) {
  const contracts = [];

  for (const { name, path } of config.contracts) {
    contracts.push({
      name,
      contractId: await deployContractBin(wallet, path),
    });
  }

  return contracts;
}

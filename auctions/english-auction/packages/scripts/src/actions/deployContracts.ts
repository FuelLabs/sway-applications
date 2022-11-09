/* eslint-disable no-restricted-syntax */
import type { Config, ContractDeployed, DeployContractOptions } from 'src/types';

import { deployContractBinary } from './deployContractBinary';
import { getWalletInstance } from './getWalletInstance';

export async function deployContracts(config: Config) {
  const wallet = await getWalletInstance();
  const contracts: Array<ContractDeployed> = [];

  for (const { name, path, options } of config.contracts) {
    let contractOptions: DeployContractOptions | undefined;

    if (typeof options === 'function') {
      contractOptions = options(contracts);
    } else if (typeof options === 'object') {
      contractOptions = options;
    }

    contracts.push({
      name,
      contractId: await deployContractBinary(wallet, path, contractOptions),
    });
  }

  return contracts;
}

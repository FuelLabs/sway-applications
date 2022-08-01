/* eslint-disable no-restricted-syntax */
import type { Config } from 'src/types';

import { deployContractBinary } from './deployContractBinary';
import { getWalletInstance } from './getWalletInstance';

export async function deployContracts(config: Config) {
  const wallet = await getWalletInstance();
  const contracts = [];

  for (const { name, path } of config.contracts) {
    contracts.push({
      name,
      contractId: await deployContractBinary(wallet, path),
    });
  }

  return contracts;
}

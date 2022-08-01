import type { Config } from '../types';

import { buildContracts } from './buildContracts';
import { deployContracts } from './deployContracts';

export async function runAll(config: Config) {
  await buildContracts(config);
  const contractIds = await deployContracts(config);
  return contractIds;
}

/* eslint-disable no-restricted-syntax */

import type { Config } from 'src/types';

import { buildContract } from './buildContract';
import { buildTypes } from './buildTypes';
import { prettifyContracts } from './prettifyContracts';

export async function buildContracts(config: Config) {
  if (config.isWorkspace) {
    await buildContract(config.contracts[0].buildPath);
  } else {
    for (const { buildPath } of config.contracts) {
      await buildContract(buildPath);
    }
  }
  await buildTypes(config);
  await prettifyContracts(config);
}

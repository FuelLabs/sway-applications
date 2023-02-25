/* eslint-disable no-restricted-syntax */

import type { Config } from 'src/types';

import { buildContract } from './buildContract';
import { buildTypes } from './buildTypes';
import { prettifyContracts } from './prettifyContracts';

export async function buildContracts(config: Config) {
  for (const { buildPath } of config.contracts) {
    await buildContract(buildPath);
  }
  await buildTypes(config);
  await prettifyContracts(config);
}

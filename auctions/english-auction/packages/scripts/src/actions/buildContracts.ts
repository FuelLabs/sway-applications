/* eslint-disable no-restricted-syntax */

import type { Config } from 'src/types';

import { buildContract } from './buildContract';
import { buildTypes } from './buildTypes';
import { prettifyContracts } from './prettifyContracts';

export async function buildContracts(config: Config) {
  for (const { path } of config.contracts) {
    await buildContract(path);
  }
  await buildTypes(config);
  await prettifyContracts(config);
}

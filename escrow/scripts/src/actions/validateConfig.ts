import type { Config } from 'src/types';

export function validateConfig(config: Config) {
  if (!Array.isArray(config.contracts)) {
    throw new Error('config.contract should be a valid array');
  }
  if (typeof config.types.artifacts !== 'string') {
    throw new Error('config.types.artifacts should be a valid string');
  }
  if (typeof config.types.output !== 'string') {
    throw new Error('config.types.output should be a valid string');
  }
}

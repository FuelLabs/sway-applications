import { bundleRequire } from 'bundle-require';
import JoyCon from 'joycon';
import path from 'path';
import { validateConfig } from 'src/actions/validateConfig';
import type { Config } from 'src/types';

export async function loadConfig(cwd: string): Promise<Config> {
  const configJoycon = new JoyCon();
  const configPath = await configJoycon.resolve({
    files: ['swayswap.config.js', 'swayswap.config.ts'],
    cwd,
    stopDir: path.parse(cwd).root,
    packageKey: 'tsup',
  });

  if (configPath) {
    const result = await bundleRequire({
      filepath: configPath,
    });
    const config = result.mod.default;

    if (config.env) {
      // If env config is provide override current
      // process.env with new envs
      Object.assign(process.env, config.env);
    }

    validateConfig(config);

    return config;
  }

  return {
    types: {
      artifacts: '',
      output: '',
    },
    contracts: [],
  };
}

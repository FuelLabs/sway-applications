import { spawn } from 'child_process';
import type { Config } from 'src/types';
// Generate types
export async function buildTypes(config: Config) {
  return new Promise((resolve, reject) => {
    const typeGeneration = spawn('pnpm', [
      'exec',
      'fuels',
      'typegen',
      '-i',
      config.types.artifacts,
      '-o',
      config.types.output,
    ]);
    typeGeneration.on('exit', (code) => {
      if (!code) return resolve(code);
      typeGeneration.kill();
      reject();
    });
  });
}

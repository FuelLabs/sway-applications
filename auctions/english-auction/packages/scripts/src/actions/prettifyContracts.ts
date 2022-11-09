import { spawn } from 'child_process';
import type { Config } from 'src/types';

export async function prettifyContracts(config: Config) {
  return new Promise((resolve, reject) => {
    const prettifyProcess = spawn(
      'node_modules/.bin/prettier',
      ['--write', config.types.output.replace(/ˆ\.\//, '')],
      {
        stdio: 'inherit',
      }
    );
    prettifyProcess.on('exit', (code) => {
      if (!code) return resolve(code);
      prettifyProcess.kill();
      reject();
    });
  });
}

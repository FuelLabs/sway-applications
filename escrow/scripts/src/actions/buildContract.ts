import { spawn } from 'child_process';
import { log } from 'src/log';

// Build contracts using forc
// We assume forc is installed locally
// if forc is not installed it will throw an error
export async function buildContract(path: string) {
  log('Build', path);
  return new Promise((resolve, reject) => {
    const forcBuild = spawn('forc', ['build', '-p', path], { stdio: 'inherit' });
    forcBuild.on('exit', (code) => {
      if (!code) return resolve(code);
      forcBuild.kill();
      reject();
    });
  });
}

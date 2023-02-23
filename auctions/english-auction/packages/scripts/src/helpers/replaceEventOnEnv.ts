import { readFile, writeFile, access, copyFile } from 'fs/promises';
import { log } from 'src/log';
import type { Event } from 'src/types';
import { Commands } from 'src/types';

// TODO: This file should be placed inside the
// english-auction.config.ts but for now as the app
// uses es5 and esbuild didn't support we have
// add it here as a helper function

/**
 * Use event output data to replace
 * on the provide path env the new
 * contract ids.
 *
 * It uses the name inform on the config.contracts.name
 * as a key to the new value. If it didn't found the key
 * on the provide path nothing happens
 */
export async function replaceEventOnEnv(path: string, event: Event) {
  if (event.type === Commands.deploy || event.type === Commands.run) {
    log(`Reading file from ${path}`);
    try {
      await access(path);
    } catch (e: unknown) {
      // If the env file does not exist yet
      // Create it by copying the example env
      if (path.slice(-4) === 'test') {
        await copyFile('./packages/app/.env.example.test', path);
      } else {
        await copyFile('./packages/app/.env.example', path);
      }
    }
    const fileEnv = (await readFile(path)).toString();
    // Replace new ides on .env file
    const newEnvFile = event.data.reduce((file, { name, contractId }) => {
      log(`Replace env ${name} with ${contractId}`);
      // Replace key with new value
      return file.replace(new RegExp(`(${name}=).*`), `$1${contractId}`);
    }, fileEnv);
    log(`Updating ${path} with new contract ids`);
    await writeFile(path, newEnvFile);
    log(`${path} contract updates!`);
  }
}

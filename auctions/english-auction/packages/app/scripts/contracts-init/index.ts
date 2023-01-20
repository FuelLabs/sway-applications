import '../../load.envs';
import './loadDocketEnv';
import { initializeNFT } from './initializeNFT';

async function main() {
  await initializeNFT();
}

main();

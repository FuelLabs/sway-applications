import type { Config } from '../config';
import { runTypeChain, glob } from 'typechain';

// Generate types using typechain
// and typechain-target-fuels modules
export async function buildTypes(config: Config) {
  const cwd = process.cwd();
  // find all files matching the glob
  const allFiles = glob(cwd, [config.types.artifacts]);
  await runTypeChain({
    cwd,
    filesToProcess: allFiles,
    allFiles,
    outDir: config.types.output,
    target: 'fuels',
  });
  console.log("Types have been generated..")
}

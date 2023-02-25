export function getBinaryName(contractPath: string) {
  const fileName = contractPath.split('/').slice(-1);
  return `/out/debug/${fileName}.bin`;
}

export function getABIName(contractPath: string) {
  const fileName = contractPath.split('/').slice(-1);
  return `/out/debug/${fileName}-abi.json`;
}

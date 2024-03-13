import { createConfig } from 'fuels';

export default createConfig({
  workspace: '../project',
  output: './src/contract-types',
  useBuiltinForc: false,
  useBuiltinFuelCore: true,
  autoStartFuelCore: true,
  chainConfig: "./chainConfig.json",
});
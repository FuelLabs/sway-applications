import { createConfig } from 'fuels';

export default createConfig({
  workspace: '../tictactoe-contracts',
  output: './src/contract-types',
  useBuiltinForc: false,
  useBuiltinFuelCore: true,
  autoStartFuelCore: true,
  chainConfig: "./chainConfig.json",
  privateKey: "0xf65d6448a273b531ee942c133bb91a6f904c7d7f3104cdaf6b9f7f50d3518871",
});

import { createConfig } from 'fuels';

export default createConfig({
  workspace: '../tictactoe-contracts',
  output: './src/contract-types',
  useBuiltinForc: false,
  useBuiltinFuelCore: true,
  autoStartFuelCore: true,
  chainConfig: "./chainConfig.json",
  //privateKey: "0x80acb3fa5b95638671fe39747571ccd82f971da8bd26545542edb81c53848552",
});

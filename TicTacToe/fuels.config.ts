import { createConfig } from 'fuels';

const isProd = process.env.NODE_ENV === 'production';

export default createConfig({
  workspace: './',
  output: isProd ? './production-contract' : './src/contract-types',
  useBuiltinForc: false,
  useBuiltinFuelCore: true,
  autoStartFuelCore: true,
  chainConfig: './chainConfig.json',
  providerUrl: isProd
    ? 'https://beta-5.fuel.network/graphql'
    : 'http://127.0.0.1:4000/graphql',
});

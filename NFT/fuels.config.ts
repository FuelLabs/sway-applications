import { createConfig } from 'fuels';
import dotenv from 'dotenv';
import { NODE_URL } from '@/lib';

dotenv.config({
  path: ['.env.local', '.env'],
});

const fuelCorePort = +(process.env.NEXT_PUBLIC_FUEL_NODE_PORT as string) || 4000;

const isProd = process.env.NODE_ENV === 'production';

export default createConfig({
  workspace: './',
  output: isProd ? './production-contract' : './src/contract-types',
  fuelCorePort,
  providerUrl: NODE_URL,
});

import baseConfig from '@swayswap/config/tsup';
import { defineConfig } from 'tsup';

export default defineConfig((options) => ({
  ...baseConfig(options),
  external: ['react'],
  entry: ['src/index.ts'],
  treeshake: true,
}));

import { defineConfig } from 'tsup';

export default defineConfig((options) => [
  {
    entry: ['src/bin/index.ts', 'src/index.ts'],
    clean: true,
    dts: {
      entry: './src/index.ts',
    },
    format: ['cjs', 'esm'],
    minify: !options.watch,
  },
]);

import react from '@vitejs/plugin-react';
import jotaiDebugLabel from 'jotai/babel/plugin-debug-label';
import jotaiReactRefresh from 'jotai/babel/plugin-react-refresh';
import './load.envs.ts';
import { defineConfig } from 'vite';
import tsconfigPaths from 'vite-tsconfig-paths';

const WHITELIST = ['NODE_ENV', 'PUBLIC_URL'];
const ENV_VARS = Object.entries(process.env).filter(([key]) =>
  WHITELIST.some((k) => k === key || key.match(/^VITE_/))
);

// https://vitejs.dev/config/
export default defineConfig({
  base: process.env.PUBLIC_URL || '/',
  build: {
    target: ['es2020'],
    outDir: process.env.BUILD_PATH || 'dist',
  },
  plugins: [
    react({
      babel: { plugins: [jotaiDebugLabel, jotaiReactRefresh] },
    }),
    tsconfigPaths(),
  ],
  server: {
    port: process.env.NODE_ENV === 'test' ? 3001 : 3000,
  },
  define: {
    'process.env': Object.fromEntries(ENV_VARS),
  },
  ...(Boolean(process.env.CI) && { logLevel: 'silent' }),
  /**
   * Need because of this issue:
   * https://github.com/vitejs/vite/issues/8644#issuecomment-1159308803
   */
  esbuild: {
    logOverride: { 'this-is-undefined-in-esm': 'silent' },
  },
});

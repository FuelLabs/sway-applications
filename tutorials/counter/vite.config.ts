import react from '@vitejs/plugin-react';
import {resolve} from 'path';
import {config} from 'dotenv';
import {defineConfig} from 'vite';

config();

// https://vitejs.dev/config/
export default defineConfig({
  build: {
    target: ['es2020'],
  },
  plugins: [react()],
  define: {
    'process.env': process.env,
  },
  resolve: {
    alias: {
      '~': resolve(__dirname, './src'),
    },
  },
});

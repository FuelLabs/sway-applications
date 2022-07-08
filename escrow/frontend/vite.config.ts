import react from '@vitejs/plugin-react';
import { resolve } from 'path';
import { defineConfig } from 'vite';
import path from "path-browserify";

// https://vitejs.dev/config/
export default defineConfig({
  base: '/',
  build: {
    target: ['es2020'],
  },
  plugins: [react()],
  define: {
    'process.env': process.env,
  },
  resolve: {
    alias: {
      '@': resolve(__dirname, './src'),
      path: "path-browserify",
    },
  },
});

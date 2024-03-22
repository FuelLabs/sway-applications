import react from '@vitejs/plugin-react';
import { defineConfig } from 'vite';

// https://vitejs.dev/config/
export default defineConfig({
  base: process.env.TICTACTOE_BASE_URL,
  build: {
    outDir: process.env.TICTACTOE_DIST,
  },
  plugins: [react()],
});

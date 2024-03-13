import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vitejs.dev/config/
export default defineConfig({
  base: process.env.TICTACTOE_BASE_URL,
  build: {
    outDir: process.env.TICTACTOE_DIST,
  },
  plugins: [
    react(),
  ],
})

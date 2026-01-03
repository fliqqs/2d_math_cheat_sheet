import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react()],
  base: '/2d-math-cheat-sheet/',
  publicDir: '../public',
  server: {
    port: 3000,
    // Remove restrictive CORS headers for development
    // These headers were blocking external scripts like miniquad
  }
})

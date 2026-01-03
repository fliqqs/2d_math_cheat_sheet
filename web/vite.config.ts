import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react()],
  base: '/2d_math_cheat_sheet/',
  publicDir: '../public',
  server: {
    port: 3000,
    // Remove restrictive CORS headers for development
    // These headers were blocking external scripts like miniquad
  }
})

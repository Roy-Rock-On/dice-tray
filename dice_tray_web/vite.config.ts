import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import wasm from 'vite-plugin-wasm'
// @ts-expect-error - library package.json exports types incorrectly
import topLevelAwait from 'vite-plugin-top-level-await'

// https://vite.dev/config/
export default defineConfig({
  base: '/dice/',  // Set base path for subdirectory hosting
  plugins: [react(), wasm(), topLevelAwait()],
  build: {
    target: ['es2022', 'chrome108', 'firefox109', 'safari15'],
    rollupOptions: {
      output: {
        manualChunks: undefined
      }
    }
  },
  esbuild: {
    target: 'es2022'
  },
  optimizeDeps: {
    rolldownOptions:{
      transform: {
        target: 'es2022'
      }
    }
  }
})

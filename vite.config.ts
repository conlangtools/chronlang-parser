import { resolve } from 'path'
import { defineConfig } from 'vite'

export default defineConfig({
  build: {
    lib: {
      entry: resolve(__dirname, './src/mod.ts'),
      name: 'chronlang-parser',
      fileName: 'chronlang-parser',
    },
  },
})
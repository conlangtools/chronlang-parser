import { resolve } from 'path'
import { defineConfig } from 'vite'

export default defineConfig({
  build: {
    lib: {
      entry: resolve(__dirname, 'mod.ts'),
      name: 'chronlang-parser',
      fileName: 'chronlang-parser',
    },
  },
})
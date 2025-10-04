import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import path from 'node:path';

// Vite configuration is colocated with the example so enterprise teams can simply execute
// `npm run dev` without additional flags.  The configuration intentionally keeps aliasing explicit
// so that Jest/Playwright and Vite share the same module resolution semantics.
export default defineConfig({
  root: __dirname,
  plugins: [react()],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, 'src'),
      '@pkg': path.resolve(__dirname, 'pkg'),
    },
  },
  server: {
    port: 4173,
    host: '0.0.0.0',
  },
  build: {
    outDir: path.resolve(__dirname, 'dist'),
    target: 'esnext',
    rollupOptions: {
      input: path.resolve(__dirname, 'index.html'),
    },
  },
});

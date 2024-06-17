import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react';

export default defineConfig({
    plugins: [react()],
    build: {
        // generate manifest.json in outDir
        manifest: true,
        rollupOptions: {
        // overwrite default .html entry
        input: 'src/main.ts'
        }
    }
})
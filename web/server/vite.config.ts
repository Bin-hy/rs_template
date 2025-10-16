import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
// import { fileURLToPath, URL } from 'node:url'
import { resolve } from 'node:path'

export const ProjectRoot = resolve(import.meta.dirname, '../..');
// directory name of the current module (web/server)
// const configDir = import.meta.dirname;
// https://vite.dev/config/
export default defineConfig({
    plugins: [
        vue(),
    ],
    base: './',
    server: {
        proxy: {

        }
    },
    build: {
        outDir: resolve(ProjectRoot, "assets/server",)
    },
    resolve: {
        alias: {
            '@': resolve("./src") //fileURLToPath(new URL('./src', import.meta.url))
        }
    }
})

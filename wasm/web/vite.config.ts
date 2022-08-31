import { defineConfig } from 'vite'
import wasmPack from 'vite-plugin-wasm-pack'

export default defineConfig({
	plugins: [wasmPack("../rust")],
	server: { port: 80 }
})

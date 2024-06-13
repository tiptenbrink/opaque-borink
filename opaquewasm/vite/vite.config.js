import wasm from "vite-plugin-wasm-esm";

/** @type {import('vite').UserConfig} */
export default {
    plugins: [
        wasm(["@tiptenbrink/opaquewasm", "opaquewasm"]),
    ],
    build: {
		target: ["chrome89", "safari15", "firefox89"],
	},
	esbuild: {
		target: ["chrome89", "safari15", "firefox89"],
	},
}
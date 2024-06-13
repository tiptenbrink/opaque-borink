This uses `vite-plugin-wasm-esm` as opposed to `vite-plugin-wasm`. I couldn't get the latter to work anymore.

It's important you provide the correct package name to the `wasm([...])` invocation in `vite.config.js`. 
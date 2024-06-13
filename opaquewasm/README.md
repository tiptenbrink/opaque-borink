This library contains extremely minimal WebAssembly JavaScript bindings of a standard configuration of [opaque-ke](https://github.com/novifinancial/opaque-ke). It exposes 4 functions, which only accept and return base64url-encoded strings.

This library is a counterpart to [opaquepy](https://github.com/tiptenbrink/tree/main/opaquepy), built upon the configuration defined in [opaque-borink](https://github.com/tiptenbrink/opaque-borink/tree/main/opaque-borink).

### Building

`wasm-pack build --target bundler` to build.

This generates a `pkg`, which you can `npm publish`. Change the name in `package.json` to `@tiptenbrink/opaquewasm`.

To test it out in a browser:

- Run `npm install` to install an environment
- Run `npm start` to start a server that activates a page where you can run the code in `index.js`

After rebuilding you might have to run `npm install ./pkg` to install it again.

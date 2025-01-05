This library contains extremely minimal Python bindings of an opinionated standard configuration of [opaque-ke](https://github.com/novifinancial/opaque-ke). It exposes 4 functions, which only accept and return base64url-encoded strings.

This library is a counterpart to [@tiptenbrink/opaquewasm](https://github.com/tiptenbrink/opaque-borink/tree/main/opaquewasm), built upon the configuration defined in [opaque-borink](https://github.com/tiptenbrink/opaque-borink/tree/main/opaque-borink).

## Development

First, install uv. Since we aim for compatibility with Python 3.9+, it's recommended to install Python 3.9. 

Do `uv sync --no-install-project --locked` to install the dependencies, not including the project. 

Next, install `maturin` and build the Rust project using `maturin develop --uv` (it's recommended to install `maturin` globally using `cargo binstall maturin` or `pipx install maturin`). 

Run the tests using `uv run pytest`.

Note that type information is not available for the Rust functions, you will have to look at the Rust source code. Maturin builds a package structures as follows:
- root `opaquepy` package
    - `_internal`: this includes `create_setup_py`, etc.
    - `lib`: this is the Python source code in `python/opaquepy/lib.py`
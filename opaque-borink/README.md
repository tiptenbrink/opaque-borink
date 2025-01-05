A simple instantiation/configuration of the [opaque-ke](https://github.com/novifinancial/opaque-ke) OPAQUE implementation.

OPAQUE ([see the Internet-Draft](https://datatracker.ietf.org/doc/draft-irtf-cfrg-opaque/)) is an upcoming standard for password authentication. It is more secure than a traditional simple salt and password hash scheme.

It enables a workflow where the server never learns the user password, yet the server does not need to provide the salt to anyone who asks, providing security against pre-computation attacks.

It uses a basic CipherSuite configured as follows:

* Ristretto255 as the OprfCs, i.e. the [`voprf`](https://github.com/facebook/voprf) CipherSuite, which means using [`curve25519_dalek`](https://github.com/dalek-cryptography/curve25519-dalek) as the Ristretto implementation and [`sha2`](https://github.com/RustCrypto/hashes)'s Sha512 as the hash implementation
* Ristretto255 as the key exchange group (KeGroup), again implemented using [`curve25519_dalek`](https://github.com/dalek-cryptography/curve25519-dalek)
* opaque-ke's own TripleDH as KeyExchange
* [argon2](https://github.com/RustCrypto/password-hashes/tree/master/argon2) default Argon2 as the key stretching function (Ksf)

It exposes four functions on both the server and client: login finish/start and register finish/start; as well as a key generation function. It also exposes a number of constants for the sizes of various structs.

It is optimized for the usecase in which the server is stateless, but the client stateful.

`opaque-borink` is useful as a stand-alone library, but also serves as the core library for `opaquepy` and `@tiptenbrink/opaquewasm`, bindings for Python and WebAssembly, respectively.

### Changelog

#### v0.4.1

- Adds support for building on newer Rust, backports Argon v0.4.1 support to the fixed version, should still work as before

#### v0.5.0

- BREAKING: Update OPAQUE to v3, which is a new protocol version and thus previous registrations are no longer valid
- BREAKING: Update Argon2 to v0.5, which uses different default parameters, also causing registrations to become invalid
- It is therefore recommended to reregister and reauthenticate all new users

#### v0.6.0

- BREAKING: The entire API has been reengineered to be somewhat more flexible and no longer only have base64url input/output 
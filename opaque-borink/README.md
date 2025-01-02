A simple configuration of the [opaque-ke](https://github.com/novifinancial/opaque-ke) OPAQUE implementation, using a base64url-encoded format to serialize and deserialize the Rust structs. 

OPAQUE ([see the Internet-Draft](https://datatracker.ietf.org/doc/draft-irtf-cfrg-opaque/)) is an upcoming standard for password authentication. It is more secure than a traditional simple salt and password hash scheme.

It enables a workflow where the server never learns the user password, yet the server does not need to provide the salt to anyone who asks, providing security against pre-computation attacks.

It uses a basic CipherSuite configured as follows:
* [curve25519_dalek](https://github.com/dalek-cryptography/curve25519-dalek) Ristretto group as Group
* opaque-ke's own TripleDH as KeyExchange
* [sha2](https://github.com/RustCrypto/hashes/tree/master/sha2) Sha512 as Hash
* [argon2](https://github.com/RustCrypto/password-hashes/tree/master/argon2) default Argon2 as SlowHash

It exposes four functions on both the server and client: login finish/start and register finish/start; as well as a key generation function.

`opaque-borink` is useful as a stand-alone library, but also serves as the core library for `opaquepy` and `@tiptenbrink/opaquewasm`, bindings for Python and WebAssembly, respectively.

### Changelog

#### v0.4.1

- Adds support for building on newer Rust, backports Argon v0.4.1 support to the fixed version, should still work as before

#### v0.5.0

- BREAKING: Update OPAQUE to v3, which is a new protocol version and thus previous registrations are no longer valid
- BREAKING: Update Argon2 to v0.5, which uses different default parameters, also causing registrations to become invalid
- It is therefore recommended to reregister and reauthenticate all new users
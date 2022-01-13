A simple configuration of the [opaque-ke](https://github.com/novifinancial/opaque-ke) OPAQUE implementation, using a base64url-encoded format to serialize and deserialize the Rust structs. 

OPAQUE ([see the Internet-Draft](https://datatracker.ietf.org/doc/html/draft-krawczyk-cfrg-opaque-06)) is an upcoming standard for password authentication. It is more secure than a traditional simple salt and password hash scheme.

It uses a basic CipherSuite configured as follows:
* [curve25519_dalek](https://github.com/dalek-cryptography/curve25519-dalek) Ristretto group as Group
* opaque-ke's own TripleDH as KeyExchange
* [sha2](https://github.com/RustCrypto/hashes/tree/master/sha2) Sha512 as Hash
* [argon2](https://github.com/RustCrypto/password-hashes/tree/master/argon2) default Argon2 as SlowHash

It exposes four functions on the server and client, login finish/start and register finish/start as well as a key generation function.

`opaquebind` serves as the core library for `opaquepy` and `@tiptenbrink/opaquewasm`, bindings for Python and WebAssembly, respectively.


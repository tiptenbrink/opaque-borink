pub mod server;
pub mod client;

use argon2::Argon2;
use curve25519_dalek;
use curve25519_dalek::ristretto::RistrettoPoint;
use opaque_ke::ciphersuite::CipherSuite;
use sha2;
use base64::DecodeError;
use digest::Digest;
use digest::generic_array::GenericArray;
use digest::generic_array::typenum::Unsigned;
use opaque_ke::errors::{InternalPakeError};
pub use opaque_ke::errors::{PakeError, ProtocolError};
use opaque_ke::hash::Hash;
use opaque_ke::keypair::KeyPair;
use opaque_ke::slow_hash::SlowHash;
use rand::rngs::OsRng;

pub struct Cipher;
impl CipherSuite for Cipher {
    type Group = curve25519_dalek::ristretto::RistrettoPoint;
    type KeyExchange = opaque_ke::key_exchange::tripledh::TripleDH;
    type Hash = sha2::Sha512;
    type SlowHash = ArgonWrapper;
}

pub struct ArgonWrapper(Argon2<'static>);

impl<D: Hash> SlowHash<D> for ArgonWrapper {
    fn hash(
        input: GenericArray<u8, <D as Digest>::OutputSize>,
    ) -> Result<Vec<u8>, InternalPakeError> {
        let params = Argon2::default();
        let mut output = vec![0u8; <D as Digest>::OutputSize::to_usize()];
        params
            .hash_password_into(
                &input,
                &[0; argon2::MIN_SALT_LEN],
                &mut output,
            )
            .map_err(|_| InternalPakeError::SlowHashError)?;
        Ok(output)
    }
}

pub enum Error {
    ProtocolError(ProtocolError),
    PakeError(PakeError),
    DecodeError(DecodeError)
}

impl From<ProtocolError> for Error {
    fn from(e: ProtocolError) -> Self {
        Error::ProtocolError(e)
    }
}

impl From<PakeError> for Error {
    fn from(e: PakeError) -> Self {
        Error::PakeError(e)
    }
}

impl From<DecodeError> for Error {
    fn from(e: DecodeError) -> Self {
        Error::DecodeError(e)
    }
}

pub fn generate_keys() -> (String, String) {
    let keypair = generate_keys_compute();
    let private_key = keypair.private().to_vec();
    let public_key = keypair.public().to_vec();

    let private_encoded = base64::encode_config(private_key, base64::URL_SAFE_NO_PAD);
    let public_encoded = base64::encode_config(public_key, base64::URL_SAFE_NO_PAD);

    (private_encoded, public_encoded)
}

fn generate_keys_compute() -> KeyPair<RistrettoPoint> {
    let mut rng = OsRng;
    Cipher::generate_random_keypair(&mut rng)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gen_test() {
        let (privt, publ) = generate_keys();
        println!("{}", privt);
        println!("{}", publ);
    }
}
pub mod client;
pub mod server;

use base64::DecodeError;
use base64::{engine::general_purpose as b64, Engine as _};
use opaque_ke::ciphersuite::CipherSuite;
pub use opaque_ke::errors::ProtocolError;
use opaque_ke::ServerSetup;
use rand::rngs::OsRng;
use std::fmt::{Debug, Display, Formatter};

pub struct Cipher;
impl CipherSuite for Cipher {
    type OprfCs = opaque_ke::Ristretto255;
    type KeGroup = opaque_ke::Ristretto255;
    type KeyExchange = opaque_ke::key_exchange::tripledh::TripleDh;
    type Ksf = argon2::Argon2<'static>;
}

#[derive(Debug)]
pub enum Error {
    ProtocolError(ProtocolError),
    DecodeError(DecodeError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}

impl std::error::Error for Error {}

impl From<ProtocolError> for Error {
    fn from(e: ProtocolError) -> Self {
        Error::ProtocolError(e)
    }
}

impl From<DecodeError> for Error {
    fn from(e: DecodeError) -> Self {
        Error::DecodeError(e)
    }
}

pub fn create_setup() -> String {
    let mut rng = OsRng;
    let server_setup = ServerSetup::<Cipher>::new(&mut rng);
    let server_setup_serialized = ServerSetup::serialize(&server_setup);

    b64::URL_SAFE_NO_PAD.encode(server_setup_serialized)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gen_test() {
        let serv_setp = create_setup();
        println!("{}", serv_setp);
    }
}

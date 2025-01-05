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

mod api {

    use generic_array::{typenum::Unsigned, ArrayLength, GenericArray};
    use opaque_ke::{key_exchange::tripledh::Ke2State, CipherSuite, ClientLogin, ClientLoginFinishParameters, CredentialFinalization, CredentialRequest, CredentialResponse, CredentialResponseLen, ServerLogin, ServerLoginStartParameters, ServerRegistration};
    use rand::rngs::OsRng;
    use typenum::{Const, ToUInt};

    use super::{Cipher, Error, IntoArray};


    pub struct PasswordFile(ServerRegistration::<Cipher>);

    pub struct ServerSetup {
        setup: opaque_ke::ServerSetup<Cipher>,
        rng: rand::rngs::ThreadRng
    }

    pub fn server_login_start(setup: &mut ServerSetup, password_file: &PasswordFile, user_id: &str, login_start_request: &[u8]) -> Result<ServerLoginStartResult, Error> {
        let login_start_request = CredentialRequest::<Cipher>::deserialize(login_start_request)?;

        let result = ServerLogin::<Cipher>::start(
            &mut setup.rng,
            &setup.setup,
            Some(password_file.0.clone()),
            login_start_request,
            user_id.as_bytes(),
            ServerLoginStartParameters::default(),
        )?;

        Ok(ServerLoginStartResult {
            response: result.message.serialize().into_array(),
            state: result.state.serialize().into_array()
        })
    }

    pub struct ClientSetup {
        rng: rand::rngs::ThreadRng,
        state: Option<ClientLogin<Cipher>>
    }

    pub fn client_login_start(client_setup: &mut ClientSetup, password: &[u8]) -> Result<ClientLoginStartResult, Error> {
        let result = ClientLogin::<Cipher>::start(&mut client_setup.rng, password)?;

        client_setup.state = Some(result.state);

        Ok(ClientLoginStartResult {
            response: result.message.serialize().into_array(),
        })
    }

    pub fn client_login_finish(client_setup: &mut ClientSetup, password: &[u8], server_message: &[u8]) -> Result<ClientLoginFinishResult, Error> {
        if client_setup.state.is_none() {
            panic!("Client state not initialized!")
        }
        let client_setup = client_setup.state.take().unwrap();
        let server_message = CredentialResponse::<Cipher>::deserialize(server_message)?;
        let result = client_setup.finish(password, server_message, ClientLoginFinishParameters::default())?;

        Ok(ClientLoginFinishResult {
            response: result.message.serialize().into_array(),
            shared_secret: result.session_key.into_array()
        })
    }

    pub fn server_login_finish(server_state: &[u8], login_finish_request: &[u8]) -> Result<ServerLoginFinishResult, Error> {
        
        let state = ServerLogin::<Cipher>::deserialize(server_state)?;
        let login_finish_request = CredentialFinalization::<Cipher>::deserialize(login_finish_request)?;
        let result = state.finish(login_finish_request)?;
        Ok(ServerLoginFinishResult {
            shared_secret: result.session_key.into_array()
        })
    }

    pub const LOGIN_SERVER_MESSAGE_LEN: usize = 320;
    pub const LOGIN_SERVER_STATE_LEN: usize = 192;

    pub const LOGIN_CLIENT_MESSAGE_LEN: usize = 96;
    pub const LOGIN_FINISH_MESSAGE_LEN: usize = 64;

    pub const SHARED_SECRET_LEN: usize = 64;

    pub struct ServerLoginStartResult {
        response: [u8; LOGIN_SERVER_MESSAGE_LEN],
        state: [u8; LOGIN_SERVER_STATE_LEN]
    }

    pub struct ClientLoginStartResult {
        response: [u8; LOGIN_CLIENT_MESSAGE_LEN],
    }

    pub struct ClientLoginFinishResult {
        response: [u8; LOGIN_FINISH_MESSAGE_LEN],
        shared_secret: [u8; SHARED_SECRET_LEN]
    }

    pub struct ServerLoginFinishResult {
        shared_secret: [u8; SHARED_SECRET_LEN]
    }

    #[cfg(test)]
    mod test {
        use opaque_ke::{ClientLogin, ClientRegistration};

        use super::*;

        #[test]
        fn check_lengths() {
            let mut rng = OsRng;
            let server_setup = opaque_ke::ServerSetup::<Cipher>::new(&mut rng);
            //let server_setup_serialized = server_setup.serialize();

            let a = ClientRegistration::start(&mut rng, "my_pass".as_bytes()).unwrap();

            let b = ServerRegistration::start(&server_setup, a.message, "my_user".as_bytes()).unwrap();

            let c = a.state.finish(&mut rng, "my_pass".as_bytes(), b.message, opaque_ke::ClientRegistrationFinishParameters::default()).unwrap();

            let pw_file = ServerRegistration::finish(c.message);

            let d = ClientLogin::<Cipher>::start(&mut rng, "my_pass".as_bytes()).unwrap();

            assert_eq!(LOGIN_CLIENT_MESSAGE_LEN, d.message.serialize().len());
            assert_eq!(d.message.serialize().as_slice(), d.message.serialize().into_array::<LOGIN_CLIENT_MESSAGE_LEN>().as_slice());

            let e = ServerLogin::<Cipher>::start(&mut rng, &server_setup, Some(pw_file.clone()), d.message, "my_user".as_bytes(), opaque_ke::ServerLoginStartParameters::default()).unwrap();

            let f: [u8; LOGIN_SERVER_MESSAGE_LEN] = e.message.serialize().into_array();
            let g: [u8; LOGIN_SERVER_MESSAGE_LEN] = unsafe { core::mem::transmute(e.message.serialize()) };
            
            assert_eq!(&f, e.message.serialize().as_slice());
            assert_eq!(&g, &f);
            assert_eq!(LOGIN_SERVER_MESSAGE_LEN, e.message.serialize().len());
            assert_eq!(LOGIN_SERVER_STATE_LEN, e.state.serialize().len());

            let h = d.state.finish("my_pass".as_bytes(), e.message, ClientLoginFinishParameters::default()).unwrap();

            assert_eq!(LOGIN_FINISH_MESSAGE_LEN, h.message.serialize().len());
            assert_eq!(h.message.serialize().as_slice(), h.message.serialize().into_array::<LOGIN_FINISH_MESSAGE_LEN>());
            assert_eq!(64, h.session_key.len());
        }

    }
}


mod gen_arr_backport {
    use generic_array::{ArrayLength, GenericArray};
    use typenum::{Const, ToUInt};

    const unsafe fn const_transmute<A, B>(a: A) -> B {
        if std::mem::size_of::<A>() != std::mem::size_of::<B>() {
            panic!("Size mismatch for generic_array::const_transmute");
        }
    
        #[repr(C)]
        union Union<A, B> {
            a: std::mem::ManuallyDrop<A>,
            b: std::mem::ManuallyDrop<B>,
        }
    
        let a = std::mem::ManuallyDrop::new(a);
        std::mem::ManuallyDrop::into_inner(Union { a }.b)
    }

    pub(crate) trait IntoArray<N: ArrayLength<u8>> {
        fn into_array<const U: usize>(self) -> [u8; U]
    where
        Const<U>: IntoArrayLength<ArrayLength = N>, Self: Sized;
    }

    impl<N: ArrayLength<u8>> IntoArray<N> for GenericArray<u8, N> {
        fn into_array<const U: usize>(self) -> [u8; U]
            where
        Const<U>: IntoArrayLength<ArrayLength = N>, Self: Sized {
            unsafe { const_transmute(self) }
        }
    }



    pub(crate) trait IntoArrayLength {
        /// The associated `ArrayLength`
        type ArrayLength: ArrayLength<u8>;
    }
    
    impl<const N: usize> IntoArrayLength for Const<N>
    where
        Const<N>: ToUInt,
        typenum::U<N>: ArrayLength<u8>,
    {
        type ArrayLength = typenum::U<N>;
    }
}

pub(crate) use gen_arr_backport::IntoArray;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gen_test() {
        let serv_setp = create_setup();
        println!("{}", serv_setp);
    }
}

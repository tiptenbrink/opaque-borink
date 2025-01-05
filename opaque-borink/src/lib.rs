pub mod client;
pub mod encoded;
pub mod server;

use base64::DecodeError;
use opaque_ke::ciphersuite::CipherSuite;
pub use opaque_ke::errors::ProtocolError;
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

pub(crate) mod opaque_impl {
    use opaque_ke::{
        ClientLogin, ClientLoginFinishParameters, ClientRegistration,
        ClientRegistrationFinishParameters, CredentialFinalization, CredentialRequest,
        CredentialResponse, RegistrationRequest, RegistrationResponse, RegistrationUpload,
        ServerLogin, ServerLoginStartParameters, ServerRegistration,
    };
    use rand::{
        rngs::{OsRng, ThreadRng},
        thread_rng,
    };

    use super::{Cipher, Error, IntoArray};

    #[repr(transparent)]
    pub struct PasswordFile(ServerRegistration<Cipher>);

    impl PasswordFile {
        pub fn serialize(&self) -> [u8; PASSWORD_FILE_SERIALIZED_LEN] {
            self.0.serialize().into_array()
        }

        pub fn deserialize(bytes: &[u8]) -> Result<Self, Error> {
            let registration = opaque_ke::ServerRegistration::<Cipher>::deserialize(bytes)?;

            Ok(Self(registration))
        }
    }

    pub struct ServerSetup(opaque_ke::ServerSetup<Cipher>);

    impl ServerSetup {
        pub fn create() -> Self {
            let mut rng = OsRng;
            let server_setup = opaque_ke::ServerSetup::<Cipher>::new(&mut rng);

            Self(server_setup)
        }

        pub fn view(&self) -> ServerSetupView {
            ServerSetupView {
                setup: &self.0,
                rng: thread_rng(),
            }
        }

        pub fn serialize(&self) -> [u8; SERVER_SETUP_LEN] {
            self.0.serialize().into_array()
        }

        pub fn deserialize(bytes: &[u8]) -> Result<Self, Error> {
            let setup = opaque_ke::ServerSetup::<Cipher>::deserialize(bytes)?;

            Ok(Self(setup))
        }
    }

    #[derive(Clone)]
    pub struct ServerSetupView<'a> {
        setup: &'a opaque_ke::ServerSetup<Cipher>,
        rng: rand::rngs::ThreadRng,
    }

    pub fn server_login_start(
        setup: &mut ServerSetupView,
        password_file: &PasswordFile,
        login_start_request: &[u8],
        user_id: &str,
    ) -> Result<ServerLoginStartResult, Error> {
        let login_start_request = CredentialRequest::<Cipher>::deserialize(login_start_request)?;

        let result = ServerLogin::<Cipher>::start(
            &mut setup.rng,
            setup.setup,
            Some(password_file.0.clone()),
            login_start_request,
            user_id.as_bytes(),
            ServerLoginStartParameters::default(),
        )?;

        Ok(ServerLoginStartResult {
            response: result.message.serialize().into_array(),
            state: result.state.serialize().into_array(),
        })
    }

    pub struct ClientStateLogin {
        rng: rand::rngs::ThreadRng,
        state: Option<ClientLogin<Cipher>>,
    }

    pub fn client_login_start(
        client_state: &mut ClientStateLogin,
        password: &[u8],
    ) -> Result<ClientLoginStartResult, Error> {
        let result = ClientLogin::<Cipher>::start(&mut client_state.rng, password)?;

        client_state.state = Some(result.state);

        Ok(ClientLoginStartResult {
            response: result.message.serialize().into_array(),
        })
    }

    pub fn client_login_finish(
        client_state: &mut ClientStateLogin,
        password: &[u8],
        server_message: &[u8],
    ) -> Result<ClientLoginFinishResult, Error> {
        if client_state.state.is_none() {
            panic!("Client state not initialized! Run `client_login_start` first!")
        }
        let client_state = client_state.state.take().unwrap();
        let server_message = CredentialResponse::<Cipher>::deserialize(server_message)?;
        let result = client_state.finish(
            password,
            server_message,
            ClientLoginFinishParameters::default(),
        )?;

        Ok(ClientLoginFinishResult {
            response: result.message.serialize().into_array(),
            shared_secret: result.session_key.into_array(),
        })
    }

    pub fn server_login_finish(
        login_finish_request: &[u8],
        server_state: &[u8],
    ) -> Result<ServerLoginFinishResult, Error> {
        let state = ServerLogin::<Cipher>::deserialize(server_state)?;
        let login_finish_request =
            CredentialFinalization::<Cipher>::deserialize(login_finish_request)?;
        let result = state.finish(login_finish_request)?;
        Ok(ServerLoginFinishResult {
            shared_secret: result.session_key.into_array(),
        })
    }

    pub struct ClientStateRegistration {
        rng: ThreadRng,
        state: Option<ClientRegistration<Cipher>>,
    }

    impl ClientStateRegistration {
        pub fn setup() -> Self {
            Self {
                rng: thread_rng(),
                state: None,
            }
        }

        pub fn deserialize(bytes: &[u8]) -> Result<Self, Error> {
            let login = ClientRegistration::<Cipher>::deserialize(bytes)?;

            Ok(Self {
                rng: thread_rng(),
                state: Some(login),
            })
        }

        pub fn serialize(&self) -> [u8; REGISTER_CLIENT_STATE_LEN] {
            self.state.as_ref().expect("Can only serialize after first step is completed!").serialize().into_array()
        }
    }

    impl ClientStateLogin {
        pub fn setup() -> Self {
            Self {
                rng: thread_rng(),
                state: None,
            }
        }

        pub fn deserialize(bytes: &[u8]) -> Result<Self, Error> {
            let login = ClientLogin::<Cipher>::deserialize(bytes)?;

            Ok(Self {
                rng: thread_rng(),
                state: Some(login),
            })
        }

        pub fn serialize(&self) -> [u8; LOGIN_CLIENT_STATE_LEN] {
            self.state.as_ref().expect("Can only serialize after first step is completed!").serialize().into_array()
        }
    }

    pub fn client_register_start(
        client_state: &mut ClientStateRegistration,
        password: &[u8],
    ) -> Result<ClientRegistrationStartResult, Error> {
        let result = ClientRegistration::start(&mut client_state.rng, password)?;

        client_state.state = Some(result.state);

        Ok(ClientRegistrationStartResult {
            response: result.message.serialize().into_array(),
        })
    }

    pub fn server_register_start(
        server_setup: &mut ServerSetupView,
        register_start_request: &[u8],
        user_id: &[u8],
    ) -> Result<ServerRegistrationStartResult, Error> {
        let register_start_request =
            RegistrationRequest::<Cipher>::deserialize(register_start_request)?;

        let result = ServerRegistration::<Cipher>::start(
            server_setup.setup,
            register_start_request,
            user_id,
        )?;

        Ok(ServerRegistrationStartResult {
            response: result.message.serialize().into_array(),
        })
    }

    pub fn client_register_finish(
        client_state: &mut ClientStateRegistration,
        password: &[u8],
        server_message: &[u8],
    ) -> Result<ClientRegistrationFinishResult, Error> {
        if client_state.state.is_none() {
            panic!("Client state not initialized! Run `server_register_start` first!")
        }
        let state = client_state.state.take().unwrap();
        let server_message = RegistrationResponse::deserialize(server_message)?;

        let result = state.finish(
            &mut client_state.rng,
            password,
            server_message,
            ClientRegistrationFinishParameters::default(),
        )?;

        Ok(ClientRegistrationFinishResult {
            response: result.message.serialize().into_array(),
        })
    }

    pub fn server_register_finish(register_finish_request: &[u8]) -> Result<PasswordFile, Error> {
        let register_finish_request =
            RegistrationUpload::<Cipher>::deserialize(register_finish_request)?;

        let result = ServerRegistration::finish(register_finish_request);

        Ok(PasswordFile(result))
    }

    pub const SERVER_SETUP_LEN: usize = 128;

    pub const LOGIN_SERVER_MESSAGE_LEN: usize = 320;
    pub const LOGIN_SERVER_STATE_LEN: usize = 192;
    pub const LOGIN_CLIENT_STATE_LEN: usize = 192;
    pub const LOGIN_CLIENT_MESSAGE_LEN: usize = 96;
    pub const LOGIN_FINISH_MESSAGE_LEN: usize = 64;
    
    pub const REGISTER_SERVER_MESSAGE_LEN: usize = 64;
    pub const REGISTER_CLIENT_STATE_LEN: usize = 64;
    pub const REGISTER_CLIENT_MESSAGE_LEN: usize = 32;
    pub const REGISTER_FINISH_MESSAGE_LEN: usize = 192;

    pub const SHARED_SECRET_LEN: usize = 64;

    pub const PASSWORD_FILE_LEN: usize = 328;
    pub const PASSWORD_FILE_SERIALIZED_LEN: usize = 192;

    pub struct ServerLoginStartResult {
        pub response: [u8; LOGIN_SERVER_MESSAGE_LEN],
        pub state: [u8; LOGIN_SERVER_STATE_LEN],
    }

    pub struct ClientLoginStartResult {
        pub response: [u8; LOGIN_CLIENT_MESSAGE_LEN],
    }

    pub struct ClientLoginFinishResult {
        pub response: [u8; LOGIN_FINISH_MESSAGE_LEN],
        pub shared_secret: [u8; SHARED_SECRET_LEN],
    }

    pub struct ServerLoginFinishResult {
        pub shared_secret: [u8; SHARED_SECRET_LEN],
    }

    pub struct ClientRegistrationStartResult {
        pub response: [u8; REGISTER_CLIENT_MESSAGE_LEN],
    }

    pub struct ClientRegistrationFinishResult {
        pub response: [u8; REGISTER_FINISH_MESSAGE_LEN],
    }

    pub struct ServerRegistrationStartResult {
        pub response: [u8; REGISTER_SERVER_MESSAGE_LEN],
    }

    #[cfg(test)]
    mod test {
        use opaque_ke::{ClientLogin, ClientRegistration};

        use super::*;

        #[test]
        fn check_lengths() {
            let mut rng = OsRng;
            let server_setup = opaque_ke::ServerSetup::<Cipher>::new(&mut rng);

            assert_eq!(SERVER_SETUP_LEN, server_setup.serialize().len());

            let a = ClientRegistration::start(&mut rng, "my_pass".as_bytes()).unwrap();

            assert_eq!(REGISTER_CLIENT_MESSAGE_LEN, a.message.serialize().len());
            assert_eq!(
                a.message.serialize().as_slice(),
                a.message
                    .serialize()
                    .into_array::<REGISTER_CLIENT_MESSAGE_LEN>()
                    .as_slice()
            );
            assert_eq!(REGISTER_CLIENT_STATE_LEN, a.state.serialize().len());

            let b =
                ServerRegistration::start(&server_setup, a.message, "my_user".as_bytes()).unwrap();

            assert_eq!(REGISTER_SERVER_MESSAGE_LEN, b.message.serialize().len());
            assert_eq!(
                b.message.serialize().as_slice(),
                b.message
                    .serialize()
                    .into_array::<REGISTER_SERVER_MESSAGE_LEN>()
                    .as_slice()
            );

            let c = a
                .state
                .finish(
                    &mut rng,
                    "my_pass".as_bytes(),
                    b.message,
                    opaque_ke::ClientRegistrationFinishParameters::default(),
                )
                .unwrap();

            assert_eq!(REGISTER_FINISH_MESSAGE_LEN, c.message.serialize().len());
            assert_eq!(
                c.message.serialize().as_slice(),
                c.message
                    .serialize()
                    .into_array::<REGISTER_FINISH_MESSAGE_LEN>()
                    .as_slice()
            );

            let pw_file = ServerRegistration::finish(c.message);

            assert_eq!(
                size_of_val(&PasswordFile(pw_file.clone())),
                PASSWORD_FILE_LEN
            );
            assert_eq!(size_of::<PasswordFile>(), PASSWORD_FILE_LEN);

            let d = ClientLogin::<Cipher>::start(&mut rng, "my_pass".as_bytes()).unwrap();

            assert_eq!(LOGIN_CLIENT_MESSAGE_LEN, d.message.serialize().len());
            assert_eq!(
                d.message.serialize().as_slice(),
                d.message
                    .serialize()
                    .into_array::<LOGIN_CLIENT_MESSAGE_LEN>()
                    .as_slice()
            );
            assert_eq!(LOGIN_CLIENT_STATE_LEN, d.state.serialize().len());

            let e = ServerLogin::<Cipher>::start(
                &mut rng,
                &server_setup,
                Some(pw_file.clone()),
                d.message,
                "my_user".as_bytes(),
                opaque_ke::ServerLoginStartParameters::default(),
            )
            .unwrap();

            let f: [u8; LOGIN_SERVER_MESSAGE_LEN] = e.message.serialize().into_array();
            let g: [u8; LOGIN_SERVER_MESSAGE_LEN] =
                unsafe { core::mem::transmute(e.message.serialize()) };

            assert_eq!(&f, e.message.serialize().as_slice());
            assert_eq!(&g, &f);
            assert_eq!(LOGIN_SERVER_MESSAGE_LEN, e.message.serialize().len());
            assert_eq!(LOGIN_SERVER_STATE_LEN, e.state.serialize().len());

            let h = d
                .state
                .finish(
                    "my_pass".as_bytes(),
                    e.message,
                    ClientLoginFinishParameters::default(),
                )
                .unwrap();

            assert_eq!(LOGIN_FINISH_MESSAGE_LEN, h.message.serialize().len());
            assert_eq!(
                h.message.serialize().as_slice(),
                h.message
                    .serialize()
                    .into_array::<LOGIN_FINISH_MESSAGE_LEN>()
            );
            assert_eq!(64, h.session_key.len());
        }
    }
}

// The following code is taken from version 1.1.1 of https://github.com/fizyk20/generic-array under the MIT License
// It's a backport of some functions that allow conversion to normal arrays to the version of generic array used in opaque_ke (0.14.7)
mod generic_array_backport {
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

    pub trait IntoArray<N: ArrayLength<u8>> {
        fn into_array<const U: usize>(self) -> [u8; U]
        where
            Const<U>: IntoArrayLength<ArrayLength = N>,
            Self: Sized;
    }

    impl<N: ArrayLength<u8>> IntoArray<N> for GenericArray<u8, N> {
        fn into_array<const U: usize>(self) -> [u8; U]
        where
            Const<U>: IntoArrayLength<ArrayLength = N>,
            Self: Sized,
        {
            unsafe { const_transmute(self) }
        }
    }

    pub trait IntoArrayLength {
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

pub(crate) use generic_array_backport::IntoArray;

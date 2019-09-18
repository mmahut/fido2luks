use ctap::FidoError;
use std::io;

pub type Fido2LuksResult<T> = Result<T, Fido2LuksError>;

#[derive(Debug, Fail)]
pub enum Fido2LuksError {
    #[fail(display = "unable to retrieve password: {}", cause)]
    AskPassError { cause: io::Error },
    #[fail(display = "unable to read keyfile: {}", cause)]
    KeyfileError { cause: io::Error },
    #[fail(display = "authenticator error: {}", cause)]
    AuthenticatorError { cause: ctap::FidoError },
    #[fail(display = "no authenticator found, please ensure you device is plugged in")]
    NoAuthenticatorError,
    #[fail(display = "luks err")]
    LuksError { cause: cryptsetup_rs::device::Error },
    #[fail(display = "no authenticator found, please ensure you device is plugged in")]
    IoError { cause: io::Error },
    #[fail(display = "failed to parse config, please check formatting and contents")]
    ConfigurationError { cause: ConfigurationError },
    #[fail(display = "the submitted secret is not applicable to this luks device")]
    WrongSecret,
    #[fail(display = "not an utf8 string")]
    StringEncodingError { cause: FromUtf8Error },
}

#[derive(Debug)]
pub enum ConfigurationError {
    Json(serde_json::error::Error),
    Env(envy::Error),
    MissingField(String),
}

impl From<serde_json::error::Error> for Fido2LuksError {
    fn from(e: serde_json::error::Error) -> Self {
        Fido2LuksError::ConfigurationError {
            cause: ConfigurationError::Json(e),
        }
    }
}

impl From<envy::Error> for Fido2LuksError {
    fn from(e: envy::Error) -> Self {
        Fido2LuksError::ConfigurationError {
            cause: ConfigurationError::Env(e),
        }
    }
}

use std::string::FromUtf8Error;
use Fido2LuksError::*;

impl From<FidoError> for Fido2LuksError {
    fn from(e: FidoError) -> Self {
        AuthenticatorError { cause: e }
    }
}

impl From<cryptsetup_rs::device::Error> for Fido2LuksError {
    fn from(e: cryptsetup_rs::device::Error) -> Self {
        LuksError { cause: e }
    }
}

impl From<io::Error> for Fido2LuksError {
    fn from(e: io::Error) -> Self {
        IoError { cause: e }
    }
}

impl From<FromUtf8Error> for Fido2LuksError {
    fn from(e: FromUtf8Error) -> Self {
        StringEncodingError { cause: e }
    }
}

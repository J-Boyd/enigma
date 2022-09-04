use std::{error::Error as StdError, fmt::{Display, Formatter}};

#[derive(Debug)]
pub enum Error {
    RotorError,
    ReflectorError,
    PlugboardError,
    InputError,
    IOError(std::io::Error),
    Exit,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            Error::RotorError => write!(f, "Rotor Error!"),
            Error::ReflectorError => write!(f, "Reflector Error!"),
            Error::PlugboardError => write!(f, "Plugboard Error!"),
            Error::InputError => write!(f, "Input Error!"),
            Error::IOError(source) => write!(f, "IO Error!\n\nCause: {}", source),
            Error::Exit => write!(f, "Exit"),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match &self {
            Error::RotorError => None,
            Error::ReflectorError => None,
            Error::PlugboardError => None,
            Error::InputError => None,
            Error::IOError(source) => Some(source),
            Error::Exit => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IOError(err)
    }
}
use std::{error::Error, fmt};

#[derive(Debug)]
pub struct EnigmaError {
    pub kind: ErrorKind,
    pub description: String,
}

impl EnigmaError {
    pub fn new(kind: ErrorKind, description: String) -> EnigmaError {
        EnigmaError {
            kind,
            description
        }
    }
}

impl Error for EnigmaError {

}

impl fmt::Display for EnigmaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::RotorError() => write!(f, "Rotor Error: {}", self.description),
            ErrorKind::ReflectorError() => write!(f, "Reflector Error: {}", self.description),
            ErrorKind::PlugboardError() => write!(f, "Plugboard Error: {}", self.description),
            ErrorKind::InputError() => write!(f, "Input Error: {}", self.description),
            ErrorKind::IOError(err) => write!(f, "IO Error: {}", err),
            ErrorKind::Exit() => write!(f, "Exit"),
        }
    }
}

// TODO - Consider making these more fine grain and get rid of description.
#[derive(Debug)]
pub enum ErrorKind {
    RotorError(),
    ReflectorError(),
    PlugboardError(),
    InputError(),
    IOError(std::io::Error),
    Exit(),
}
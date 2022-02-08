#[derive(Debug)]
pub enum Error {
    Parse(ParseCause),
}

#[derive(Debug)]
pub enum ParseCause {
    InvalidLength,
    InvalidChar,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parse(c) => write!(f, "parsing error: {:?}", c),
        }
    }
}

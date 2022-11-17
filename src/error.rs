pub enum ErrorKind {
    Error(Box<dyn std::error::Error>),
    IOError(std::io::Error),
    ParseError(std::num::ParseIntError),
    ParseFloatError(std::num::ParseFloatError),
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErrorKind::Error(e) => write!(f, "{}", e),
            ErrorKind::IOError(e) => write!(f, "{}", e),
            ErrorKind::ParseError(e) => write!(f, "{}", e),
            ErrorKind::ParseFloatError(e) => write!(f, "{}", e),
        }
    }
}

impl From<std::io::Error> for ErrorKind {
    fn from(e: std::io::Error) -> Self {
        ErrorKind::IOError(e)
    }
}

impl From<std::num::ParseIntError> for ErrorKind {
    fn from(e: std::num::ParseIntError) -> Self {
        ErrorKind::ParseError(e)
    }
}

impl From<std::num::ParseFloatError> for ErrorKind {
    fn from(e: std::num::ParseFloatError) -> Self {
        ErrorKind::ParseFloatError(e)
    }
}

impl From<Box<dyn std::error::Error>> for ErrorKind {
    fn from(e: Box<dyn std::error::Error>) -> Self {
        ErrorKind::Error(e)
    }
}
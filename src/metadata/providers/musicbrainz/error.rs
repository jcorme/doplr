#[derive(Debug)]
pub enum Error {
    DecodingError(serde_json::Error),
    HTTPError(reqwest::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        use Error::*;
        match self {
            DecodingError(e) => write!(fmt, "{}", e),
            HTTPError(e) => write!(fmt, "{}", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use Error::*;
        match self {
            DecodingError(e) => Some(e),
            HTTPError(e) => Some(e),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::HTTPError(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::DecodingError(e)
    }
}

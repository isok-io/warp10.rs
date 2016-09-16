extern crate hyper;
extern crate url;

use std::{error, fmt, result};

#[derive(Debug)]
pub enum Error {
    HttpError(hyper::Error),
    UrlError(url::ParseError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::HttpError(ref err) => write!(f, "Warp10 HTTP error: {}", err),
            Error::UrlError(ref err)  => write!(f, "Warp10 URL error: {}",  err),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::HttpError(ref err) => err.description(),
            Error::UrlError(ref err)  => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::HttpError(ref err) => Some(err),
            Error::UrlError(ref err)  => Some(err),
        }
    }
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Error {
        Error::HttpError(err)
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Error {
        Error::UrlError(err)
    }
}

pub type Result<T> = result::Result<T, Error>;

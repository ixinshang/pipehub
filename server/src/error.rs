use actix_http::body::Body;
use actix_http::{Response, ResponseError};
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    Initialization(config::ConfigError),
    DataAccess(String),
    Execution(String),
    Io(std::io::Error),
    Dependency(String),
    Unexpected(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> Response<Body> {
        Response::InternalServerError().body(self.to_string())
    }
}

impl From<config::ConfigError> for Error {
    fn from(e: config::ConfigError) -> Self {
        Error::Initialization(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<r2d2::Error> for Error {
    fn from(e: r2d2::Error) -> Self {
        Error::DataAccess(e.to_string())
    }
}

impl From<diesel::result::Error> for Error {
    fn from(e: diesel::result::Error) -> Self {
        Error::DataAccess(e.to_string())
    }
}

impl From<actix_http::Error> for Error {
    fn from(e: actix_http::Error) -> Self {
        Error::Execution(e.to_string())
    }
}

impl<E> From<actix_threadpool::BlockingError<E>> for Error
where
    E: fmt::Debug,
{
    fn from(e: actix_threadpool::BlockingError<E>) -> Self {
        Error::DataAccess(e.to_string())
    }
}

impl From<oauth2::RequestTokenError<oauth2::basic::BasicErrorResponseType>> for Error {
    fn from(e: oauth2::RequestTokenError<oauth2::basic::BasicErrorResponseType>) -> Self {
        Error::Dependency(e.to_string())
    }
}

impl From<base58::FromBase58Error> for Error {
    fn from(e: base58::FromBase58Error) -> Self {
        Error::Unexpected(format!("{:?}", e))
    }
}

impl From<actix_http::client::SendRequestError> for Error {
    fn from(e: actix_http::client::SendRequestError) -> Self {
        Error::Dependency(e.to_string())
    }
}

impl From<awc::error::JsonPayloadError> for Error {
    fn from(e: awc::error::JsonPayloadError) -> Self {
        Error::Dependency(e.to_string())
    }
}

use std::fmt::{self, Display};

use failure::{Backtrace, Context, Fail};
use lambda_runtime::error::HandlerError;

/// The error type for Vicuna operations. Wraps an [`ErrorKind`].
///
/// [`ErrorKind`]: enum.ErrorKind.html
#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }
}

impl Fail for Error {
    fn cause(&self) -> Option<&dyn Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl From<HandlerError> for Error {
    fn from(err: HandlerError) -> Error {
        ErrorKind::Handler(err).into()
    }
}

impl From<failure::Error> for Error {
    fn from(err: failure::Error) -> Error {
        ErrorKind::Failure(err).into()
    }
}

/// A list specifying kinds of errors Vicuna handlers may encounter.
#[derive(Debug, Fail)]
pub enum ErrorKind {
    /// A `HandlerError`, as provided by the `lambda_rust` crate.
    #[fail(display = "{}", _0)]
    Handler(#[cause] HandlerError),

    /// A `failure::Error`, as provided by the `failure` crate.
    #[fail(display = "{}", _0)]
    Failure(#[cause] failure::Error),
}

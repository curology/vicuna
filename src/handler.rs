use std::result::Result;

use lambda_http::{Body, Request, Response};
use lambda_runtime::error::HandlerError;

use crate::error;

type LambdaResponse = Response<Body>;

type HandlerResult<E = error::Error> = Result<LambdaResponse, E>;

/// A trait which provides a method for handling requests.
pub trait Handle {
    /// Handles a request.
    fn handle(self, req: Request) -> Result<LambdaResponse, HandlerError>;
}

/// A trait which provides a method for wrapping handlers with middleware.
pub trait WrapWith<E>: Handle {
    /// Wraps a handler with the provided middleware, returning a new handler.
    fn wrap_with<M: Fn(Handler<E>) -> Handler<E>>(self, middleware: M) -> Handler<E>;
}

/// A type alias for handler functions.
///
/// While a default error type is provided, callers may provide their own alternative. This is
/// particularly important when your application deals with error types that are not supported out
/// of the box, such as Serde JSON errors.
pub type Handler<E = error::Error> = Box<dyn Fn(Request) -> HandlerResult<E>>;

impl<E> Handle for Handler<E>
where
    E: 'static + Sync + Send + failure::Fail + From<failure::Error>,
{
    /// Handles a request, returning a `HandlerResult`. Any errors will be mapped to a
    /// `failure::Error`.
    fn handle(self, req: Request) -> Result<LambdaResponse, HandlerError> {
        Ok(self(req).map_err(|e| -> failure::Error { e.into() })?)
    }
}

impl<E> WrapWith<E> for Handler<E>
where
    E: 'static + Sync + Send + failure::Fail + From<failure::Error>,
{
    fn wrap_with<M: Fn(Handler<E>) -> Handler<E>>(self, middleware: M) -> Handler<E> {
        middleware(self)
    }
}

/// A default handler which returns a successful response.
///
/// This is often useful as the beginning of a handler which will be wrapped
/// with middleware.
pub fn default_handler<E>() -> Handler<E> {
    Box::new(|_| Ok(Response::default()))
}

use std::result::Result;

use lambda_http::{Body, Request, Response};

use crate::error;
use crate::middleware::HasMiddleware;

type LambdaResponse = Response<Body>;

type HandlerResult<E = error::Error> = Result<LambdaResponse, E>;

/// A type alias for handler functions.
///
/// While a default error type is provided, callers may provide their own alternative. This is
/// particularly important when your application deals with error types that are not supported out
/// of the box, such as Serde JSON errors.
pub type Handler<E = error::Error> = Box<dyn Fn(Request) -> HandlerResult<E>>;

impl<E> HasMiddleware<E> for Handler<E> {
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

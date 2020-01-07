use std::result::Result;

use lambda_http::{Body, Request, Response};
use lambda_runtime::{error::HandlerError, Context};

use crate::error;

type LambdaResponse = Response<Body>;

type HandlerResult<E> = Result<LambdaResponse, E>;

/// A type alias for handler functions.
///
/// While a default error type is provided, callers may provide their own alternative. This is
/// particularly important when your application deals with error types that are not supported out
/// of the box, such as Serde JSON errors.
pub type Handler<E = error::Error> = Box<dyn Fn(Request, Context) -> HandlerResult<E>>;

/// A trait that houses methods related to handlers that can be wrapped with middleware.
pub trait WrappingHandler<E> {
    /// Wraps a `Handler` with the provided middleware, returning a new `Handler`.
    fn wrap_with<M: Fn(Handler<E>) -> Handler<E>>(self, middleware: M) -> Handler<E>
    where
        Self: 'static + Fn(Request, Context) -> Result<LambdaResponse, E> + Sized,
    {
        middleware(Box::new(self))
    }

    /// Returns a `Handler` that maps errors to `HandlerError`, suitable for passing directly to
    /// the `lambda` macro.
    fn handler(self) -> Handler<HandlerError>
    where
        Self: 'static + Fn(Request, Context) -> Result<LambdaResponse, E> + Sized,
        E: Send + Sync + failure::Fail + From<failure::Error>,
    {
        Box::new(move |request, context| {
            Ok(self(request, context).map_err(|e| -> failure::Error { e.into() })?)
        })
    }
}

impl<E> WrappingHandler<E> for Handler<E> {}

/// A default handler which returns a successful response.
///
/// This is often useful as the beginning of a handler which will be wrapped
/// with middleware.
pub fn default_handler<E>() -> Handler<E> {
    Box::new(|_, _| Ok(Response::default()))
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use lambda_http::{
        http::{
            header::{HeaderName, HeaderValue},
            StatusCode,
        },
        Body, IntoResponse,
    };
    use lambda_runtime::Context;

    use super::*;

    fn echo_body(_: Handler) -> Handler {
        Box::new(move |request, _context| Ok(request.into_body().into_response()))
    }

    fn hello_world(_: Handler) -> Handler {
        Box::new(move |_request, _context| {
            Ok(Response::builder().body("Hello, world!")?.into_response())
        })
    }

    fn add_header(handler: Handler) -> Handler {
        Box::new(move |request, context| {
            let mut resp = handler(request, context)?;
            resp.headers_mut().insert(
                HeaderName::from_static("x-hello"),
                HeaderValue::from_static("world"),
            );
            Ok(resp)
        })
    }

    fn handler_resp<E: Debug>(handler: Handler<E>) -> LambdaResponse {
        let request = Request::default();
        let context = Context::default();
        handler(request, context).unwrap()
    }

    #[test]
    fn test_wrapping_handler_echo_body() {
        let handler = default_handler().wrap_with(echo_body).handler();
        let resp = handler_resp(handler);
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(resp.into_body(), Body::default());
    }

    #[test]
    fn test_wrapping_handler_hello_world() {
        let handler = default_handler().wrap_with(hello_world).handler();
        let resp = handler_resp(handler);
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(resp.into_body(), Body::Text("Hello, world!".to_string()));
    }

    #[test]
    fn test_wrapping_handler_chaining() {
        let handler = default_handler()
            .wrap_with(hello_world)
            .wrap_with(add_header)
            .handler();
        let resp = handler_resp(handler);
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get("x-hello"),
            Some(&HeaderValue::from_static("world"))
        );
        assert_eq!(resp.into_body(), Body::Text("Hello, world!".to_string()));
    }
}

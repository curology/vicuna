#![deny(clippy::all, missing_docs)]
//! AWS Lambdas made simple.
//!
//! Vicuna is a library for writing AWS Lambda services using a simple middleware pattern.
//! Middleware can be composed to enable flexible and extensible request and response handling.
//! Because middleware are just functions, they are easy to test and naturally modular.
//!
//! The library is built on top of the [`lambda_runtime`] crate and is meant to be paired with the
//! [`serverless-rust`] plugin for the [Serverless framework].
//!
//!
//! ## Design
//!
//! Middleware are functions which are passed a [`Handler`] and return a `Handler`. Ultimately the
//! `Handler` will process a [`lambda_http::Request`]. The request is passed through the middleware
//! to allow for arbitrary processing of the request-response lifecycle.
//!
//! `Handler` types take in a `lambda_http::Request` and return a [`HandlerResult`].This result can
//! be unfurled to allow for manipulation of the response at that point in the middleware chain.
//!
//! ## Examples
//!
//! To make this more concrete, let's demonstrate what the structure of middleware looks like.
//!
//! ```rust,no_run
//! use vicuna::Handler;
//!
//! fn my_middleware(handler: Handler) -> Handler {
//!     Box::new(move |request, context| {
//!         // Resolve upstream middleware chain into a response...
//!         let mut response = handler(request, context);
//!         // ...mutate response as desired.
//!         response
//!     })
//! }
//! ```
//!
//! More complex variations of this are possible. For instance, we could introspect the request and
//! conditionally respond based on how it's formed. This is often useful for things such as input
//! validation, event filtering, or additional routing.
//!
//! A chain of middleware must be passed a `Handler`. A [`default_handler`] is provided as a
//! convenience and can be used to start a chain of middleware. Once the chain is established, we
//! are ready to provide it as a handler to the `lambda_runtime` framework via the [`lambda!`] macro.
//!
//! To illustrate, let's examine an example that utilizes builtin middleware.
//!
//! ```rust,no_run
//! use vicuna::{
//!     default_handler,
//!     error,
//!     lambda_http::lambda,
//!     middleware::{body, header},
//!     Handler, WrappingHandler,
//! };
//!
//! lambda!(default_handler::<error::Error>()
//!     .wrap_with(body("Hello, world!"))
//!     .wrap_with(header("x-foo", "bar"))
//!     .handler())
//! ```
//!
//! This is a simple example that demonstrates how straightforward it is to establish an AWS Lambda
//! function that's capable of arbitrary request and response handling via a set of composable and
//! reusable middleware functions.
//!
//! [Serverless framework]: https://serverless.com
//! [`HandlerResult`]: handler/type.HandlerResult.html
//! [`Handler`]: handler/type.Handler.html
//! [`default_handler`]: handler/fn.default_handler.html
//! [`lambda!`]: ../lambda_runtime/macro.lambda.html
//! [`lambda_http::Request`]: ../lambda_http/type.Request.html
//! [`lambda_runtime`]: ../lambda_runtime/index.html
//! [`serverless-rust`]: https://github.com/softprops/serverless-rust

pub use handler::{default_handler, Handler, WrappingHandler};
pub use middleware::Middleware;

pub use lambda_http;
pub use lambda_runtime;

/// Error and result types.
pub mod error;

/// AWS Lambda request handler.
pub mod handler;

/// Handler middleware type and trait.
pub mod middleware;

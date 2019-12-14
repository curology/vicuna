use crate::handler::Handler;
use crate::error;

/// Middleware type alias.
pub type Middleware<E = error::Error> = Box<dyn Fn(Handler<E>) -> Handler<E>>;

/// A trait which enables wrapping of middleware around the handler.
pub trait HasMiddleware<E> {
    /// Wraps a handler with the provided middleware, returning a new handler.
    fn wrap_with<M: Fn(Handler<E>) -> Handler<E>>(self, middleware: M) -> Handler<E>;
}

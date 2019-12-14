use crate::handler::Handler;

/// Middleware type alias.
pub type Middleware<E> = Box<dyn Fn(Handler<E>) -> Handler<E>>;

/// A trait which enables wrapping of middleware around the handler.
pub trait HasMiddleware<E> {
    /// Wraps a handler with the provided middleware, returning a new handler.
    fn wrap_with<M: Fn(Handler<E>) -> Handler<E>>(self, middleware: M) -> Handler<E>;
}

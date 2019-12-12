use crate::handler::Handler;

/// A trait which enables wrapping of middleware around the handler.
pub trait HasMiddleware<R> {
    /// Wraps a handler with the provided middleware, returning a new handler.
    fn wrap_with<M: Fn(Handler<R>) -> Handler<R>>(self, middleware: M) -> Handler<R>;
}

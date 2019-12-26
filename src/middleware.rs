use crate::error;
use crate::handler::Handler;

/// Middleware type alias.
pub type Middleware<E = error::Error> = Box<dyn Fn(Handler<E>) -> Handler<E>>;

use lambda_http::{
    http::{
        header::{HeaderValue, IntoHeaderName},
        StatusCode,
    },
    Body,
};

use crate::{error, handler::Handler};

/// Middleware type alias.
pub type Middleware<E = error::Error> = Box<dyn Fn(Handler<E>) -> Handler<E>>;

/// Adds the given header name and value to the response.
pub fn header<E: 'static, N, V>(header_name: N, header_value: V) -> Middleware<E>
where
    N: IntoHeaderName + Copy + 'static,
    V: Into<String> + Copy + 'static,
{
    Box::new(move |handler| {
        Box::new(move |request, context| {
            let mut response = handler(request, context)?;
            if let Ok(value) = HeaderValue::from_str(header_value.into().as_str()) {
                response.headers_mut().insert(header_name, value);
            }
            Ok(response)
        })
    })
}

/// Alters the status code of the response.
pub fn status<E: 'static>(code: StatusCode) -> Middleware<E> {
    Box::new(move |handler| {
        Box::new(move |request, context| {
            let mut response = handler(request, context)?;
            *response.status_mut() = code;
            Ok(response)
        })
    })
}

/// Alters the body of the response.
pub fn body<E: 'static, B>(body: B) -> Middleware<E>
where
    B: Into<Body> + Copy + 'static,
{
    Box::new(move |handler| {
        Box::new(move |request, context| {
            let mut response = handler(request, context)?;
            *response.body_mut() = body.into();
            Ok(response)
        })
    })
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use lambda_http::{http::StatusCode, Body, Request, Response};
    use lambda_runtime::Context;

    use crate::handler::{default_handler, WrappingHandler};

    use super::*;

    fn handler_resp<E: Debug>(handler: Handler<E>) -> Response<Body> {
        let request = Request::default();
        let context = Context::default();
        handler(request, context).unwrap()
    }

    #[test]
    fn test_header() {
        let handler = default_handler::<error::Error>()
            .wrap_with(header("x-foo", "bar"))
            .handler();
        let resp = handler_resp(handler);
        assert_eq!(
            resp.headers().get("x-foo"),
            Some(&HeaderValue::from_static("bar"))
        );
    }

    #[test]
    fn test_status() {
        let handler = default_handler::<error::Error>()
            .wrap_with(status(StatusCode::CREATED))
            .handler();
        let resp = handler_resp(handler);
        assert_eq!(resp.status(), StatusCode::CREATED);
    }

    #[test]
    fn test_body() {
        let handler = default_handler::<error::Error>()
            .wrap_with(body("foo"))
            .handler();
        let resp = handler_resp(handler);
        assert_eq!(*resp.body(), Body::Text("foo".to_string()));
    }
}

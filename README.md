# Vicuna

[![GitHub Actions](https://github.com/PocketDerm/vicuna/workflows/Rust/badge.svg)](https://github.com/PocketDerm/vicuna/actions) [![crates.io](http://meritbadge.herokuapp.com/vicuna)](https://crates.io/crates/vicuna) [![Released API docs](https://docs.rs/vicuna/badge.svg)](http://docs.rs/vicuna) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

> AWS Lambdas in Rust made simple. 

- Simple, middleware-based interface
- Naturally modular design
- Purpose-built for [`serverless-rust`](https://www.npmjs.com/package/serverless-rust)

> ⚠️ **Active Development**: Vicuna's API has not stabalized and may change without warning between releases!

## 📦 Install

Add the following to your `Cargo.toml` file.

```toml
[dependencies]
vicuna = "0.4.0"
```

## 🤸 Usage

> 💡 This crate is intended to be paired with the [`serverless-rust`](https://www.npmjs.com/package/serverless-rust) plugin.

Vicuna produces handlers which take in a Lambda request and produce an
appropriate response. The simplest handler is the `default_handler` provided by
the crate:

```rust
use lambda_http::lambda;
use vicuna::default_handler;

fn main() {
    lambda!(default_handler())
}
```

Handlers can be composed from middleware which can handle the request-response
lifecycle in an arbitrary fashion. For example, a middleware that adds a
header to our response could look like this:

```rust
use lambda_http::http::header::{HeaderName, HeaderValue};
use vicuna::Handler;

fn add_header(handler: Handler) -> Handler {
    Box::new(move |request, context| {
        // Resolve any upstream middleware into a response.
        let mut resp = handler(request, context)?;
        // Add our custom header to the response.
        resp.headers_mut().insert(
            HeaderName::from_static("x-hello"),
            HeaderValue::from_static("world"),
        );
        Ok(resp)
    })
}
```

Middleware are wrapped around handlers, which themselves produce a handler for
chainable invocation:

```rust
use lambda_http::lambda;
use vicuna::{default_handler, WrappingHandler};

fn main() {
    lambda!(default_handler()
        .wrap_with(say_hello)
        .wrap_with(add_header)
        .handler())
}
```

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
vicuna = "0.4.1"
```

## 🤸 Usage

> 💡 This crate is intended to be paired with the [`serverless-rust`](https://www.npmjs.com/package/serverless-rust) plugin.

Vicuna produces handlers which take in a Lambda request and produce an
appropriate response. The simplest handler is the `default_handler` provided by
the crate:

```rust
use vicuna::{default_handler, lambda_http::lambda};

fn main() {
    lambda!(default_handler())
}
```

Handlers can be composed from middleware which can handle the request-response
lifecycle in an arbitrary fashion. For example, custom middleware can be
written like so:

```rust
use vicuna::Handler;

fn my_middleware(handler: Handler) -> Handler {
    Box::new(move |request, context| {
        // Resolve upstream middleware chain into a response...
        let mut response = handler(request, context);
        // ...mutate response as desired.
        Ok(response)
    })
}
```

Middleware are wrapped around handlers, which themselves produce a handler for
chainable invocation:

```rust
use vicuna::{
    default_handler,
    lambda_http::lambda,
    middleware::{body, header},
    Handler,
    WrappingHandler,
};

fn main() {
    lambda!(default_handler()
        .wrap_with(body("Hello, world!"))
        .wrap_with(header("x-foo", "bar"))
        .handler())
}
```

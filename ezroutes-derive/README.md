# ezroutes-derive

> Procedural macro implementation for `ezroutes`.

[![Crates.io](https://img.shields.io/crates/v/ezroutes-derive.svg)](https://crates.io/crates/ezroutes-derive)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

---

## ⚠️ Important

**Do not add this crate directly to your project.**

`ezroutes-derive` is an internal implementation crate used by `ezroutes`.
It is automatically included when you add the main crate.

Use this instead:

```toml
[dependencies]
ezroutes = "0.1"
```

---

## Overview

`ezroutes-derive` provides the procedural macros that power `ezroutes`.

It parses route modules annotated with `#[routes]` and generates Axum `Router` code automatically at compile time.

This allows `ezroutes` to provide:

- Attribute-based route definitions
- Automatic router generation
- Middleware composition
- Reduced Axum boilerplate
- Fully typed compile-time routing

---

## Example

### User Code

```rust
use ezroutes::routes;

#[routes(state = AppState)]
pub mod bookmark_routes {

    #[get("/health")]
    pub async fn health_check() -> &'static str {
        "OK"
    }

    #[get("/", middleware = [auth_middleware])]
    pub async fn list_bookmarks() -> Vec<Bookmark> {
        // ...
    }

    #[post("/", middleware = [auth_middleware])]
    pub async fn create_bookmark() -> Result<Bookmark, AppError> {
        // ...
    }
}
```

---

### Generated Internally

```rust
pub mod bookmark_routes {

    pub fn router(state: AppState) -> Router<AppState> {

        Router::new()
            .route(
                "/health",
                get(health_check),
            )
            .route(
                "/",
                get(list_bookmarks),
            )
            .route_layer(
                middleware::from_fn_with_state(
                    state.clone(),
                    auth_middleware,
                ),
            )
            .route(
                "/",
                post(create_bookmark),
            )
            .route_layer(
                middleware::from_fn_with_state(
                    state.clone(),
                    auth_middleware,
                ),
            )

        // additional generated routes...
    }
}
```

---

## Purpose

This crate exists primarily to separate:

- Procedural macro implementation
- Public runtime API
- Internal code generation logic

This is the standard Rust ecosystem pattern for proc-macro crates.

---

## Related Crates

| Crate | Purpose |
|---|---|
| `ezroutes` | Public runtime crate |
| `ezroutes-derive` | Internal procedural macro crate |

---

## License

MIT License

Made by [Samujalphukan228](https://github.com/Samujalphukan228)
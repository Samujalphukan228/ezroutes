# ezroutes

> Zero-boilerplate Axum route definitions with middleware support.

[![Crates.io](https://img.shields.io/crates/v/ezroutes.svg)](https://crates.io/crates/ezroutes)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

---

## Motivation

Defining routes in Axum usually ends up with lots of repetitive boilerplate:

```rust
use axum::{
    routing::{get, post, put, delete},
    Router,
};

let app = Router::new()
    .route("/health", get(health_check))
    .route("/bookmarks", get(list_bookmarks))
    .route("/bookmarks", post(create_bookmark))
    .route("/bookmarks/:id", get(get_bookmark))
    .route("/bookmarks/:id", put(update_bookmark))
    .route("/bookmarks/:id", delete(delete_bookmark))
    .layer(middleware::from_fn_with_state(
        state.clone(),
        auth_middleware,
    ))
    .layer(CorsLayer::permissive())
    .layer(TraceLayer::new_for_http());
```

This becomes noisy and repetitive as APIs grow.

**ezroutes** removes this repetition by allowing routes to be declared directly on handler functions using attribute macros.

---

## Features

- Simple attribute macros:
  - `#[get]`
  - `#[post]`
  - `#[put]`
  - `#[delete]`
  - `#[patch]`
- Group routes inside a module
- Route-level middleware support
- Group-level middleware support
- Automatic `Router` generation
- Full compatibility with:
  - Axum extractors
  - Axum handlers
  - Tower middleware
- Zero runtime overhead

---

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ezroutes = "0.1"
axum = "0.7"
tokio = { version = "1", features = ["full"] }
```

---

## Quick Example

### Define Routes

```rust
use axum::{
    extract::Path,
    Json,
};

use ezroutes::routes;

#[derive(Clone)]
pub struct AppState;

pub struct Bookmark;
pub struct CreateBookmark;
pub struct AppError;

#[routes(state = AppState)]
pub mod bookmark_routes {

    use super::*;

    #[get("/health")]
    pub async fn health_check() -> &'static str {
        "OK"
    }

    #[get("/")]
    pub async fn list_bookmarks() -> Vec<Bookmark> {
        vec![]
    }

    #[post("/")]
    pub async fn create_bookmark(
        payload: Json<CreateBookmark>,
    ) -> Result<Json<Bookmark>, AppError> {
        todo!()
    }

    #[get("/:id")]
    pub async fn get_bookmark(
        path: Path<u32>,
    ) -> Result<Json<Bookmark>, AppError> {
        todo!()
    }

    #[put("/:id")]
    pub async fn update_bookmark(
        path: Path<u32>,
        payload: Json<CreateBookmark>,
    ) -> Result<Json<Bookmark>, AppError> {
        todo!()
    }

    #[delete("/:id", middleware = [auth_middleware, admin_only])]
    pub async fn delete_bookmark(
        path: Path<u32>,
    ) -> Result<(), AppError> {
        todo!()
    }
}
```

---

### Use the Generated Router

```rust
use axum::Router;

#[tokio::main]
async fn main() {

    let state = AppState;

    let app = Router::new()
        .nest(
            "/bookmarks",
            bookmark_routes::router(state.clone()),
        )
        .with_state(state);
}
```

---

## Middleware Support

### Route-Level Middleware

```rust
#[get(
    "/admin",
    middleware = [auth_middleware, admin_only]
)]
pub async fn admin_dashboard() -> &'static str {
    "secret"
}
```

### Group-Level Middleware

```rust
#[routes(
    state = AppState,
    middleware = [auth_middleware]
)]
pub mod protected_routes {

    #[get("/profile")]
    pub async fn profile() -> &'static str {
        "profile"
    }

    #[get("/settings")]
    pub async fn settings() -> &'static str {
        "settings"
    }
}
```

---

## Generated API

For every module annotated with `#[routes]`, ezroutes generates:

```rust
pub fn router(state: AppState) -> axum::Router
```

You can compose routers normally using standard Axum APIs.

---

## Why ezroutes?

### Without ezroutes

```rust
Router::new()
    .route("/users", get(list_users))
    .route("/users", post(create_user))
    .route("/users/:id", get(get_user))
    .route("/users/:id", put(update_user))
    .route("/users/:id", delete(delete_user))
```

### With ezroutes

```rust
#[routes(state = AppState)]
mod users {

    #[get("/")]
    async fn list_users() {}

    #[post("/")]
    async fn create_user() {}

    #[get("/:id")]
    async fn get_user() {}

    #[put("/:id")]
    async fn update_user() {}

    #[delete("/:id")]
    async fn delete_user() {}
}
```

Cleaner, easier to scale, and significantly less boilerplate.

---

## When to Use

### Use ezroutes if:

- Your API has many endpoints
- You use middleware extensively
- You want cleaner route organization
- You prefer declarative routing
- You want to reduce repetitive `.route(...)` calls

### Plain Axum may be simpler if:

- Your project is very small
- You only have a few endpoints
- You prefer explicit router construction

---

## Compatibility

| Component | Supported |
|---|---|
| Axum 0.7 | ✅ |
| Extractors | ✅ |
| State | ✅ |
| Middleware | ✅ |
| Nested Routers | ✅ |
| Async Handlers | ✅ |

---

## Future Plans

- OpenAPI generation
- Route guards
- Typed middleware composition
- Better compile-time diagnostics
- Automatic documentation generation

---

## License

MIT License

Made by [Samujalphukan228](https://github.com/Samujalphukan228)
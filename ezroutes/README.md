# ezroutes

> Zero-boilerplate Axum route definitions with middleware support.

[![Crates.io](https://img.shields.io/crates/v/ezroutes.svg)](https://crates.io/crates/ezroutes)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

---

## Motivation

Axum route setup often becomes repetitive and difficult to maintain as applications grow.

**ezroutes** allows you to define routes directly on handler functions using simple attribute macros, reducing boilerplate while preserving full compatibility with Axum.

---

## Features

- Attribute-based route definitions:
  - `#[get]`
  - `#[post]`
  - `#[put]`
  - `#[delete]`
  - `#[patch]`
  - `#[head]`
  - `#[options]`
- Automatic `Router` generation
- Per-route middleware support
- Group-level middleware support
- Clean module-based route organization
- Full support for:
  - Axum extractors
  - Axum handlers
  - Tower middleware
- Zero runtime overhead

---

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
ezroutes = "0.1"
axum = "0.8"
tokio = { version = "1", features = ["full"] }
```

---

## Basic Usage

```rust
use axum::Json;
use ezroutes::routes;

#[routes(state = AppState)]
pub mod bookmark_routes {

    #[get("/health")]
    pub async fn health_check() -> &'static str {
        "OK"
    }

    #[get("/")]
    pub async fn list_bookmarks() -> Vec<Bookmark> {
        // ...
    }

    #[post("/")]
    pub async fn create_bookmark(
        Json(payload): Json<CreateBookmark>
    ) -> Result<Bookmark, AppError> {
        // ...
    }
}
```

### In `main.rs`

```rust
use axum::Router;

#[tokio::main]
async fn main() {

    let state = AppState::new();

    let app = bookmark_routes::router(state.clone())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}
```

---

## Middleware

You can attach middleware directly to routes.

```rust
use axum::extract::Path;

#[routes(state = AppState)]
pub mod bookmark_routes {

    // No middleware
    #[get("/health")]
    pub async fn health_check() -> &'static str {
        "OK"
    }

    // Single middleware
    #[get("/", middleware = [auth_middleware])]
    pub async fn list_bookmarks() -> Vec<Bookmark> {
        // ...
    }

    // Multiple middleware (executed left to right)
    #[delete("/:id", middleware = [auth_middleware, admin_only])]
    pub async fn delete_bookmark(
        path: Path<u32>
    ) -> Result<(), AppError> {
        // ...
    }
}
```

---

## Multiple Route Modules

```rust
use axum::Router;

let app = Router::new()
    .nest(
        "/bookmarks",
        bookmark_routes::router(state.clone())
    )
    .nest(
        "/auth",
        auth_routes::router(state.clone())
    )
    .with_state(state);
```

---

## Supported HTTP Methods

| Attribute      | HTTP Method |
|----------------|-------------|
| `#[get]`       | GET         |
| `#[post]`      | POST        |
| `#[put]`       | PUT         |
| `#[delete]`    | DELETE      |
| `#[patch]`     | PATCH       |
| `#[head]`      | HEAD        |
| `#[options]`   | OPTIONS     |

---

## Comparison with Vanilla Axum

| Feature | Vanilla Axum | ezroutes |
|---|---|---|
| Route Registration | `.route(path, method(handler))` | `#[get("/path")]` |
| Middleware per route | Manual `.layer()` nesting | `middleware = [fn1, fn2]` |
| Route Organization | Flat or manually nested | Module-based with `#[routes]` |
| Boilerplate | High | Very low |

---

## When to Use

### Recommended For

- Medium to large APIs
- Projects with many endpoints
- Heavy middleware usage
- Teams that prefer declarative routing

### Plain Axum May Be Simpler For

- Very small projects
- Minimal APIs
- Explicit router construction preferences

---

## Example: Vanilla Axum vs ezroutes

### Vanilla Axum

```rust
Router::new()
    .route("/users", get(list_users))
    .route("/users", post(create_user))
    .route("/users/:id", get(get_user))
    .route("/users/:id", put(update_user))
    .route("/users/:id", delete(delete_user));
```

### ezroutes

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

---

## License

MIT License

Made by [Samujalphukan228](https://github.com/Samujalphukan228)
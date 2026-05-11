//! # ezroutes
//!
//! Zero-boilerplate Axum route definitions with middleware support.
//!
//! ## Usage
//!
//! ```rust
//! use ezroutes::routes;
//!
//! #[routes(state = AppState)]
//! pub mod bookmark_routes {
//!
//!     #[get("/health")]
//!     pub async fn health_check() -> &'static str {
//!         "OK"
//!     }
//!
//!     #[get("/", middleware = [auth_middleware])]
//!     pub async fn list_bookmarks() -> &'static str {
//!         "bookmarks"
//!     }
//!
//!     #[post("/", middleware = [auth_middleware])]
//!     pub async fn create_bookmark() -> &'static str {
//!         "created"
//!     }
//! }
//!
//! // In main.rs
//! // let app = bookmark_routes::router(state.clone()).with_state(state);
//! ```

pub use ezroutes_derive::routes;
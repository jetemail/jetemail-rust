//! # JetEmail Rust SDK
//!
//! Official Rust SDK for the [JetEmail](https://jetemail.com) email API.
//!
//! ## Quick Start
//!
//! ```no_run
//! # #[tokio::main]
//! # async fn main() -> jetemail::Result<()> {
//! let client = jetemail::JetEmail::new("je_your_api_key");
//!
//! let email = jetemail::CreateEmailOptions::new(
//!     "you@example.com",
//!     "recipient@example.com",
//!     "Hello from JetEmail!",
//! )
//! .with_html("<h1>Hello!</h1>");
//!
//! let response = client.emails.send(email).await?;
//! println!("Sent email: {}", response.id);
//! # Ok(())
//! # }
//! ```
//!
//! ## Blocking Usage
//!
//! Enable the `blocking` feature in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! jetemail = { version = "0.1", features = ["blocking"] }
//! ```
//!
//! Then use the same API synchronously:
//!
//! ```ignore
//! let client = jetemail::JetEmail::new("je_your_api_key");
//! let response = client.emails.send(email)?;
//! ```

mod batch;
mod client;
mod config;
mod emails;
mod error;

// Re-export public API.
pub use batch::{BatchEmailResponse, BatchSvc};
pub use client::JetEmail;
pub use config::{Config, ConfigBuilder};
pub use emails::{
    Attachment, CreateEmailOptions, CreateEmailResponse, EmailsSvc, IntoEmailRecipients,
};
pub use error::{Error, ErrorResponse, Result};

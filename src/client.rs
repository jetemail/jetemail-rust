use std::sync::Arc;

use crate::batch::BatchSvc;
use crate::config::{Config, ConfigBuilder};
use crate::emails::EmailsSvc;

/// The JetEmail API client.
///
/// # Example
///
/// ```no_run
/// # #[tokio::main]
/// # async fn main() -> jetemail::Result<()> {
/// let client = jetemail::JetEmail::new("je_your_api_key");
///
/// let email = jetemail::CreateEmailOptions::new(
///     "you@example.com",
///     vec!["recipient@example.com".into()],
///     "Hello from JetEmail!",
/// )
/// .with_html("<h1>Hello!</h1>");
///
/// let response = client.emails.send(email).await?;
/// println!("Sent email: {}", response.id);
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct JetEmail {
    /// Service for sending individual emails.
    pub emails: EmailsSvc,
    /// Service for sending batch emails.
    pub batch: BatchSvc,
}

impl JetEmail {
    /// Create a new JetEmail client with the given API key.
    pub fn new(api_key: impl Into<String>) -> Self {
        let config = ConfigBuilder::new(api_key).build();
        Self::with_config(config)
    }

    /// Create a new JetEmail client with a custom reqwest client.
    #[cfg(not(feature = "blocking"))]
    pub fn with_client(api_key: impl Into<String>, client: reqwest::Client) -> Self {
        let config = ConfigBuilder::new(api_key).client(client).build();
        Self::with_config(config)
    }

    /// Create a new JetEmail client with a custom reqwest blocking client.
    #[cfg(feature = "blocking")]
    pub fn with_client(api_key: impl Into<String>, client: reqwest::blocking::Client) -> Self {
        let config = ConfigBuilder::new(api_key).client(client).build();
        Self::with_config(config)
    }

    /// Create a new JetEmail client from a [`Config`].
    pub fn with_config(config: Config) -> Self {
        let config = Arc::new(config);
        Self {
            emails: EmailsSvc(Arc::clone(&config)),
            batch: BatchSvc(Arc::clone(&config)),
        }
    }
}

impl Default for JetEmail {
    /// Create a JetEmail client using the `JETEMAIL_API_KEY` environment variable.
    ///
    /// # Panics
    ///
    /// Panics if `JETEMAIL_API_KEY` is not set.
    fn default() -> Self {
        let api_key = std::env::var("JETEMAIL_API_KEY")
            .expect("JETEMAIL_API_KEY environment variable must be set");
        Self::new(api_key)
    }
}

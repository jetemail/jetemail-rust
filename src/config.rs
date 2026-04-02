use reqwest::header::{ACCEPT, CONTENT_TYPE, USER_AGENT};
use reqwest::Url;

#[cfg(not(feature = "blocking"))]
use reqwest::{Client, Method, RequestBuilder};

#[cfg(feature = "blocking")]
use reqwest::blocking::{Client, RequestBuilder};
#[cfg(feature = "blocking")]
use reqwest::Method;

use crate::error::{Error, Result};

/// Configuration for the JetEmail API client.
#[derive(Clone, Debug)]
pub struct Config {
    pub(crate) api_key: String,
    pub(crate) base_url: Url,
    pub(crate) user_agent: String,
    pub(crate) client: Client,
}

/// Builder for constructing a [`Config`].
pub struct ConfigBuilder {
    api_key: String,
    base_url: Url,
    user_agent: String,
    client: Option<Client>,
}

impl ConfigBuilder {
    /// Create a new config builder with the given API key.
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: Url::parse("https://api.jetemail.com").expect("valid base URL"),
            user_agent: format!("jetemail-rust/{}", env!("CARGO_PKG_VERSION")),
            client: None,
        }
    }

    /// Set a custom base URL.
    pub fn base_url(mut self, url: impl AsRef<str>) -> Self {
        self.base_url = Url::parse(url.as_ref()).expect("valid URL");
        self
    }

    /// Set a custom user agent string.
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = user_agent.into();
        self
    }

    /// Set a custom reqwest client.
    pub fn client(mut self, client: Client) -> Self {
        self.client = Some(client);
        self
    }

    /// Build the config.
    pub fn build(self) -> Config {
        Config {
            api_key: self.api_key,
            base_url: self.base_url,
            user_agent: self.user_agent,
            client: self.client.unwrap_or_default(),
        }
    }
}

impl Config {
    /// Build an HTTP request with authentication headers.
    pub(crate) fn build_request(&self, method: Method, path: &str) -> RequestBuilder {
        let url = self.base_url.join(path).expect("valid API endpoint");
        self.client
            .request(method, url)
            .bearer_auth(&self.api_key)
            .header(USER_AGENT, &self.user_agent)
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/json")
    }

    /// Send a request and handle error responses.
    #[maybe_async::maybe_async]
    pub(crate) async fn send(
        &self,
        method: Method,
        path: &str,
        body: Option<serde_json::Value>,
    ) -> Result<serde_json::Value> {
        let mut req = self.build_request(method, path);

        if let Some(body) = body {
            req = req.json(&body);
        }

        let response = req.send().await?;
        let status = response.status();

        if status.is_success() {
            let text = response.text().await?;
            if text.is_empty() {
                return Ok(serde_json::Value::Null);
            }
            serde_json::from_str(&text).map_err(|e| Error::Parse(e.to_string()))
        } else {
            let status_code = status.as_u16();
            let body: Option<serde_json::Value> = response.json().await.ok();

            let message = body
                .as_ref()
                .and_then(|b: &serde_json::Value| {
                    b.get("message")
                        .or_else(|| b.get("error"))
                        .and_then(|v: &serde_json::Value| v.as_str())
                        .map(String::from)
                })
                .unwrap_or_else(|| format!("HTTP {status_code}"));

            Err(Error::JetEmail {
                message,
                status_code,
                response: body,
            })
        }
    }
}

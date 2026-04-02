use std::sync::Arc;

use serde::Deserialize;

use crate::config::Config;
use crate::emails::{CreateEmailOptions, CreateEmailResponse};
use crate::error::Result;

/// Response from sending a batch of emails.
#[derive(Clone, Debug, Deserialize)]
pub struct BatchEmailResponse {
    /// The responses for each email in the batch.
    pub data: Vec<CreateEmailResponse>,
}

/// Service for sending batch emails via the JetEmail API.
#[derive(Clone, Debug)]
pub struct BatchSvc(pub(crate) Arc<Config>);

impl BatchSvc {
    /// Send a batch of emails (up to 100).
    #[maybe_async::maybe_async]
    pub async fn send(&self, emails: Vec<CreateEmailOptions>) -> Result<BatchEmailResponse> {
        if emails.is_empty() {
            return Err(crate::error::Error::JetEmail {
                message: "Batch must contain at least one email".into(),
                status_code: 0,
                response: None,
            });
        }
        if emails.len() > 100 {
            return Err(crate::error::Error::JetEmail {
                message: "Batch cannot contain more than 100 emails".into(),
                status_code: 0,
                response: None,
            });
        }

        let body = serde_json::json!({ "emails": emails });
        let response = self
            .0
            .send(reqwest::Method::POST, "/email-batch", Some(body))
            .await?;
        serde_json::from_value(response).map_err(|e| crate::error::Error::Parse(e.to_string()))
    }
}

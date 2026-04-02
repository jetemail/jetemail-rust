use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::error::Result;

/// An email attachment.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Attachment {
    /// The filename of the attachment.
    pub filename: String,

    /// Base64-encoded file content.
    #[serde(rename = "data")]
    pub content: String,
}

impl Attachment {
    /// Create an attachment from raw bytes and a filename.
    pub fn from_content(content: impl AsRef<[u8]>, filename: impl Into<String>) -> Self {
        use base64::Engine;
        Self {
            filename: filename.into(),
            content: base64::engine::general_purpose::STANDARD.encode(content),
        }
    }

    /// Create an attachment from a file path.
    ///
    /// Reads the file and base64-encodes it. The filename defaults to the
    /// file's name component.
    pub fn from_path(path: impl AsRef<std::path::Path>) -> std::io::Result<Self> {
        let path = path.as_ref();
        let filename = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned();
        let bytes = std::fs::read(path)?;
        Ok(Self::from_content(bytes, filename))
    }
}

/// Options for creating and sending an email.
#[derive(Clone, Debug, Serialize)]
pub struct CreateEmailOptions {
    /// Sender email address.
    pub from: String,
    /// Recipient email address(es).
    pub to: Vec<String>,
    /// Email subject line.
    pub subject: String,
    /// HTML body content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    /// Plain text body content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// CC recipient(s).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc: Option<Vec<String>>,
    /// BCC recipient(s).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcc: Option<Vec<String>>,
    /// Reply-to address(es).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<Vec<String>>,
    /// Custom email headers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
    /// File attachments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
}

impl CreateEmailOptions {
    /// Create a new email with required fields.
    ///
    /// `to` accepts a single string or a `Vec<String>`.
    pub fn new(
        from: impl Into<String>,
        to: impl IntoEmailRecipients,
        subject: impl Into<String>,
    ) -> Self {
        Self {
            from: from.into(),
            to: to.into_recipients(),
            subject: subject.into(),
            html: None,
            text: None,
            cc: None,
            bcc: None,
            reply_to: None,
            headers: None,
            attachments: None,
        }
    }

    /// Set the HTML body.
    pub fn with_html(mut self, html: impl Into<String>) -> Self {
        self.html = Some(html.into());
        self
    }

    /// Set the plain text body.
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Set the CC recipients.
    pub fn with_cc(mut self, cc: impl IntoEmailRecipients) -> Self {
        self.cc = Some(cc.into_recipients());
        self
    }

    /// Set the BCC recipients.
    pub fn with_bcc(mut self, bcc: impl IntoEmailRecipients) -> Self {
        self.bcc = Some(bcc.into_recipients());
        self
    }

    /// Set the reply-to addresses.
    pub fn with_reply_to(mut self, reply_to: impl IntoEmailRecipients) -> Self {
        self.reply_to = Some(reply_to.into_recipients());
        self
    }

    /// Add a custom header.
    pub fn with_header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers
            .get_or_insert_with(HashMap::new)
            .insert(name.into(), value.into());
        self
    }

    /// Add an attachment.
    pub fn with_attachment(mut self, attachment: Attachment) -> Self {
        self.attachments
            .get_or_insert_with(Vec::new)
            .push(attachment);
        self
    }
}

/// Trait for types that can be converted into a list of email recipients.
pub trait IntoEmailRecipients {
    fn into_recipients(self) -> Vec<String>;
}

impl IntoEmailRecipients for &str {
    fn into_recipients(self) -> Vec<String> {
        vec![self.to_string()]
    }
}

impl IntoEmailRecipients for String {
    fn into_recipients(self) -> Vec<String> {
        vec![self]
    }
}

impl IntoEmailRecipients for Vec<String> {
    fn into_recipients(self) -> Vec<String> {
        self
    }
}

impl IntoEmailRecipients for &[&str] {
    fn into_recipients(self) -> Vec<String> {
        self.iter().map(|s| s.to_string()).collect()
    }
}

impl<const N: usize> IntoEmailRecipients for [&str; N] {
    fn into_recipients(self) -> Vec<String> {
        self.iter().map(|s| s.to_string()).collect()
    }
}

/// Response from sending an email.
#[derive(Clone, Debug, Deserialize)]
pub struct CreateEmailResponse {
    /// The ID of the sent email.
    pub id: String,
}

/// Service for sending individual emails via the JetEmail API.
#[derive(Clone, Debug)]
pub struct EmailsSvc(pub(crate) Arc<Config>);

impl EmailsSvc {
    /// Send an email.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> jetemail::Result<()> {
    /// let client = jetemail::JetEmail::new("je_api_key");
    /// let response = client.emails.send(
    ///     jetemail::CreateEmailOptions::new("from@example.com", "to@example.com", "Hello")
    ///         .with_html("<h1>Hello!</h1>")
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[maybe_async::maybe_async]
    pub async fn send(&self, email: CreateEmailOptions) -> Result<CreateEmailResponse> {
        let body =
            serde_json::to_value(&email).map_err(|e| crate::error::Error::Parse(e.to_string()))?;
        let response = self
            .0
            .send(reqwest::Method::POST, "/email", Some(body))
            .await?;
        serde_json::from_value(response).map_err(|e| crate::error::Error::Parse(e.to_string()))
    }
}

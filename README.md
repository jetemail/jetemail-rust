# JetEmail Rust SDK

The official Rust SDK for the [JetEmail](https://jetemail.com) email API.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
jetemail = "0.1"
```

## Usage

### Send an email

```rust
use jetemail::{CreateEmailOptions, JetEmail};

#[tokio::main]
async fn main() -> jetemail::Result<()> {
    let client = JetEmail::new("je_your_api_key");

    let email = CreateEmailOptions::new(
        "you@example.com",
        "recipient@example.com",
        "Hello from JetEmail!",
    )
    .with_html("<h1>Hello World!</h1>");

    let response = client.emails.send(email).await?;
    println!("Email sent! ID: {}", response.id);

    Ok(())
}
```

### Send to multiple recipients

```rust
let email = CreateEmailOptions::new(
    "you@example.com",
    vec!["alice@example.com".into(), "bob@example.com".into()],
    "Hello everyone!",
)
.with_html("<p>Hi there!</p>")
.with_cc("cc@example.com")
.with_bcc("bcc@example.com")
.with_reply_to("reply@example.com");
```

### Send with attachments

```rust
use jetemail::Attachment;

let email = CreateEmailOptions::new(
    "you@example.com",
    "recipient@example.com",
    "Email with attachment",
)
.with_html("<p>See attached.</p>")
.with_attachment(Attachment::from_content(b"file contents", "file.txt"))
.with_attachment(Attachment::from_path("./report.pdf")?);
```

### Send batch emails

Send up to 100 emails in a single API call:

```rust
let emails = vec![
    CreateEmailOptions::new("you@example.com", "alice@example.com", "Hello Alice!")
        .with_html("<p>Hi Alice!</p>"),
    CreateEmailOptions::new("you@example.com", "bob@example.com", "Hello Bob!")
        .with_html("<p>Hi Bob!</p>"),
];

let response = client.batch.send(emails).await?;
```

### Custom headers

```rust
let email = CreateEmailOptions::new(
    "you@example.com",
    "recipient@example.com",
    "With custom headers",
)
.with_html("<p>Hello!</p>")
.with_header("X-Custom-Header", "custom-value");
```

## Configuration

### Environment variable

```rust
// Uses JETEMAIL_API_KEY environment variable
let client = JetEmail::default();
```

### Advanced configuration

Use `ConfigBuilder` for full control over the client. This lets you override the API base URL (useful for testing against a local or staging server), set a custom user agent, or provide your own `reqwest` client with custom timeouts or proxy settings.

```rust
use jetemail::ConfigBuilder;

let config = ConfigBuilder::new("je_your_api_key")
    .base_url("https://staging-api.jetemail.com") // default: https://api.jetemail.com
    .user_agent("my-app/1.0")
    .client(
        reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap(),
    )
    .build();

let client = JetEmail::with_config(config);
```

## Blocking Usage

Enable the `blocking` feature for synchronous usage:

```toml
[dependencies]
jetemail = { version = "0.1", features = ["blocking"] }
```

```rust
let client = JetEmail::new("je_your_api_key");
let response = client.emails.send(email)?;
```

## License

MIT

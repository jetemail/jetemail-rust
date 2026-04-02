use jetemail::{CreateEmailOptions, JetEmail};

#[tokio::main]
async fn main() -> jetemail::Result<()> {
    let client = JetEmail::new("je_your_api_key");

    let emails = vec![
        CreateEmailOptions::new("you@example.com", "alice@example.com", "Hello Alice!")
            .with_html("<p>Hi Alice!</p>"),
        CreateEmailOptions::new("you@example.com", "bob@example.com", "Hello Bob!")
            .with_html("<p>Hi Bob!</p>"),
    ];

    let response = client.batch.send(emails).await?;
    for email in &response.data {
        println!("Sent email ID: {}", email.id);
    }

    Ok(())
}

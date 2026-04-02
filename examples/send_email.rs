use jetemail::{CreateEmailOptions, JetEmail};

#[tokio::main]
async fn main() -> jetemail::Result<()> {
    let client = JetEmail::new("je_your_api_key");

    let email = CreateEmailOptions::new(
        "you@example.com",
        "recipient@example.com",
        "Hello from JetEmail!",
    )
    .with_html("<h1>Hello World!</h1><p>Sent with JetEmail Rust SDK.</p>");

    let response = client.emails.send(email).await?;
    println!("Email sent! ID: {}", response.id);

    Ok(())
}

use jetemail::{Attachment, CreateEmailOptions, JetEmail};

#[tokio::main]
async fn main() -> jetemail::Result<()> {
    let client = JetEmail::new("je_your_api_key");

    let email = CreateEmailOptions::new(
        "you@example.com",
        "recipient@example.com",
        "Email with attachment",
    )
    .with_html("<p>Please see the attached file.</p>")
    .with_attachment(Attachment::from_content(b"Hello, World!", "hello.txt"))
    .with_cc("cc@example.com")
    .with_reply_to("reply@example.com");

    let response = client.emails.send(email).await?;
    println!("Email sent! ID: {}", response.id);

    Ok(())
}

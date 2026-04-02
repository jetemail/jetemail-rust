use jetemail::{Attachment, CreateEmailOptions};

#[test]
fn serialize_minimal_email() {
    let email = CreateEmailOptions::new("from@example.com", "to@example.com", "Hello");

    let json = serde_json::to_value(&email).unwrap();
    assert_eq!(json["from"], "from@example.com");
    assert_eq!(json["to"], serde_json::json!(["to@example.com"]));
    assert_eq!(json["subject"], "Hello");

    // Optional fields should be absent
    assert!(json.get("html").is_none());
    assert!(json.get("text").is_none());
    assert!(json.get("cc").is_none());
    assert!(json.get("bcc").is_none());
    assert!(json.get("reply_to").is_none());
    assert!(json.get("headers").is_none());
    assert!(json.get("attachments").is_none());
}

#[test]
fn serialize_full_email() {
    let email = CreateEmailOptions::new("from@example.com", "to@example.com", "Hello")
        .with_html("<p>Hi</p>")
        .with_text("Hi")
        .with_cc("cc@example.com")
        .with_reply_to("reply@example.com")
        .with_header("X-Tag", "test");

    let json = serde_json::to_value(&email).unwrap();
    assert_eq!(json["html"], "<p>Hi</p>");
    assert_eq!(json["text"], "Hi");
    assert_eq!(json["cc"], serde_json::json!(["cc@example.com"]));
    assert_eq!(json["reply_to"], serde_json::json!(["reply@example.com"]));
    assert_eq!(json["headers"]["X-Tag"], "test");
}

#[test]
fn serialize_attachment_uses_data_key() {
    let attachment = Attachment::from_content(b"hello", "test.txt");
    let json = serde_json::to_value(&attachment).unwrap();

    assert_eq!(json["filename"], "test.txt");
    // Content should be serialized as "data" (not "content")
    assert!(json.get("data").is_some());
    assert!(json.get("content").is_none());
}

#[test]
fn attachment_base64_encoding() {
    let attachment = Attachment::from_content(b"Hello, World!", "test.txt");
    assert_eq!(attachment.content, "SGVsbG8sIFdvcmxkIQ==");
    assert_eq!(attachment.filename, "test.txt");
}

#[test]
fn serialize_batch_payload() {
    let emails = vec![
        CreateEmailOptions::new("from@example.com", "a@example.com", "Hello A"),
        CreateEmailOptions::new("from@example.com", "b@example.com", "Hello B"),
    ];

    let payload = serde_json::json!({ "emails": emails });
    let arr = payload["emails"].as_array().unwrap();
    assert_eq!(arr.len(), 2);
    assert_eq!(arr[0]["to"], serde_json::json!(["a@example.com"]));
    assert_eq!(arr[1]["to"], serde_json::json!(["b@example.com"]));
}

use jetemail::{Attachment, CreateEmailOptions};

#[test]
fn create_email_with_required_fields() {
    let email = CreateEmailOptions::new("from@example.com", "to@example.com", "Test Subject");

    assert_eq!(email.from, "from@example.com");
    assert_eq!(email.to, vec!["to@example.com"]);
    assert_eq!(email.subject, "Test Subject");
    assert!(email.html.is_none());
    assert!(email.text.is_none());
    assert!(email.cc.is_none());
    assert!(email.bcc.is_none());
    assert!(email.reply_to.is_none());
    assert!(email.headers.is_none());
    assert!(email.attachments.is_none());
}

#[test]
fn create_email_with_all_options() {
    let email = CreateEmailOptions::new("from@example.com", "to@example.com", "Subject")
        .with_html("<h1>Hello</h1>")
        .with_text("Hello")
        .with_cc("cc@example.com")
        .with_bcc("bcc@example.com")
        .with_reply_to("reply@example.com")
        .with_header("X-Custom", "value")
        .with_attachment(Attachment::from_content(b"data", "file.txt"));

    assert_eq!(email.html.unwrap(), "<h1>Hello</h1>");
    assert_eq!(email.text.unwrap(), "Hello");
    assert_eq!(email.cc.unwrap(), vec!["cc@example.com"]);
    assert_eq!(email.bcc.unwrap(), vec!["bcc@example.com"]);
    assert_eq!(email.reply_to.unwrap(), vec!["reply@example.com"]);

    let headers = email.headers.unwrap();
    assert_eq!(headers.get("X-Custom").unwrap(), "value");

    let attachments = email.attachments.unwrap();
    assert_eq!(attachments.len(), 1);
    assert_eq!(attachments[0].filename, "file.txt");
}

#[test]
fn multiple_recipients_via_vec() {
    let email = CreateEmailOptions::new(
        "from@example.com",
        vec!["a@example.com".into(), "b@example.com".into()],
        "Subject",
    );

    assert_eq!(email.to, vec!["a@example.com", "b@example.com"]);
}

#[test]
fn multiple_recipients_via_array() {
    let email = CreateEmailOptions::new(
        "from@example.com",
        ["a@example.com", "b@example.com"],
        "Subject",
    );

    assert_eq!(email.to, vec!["a@example.com", "b@example.com"]);
}

#[test]
fn single_recipient_via_string() {
    let email = CreateEmailOptions::new(
        "from@example.com",
        String::from("to@example.com"),
        "Subject",
    );

    assert_eq!(email.to, vec!["to@example.com"]);
}

#[test]
fn multiple_headers() {
    let email = CreateEmailOptions::new("from@example.com", "to@example.com", "Subject")
        .with_header("X-First", "one")
        .with_header("X-Second", "two");

    let headers = email.headers.unwrap();
    assert_eq!(headers.len(), 2);
    assert_eq!(headers.get("X-First").unwrap(), "one");
    assert_eq!(headers.get("X-Second").unwrap(), "two");
}

#[test]
fn multiple_attachments() {
    let email = CreateEmailOptions::new("from@example.com", "to@example.com", "Subject")
        .with_attachment(Attachment::from_content(b"one", "one.txt"))
        .with_attachment(Attachment::from_content(b"two", "two.txt"));

    let attachments = email.attachments.unwrap();
    assert_eq!(attachments.len(), 2);
    assert_eq!(attachments[0].filename, "one.txt");
    assert_eq!(attachments[1].filename, "two.txt");
}

#[test]
fn multiple_cc_bcc_reply_to() {
    let email = CreateEmailOptions::new("from@example.com", "to@example.com", "Subject")
        .with_cc(["cc1@example.com", "cc2@example.com"])
        .with_bcc(vec!["bcc1@example.com".into(), "bcc2@example.com".into()])
        .with_reply_to("reply@example.com");

    assert_eq!(
        email.cc.unwrap(),
        vec!["cc1@example.com", "cc2@example.com"]
    );
    assert_eq!(
        email.bcc.unwrap(),
        vec!["bcc1@example.com", "bcc2@example.com"]
    );
    assert_eq!(email.reply_to.unwrap(), vec!["reply@example.com"]);
}

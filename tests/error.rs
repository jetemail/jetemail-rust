use jetemail::Error;

#[test]
fn error_display_jetemail() {
    let err = Error::JetEmail {
        message: "Invalid API key".into(),
        status_code: 401,
        response: None,
    };
    assert_eq!(err.to_string(), "JetEmail API error (401): Invalid API key");
}

#[test]
fn error_display_parse() {
    let err = Error::Parse("unexpected token".into());
    assert_eq!(
        err.to_string(),
        "Failed to parse response: unexpected token"
    );
}

#[test]
fn deserialize_error_response_with_message() {
    let json = r#"{"message": "Not found", "status_code": 404}"#;
    let err: jetemail::ErrorResponse = serde_json::from_str(json).unwrap();
    assert_eq!(err.message, "Not found");
    assert_eq!(err.status_code, 404);
}

#[test]
fn deserialize_error_response_with_error_alias() {
    let json = r#"{"error": "Something went wrong"}"#;
    let err: jetemail::ErrorResponse = serde_json::from_str(json).unwrap();
    assert_eq!(err.message, "Something went wrong");
    assert_eq!(err.status_code, 0); // default
}

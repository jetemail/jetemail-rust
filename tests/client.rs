use jetemail::{ConfigBuilder, JetEmail};

#[test]
fn create_client_with_api_key() {
    let client = JetEmail::new("je_test_key");
    // Should not panic — client is constructed successfully
    let _ = format!("{:?}", client);
}

#[test]
fn create_client_with_config() {
    let config = ConfigBuilder::new("je_test_key")
        .base_url("https://staging-api.jetemail.com")
        .user_agent("test-app/1.0")
        .build();

    let client = JetEmail::with_config(config);
    let _ = format!("{:?}", client);
}

#[test]
#[should_panic(expected = "JETEMAIL_API_KEY")]
fn default_client_panics_without_env_var() {
    // Ensure the env var is not set
    std::env::remove_var("JETEMAIL_API_KEY");
    let _ = JetEmail::default();
}

#[test]
fn client_is_clone() {
    let client = JetEmail::new("je_test_key");
    let cloned = client.clone();
    let _ = format!("{:?}", cloned);
}

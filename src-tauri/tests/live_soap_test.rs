//! Live integration test against a real, running AzerothCore worldserver
//! with SOAP enabled. Not run as part of normal `cargo test`/CI since it
//! depends on external server state and real credentials -- run explicitly
//! with environment variables set, e.g.:
//!
//!   WOW_GM_TEST_HOST=127.0.0.1 WOW_GM_TEST_PORT=7879 \
//!   WOW_GM_TEST_USERNAME=admin WOW_GM_TEST_PASSWORD=your-password \
//!   cargo test --test live_soap_test -- --ignored

use std::env;

struct TestServer {
    host: String,
    port: u16,
    username: String,
    password: String,
}

fn test_server() -> TestServer {
    TestServer {
        host: env::var("WOW_GM_TEST_HOST").expect("set WOW_GM_TEST_HOST"),
        port: env::var("WOW_GM_TEST_PORT")
            .expect("set WOW_GM_TEST_PORT")
            .parse()
            .expect("WOW_GM_TEST_PORT must be a valid port number"),
        username: env::var("WOW_GM_TEST_USERNAME").expect("set WOW_GM_TEST_USERNAME"),
        password: env::var("WOW_GM_TEST_PASSWORD").expect("set WOW_GM_TEST_PASSWORD"),
    }
}

#[tokio::test]
#[ignore]
async fn server_info_returns_real_data() {
    let server = test_server();
    let result = wow_gm_console_lib::soap::execute_command(
        &server.host,
        server.port,
        &server.username,
        &server.password,
        "server info",
    )
    .await
    .expect("SOAP call should succeed against a live, SOAP-enabled server");

    assert!(result.contains("Connected players"), "unexpected response: {result}");
}

#[tokio::test]
#[ignore]
async fn bad_password_is_rejected() {
    let server = test_server();
    let err = wow_gm_console_lib::soap::execute_command(
        &server.host,
        server.port,
        &server.username,
        "definitely-not-the-password",
        "server info",
    )
    .await
    .expect_err("wrong password should be rejected");

    assert!(matches!(err, wow_gm_console_lib::soap::SoapError::AuthFailed));
}

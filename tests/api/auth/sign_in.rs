use jsonwebtoken::{decode, Validation};
use matoscout_api::handlers::{Claims, Keys, Tokens};
use serde_json::json;

use crate::helpers::TestApplication;

#[tokio::test]
async fn sign_in_with_valid_credentials_return_valid_tokens() {
    let app = TestApplication::spawn().await;

    let email = "test@domain.com";
    let password = "password";

    app.post("/auth/sign-up")
        .json(&json!({
        "email": email,
        "password": password
        }))
        .send()
        .await
        .expect("Failed to execute request.");

    let response = app
        .post("/auth/sign-in")
        .json(&json!({
            "email": email,
            "password": password
        }))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let tokens: Tokens = response
        .json()
        .await
        .expect("Valid sign-in didn't return pair of tokens");

    let secret = app.jwt_secret.expose_secret();

    let keys = Keys::new(secret);

    let validation = Validation::default();

    let access_token: Claims = decode(&tokens.access_token, &keys.decoding, &validation)
        .expect("Access token is invalid")
        .claims;

    let refresh_token: Claims = decode(&tokens.refresh_token, &keys.decoding, &validation)
        .expect("refresh token is invalid")
        .claims;

    assert_eq!(access_token.iat, refresh_token.iat);
}

#[tokio::test]
async fn sign_in_with_valid_credentials_return_200() {
    let app = TestApplication::spawn().await;

    let email = "test@domain.com";
    let password = "password";

    app.post("/auth/sign-up")
        .json(&json!({
        "email": email,
        "password": password
        }))
        .send()
        .await
        .expect("Failed to execute request.");

    let response = app
        .post("/auth/sign-in")
        .json(&json!({
            "email": email,
            "password": password
        }))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn sign_in_with_non_existing_email_return_401() {
    let app = TestApplication::spawn().await;

    let response = app
        .post("/auth/sign-in")
        .json(&json!({
            "email": "nonexisting@domain.com",
            "password": "password"
        }))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(401, response.status().as_u16());
}

#[tokio::test]
async fn sign_in_with_non_invalid_password_return_401() {
    let app = TestApplication::spawn().await;

    let email = "test@domain.com";

    app.post("/auth/sign-up")
        .json(&json!({
        "email": email,
        "password": "correct-password"
        }))
        .send()
        .await
        .expect("Failed to execute request.");

    let response = app
        .post("/auth/sign-in")
        .json(&json!({
            "email": email,
            "password": "wrong-password"
        }))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(401, response.status().as_u16());
}

#[tokio::test]
async fn sign_in_with_invalid_data_returns_422() {
    let app = TestApplication::spawn().await;

    let test_cases = vec![
        (json!({}), "missing data"),
        (json!({"password": "password"}), "missing email"),
        (json!({"email": "test@domain.com"}), "missing password"),
        (
            json!({"email": "testwrong@@gmail.com", "password": "password"}),
            "invalid email",
        ),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = app
            .post("/auth/sign-in")
            .json(&invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            422,
            response.status().as_u16(),
            "/auth/sign-in did not return 422 when body was {}",
            error_message
        );
    }
}

#[tokio::test]
#[cfg(feature = "simple-oauth")]
async fn test_client_credentials_grant() {
    use drupal_kit::simple_oauth::SimpleOauthGrant;
    use drupal_kit::Drupalkit;

    let mut server = mockito::Server::new_async().await;
    let client_id = "_client_id_";
    let client_secret = "_client_secret_";
    let scopes = vec!["some-scope".to_owned(), "other-scope".to_owned()];

    let mock = server.mock("POST", "/oauth/token")
        .with_status(200)
        .with_body(r#"{"token_type": "bearer", "expires_in": 3000, "access_token": "_access-token-value_"}"#)
        .match_body(r#"client_id=_client_id_&client_secret=_client_secret_&scopes=some-scope,other-scope"#)
        .create_async()
        .await;

    let url = server.url();

    let client = Drupalkit::new(&url, Some(&client_id));

    let res = client
        .request_token(SimpleOauthGrant::ClientCredentials {
            client_id: client_id.to_owned(),
            client_secret: client_secret.to_owned(),
            scopes,
        })
        .await;

    mock.assert();

    let res = res.expect("request token must not fail");

    assert_eq!(res.access_token, "_access-token-value_");
    assert_eq!(res.expires_in, 3000);
    assert_eq!(res.token_type, "bearer");
    assert_eq!(res.refresh_token, None);
}

#[tokio::test]
#[cfg(feature = "simple-oauth")]
async fn test_client_credentials_grant_no_scopes() {
    use drupal_kit::simple_oauth::SimpleOauthGrant;
    use drupal_kit::Drupalkit;

    let mut server = mockito::Server::new_async().await;
    let client_id = "_client_id_";
    let client_secret = "_client_secret_";
    let scopes = vec![];

    let mock = server.mock("POST", "/oauth/token")
        .with_status(200)
        .with_body(r#"{"token_type": "bearer", "expires_in": 3000, "access_token": "_access-token-value_"}"#)
        .match_body(r#"client_id=_client_id_&client_secret=_client_secret_"#)
        .create_async()
        .await;

    let url = server.url();

    let client = Drupalkit::new(&url, Some(&client_id));

    let res = client
        .request_token(SimpleOauthGrant::ClientCredentials {
            client_id: client_id.to_owned(),
            client_secret: client_secret.to_owned(),
            scopes,
        })
        .await;

    mock.assert();

    let res = res.expect("request token must not fail");

    assert_eq!(res.access_token, "_access-token-value_");
    assert_eq!(res.expires_in, 3000);
    assert_eq!(res.token_type, "bearer");
    assert_eq!(res.refresh_token, None);
}

#[tokio::test]
#[cfg(feature = "simple-oauth")]
async fn test_refresh_token_grant() {
    use drupal_kit::simple_oauth::SimpleOauthGrant;
    use drupal_kit::Drupalkit;

    let mut server = mockito::Server::new_async().await;
    let client_id = "_client_id_";
    let client_secret = "_client_secret_";
    let refresh_token = "_refresh-token_";
    let scopes = vec!["some-scope".to_owned(), "other-scope".to_owned()];

    let mock = server.mock("POST", "/oauth/token")
        .with_status(200)
        .with_body(r#"{"token_type": "bearer", "expires_in": 3000, "access_token": "_access-token-value_"}"#)
        .match_body(r#"client_id=_client_id_&client_secret=_client_secret_&refresh_token=_refresh-token_&scopes=some-scope,other-scope"#)
        .create_async()
        .await;

    let url = server.url();

    let client = Drupalkit::new(&url, Some(&client_id));

    let res = client
        .request_token(SimpleOauthGrant::RefreshToken {
            client_id: client_id.to_owned(),
            client_secret: client_secret.to_owned(),
            refresh_token: refresh_token.to_owned(),
            scopes,
        })
        .await;

    mock.assert_async().await;

    let res = res.expect("request token must not fail");

    assert_eq!(res.access_token, "_access-token-value_");
    assert_eq!(res.expires_in, 3000);
    assert_eq!(res.token_type, "bearer");
    assert_eq!(res.refresh_token, None);
}

#[tokio::test]
#[cfg(feature = "simple-oauth")]
async fn test_client_credentials_auth_strategy() {
    use std::sync::Arc;

    use drupal_kit::http_client::HttpClient;
    use drupal_kit::simple_oauth::{ClientCredentialsAuthStrategy, SimpleOauthGrant};
    use drupal_kit::Drupalkit;
    use http::Method;

    let mut server = mockito::Server::new_async().await;
    let client_id = "_client_id_";
    let client_secret = "_client_secret_";
    let refresh_token = "_refresh-token_";
    let scopes = vec!["some-scope".to_owned(), "other-scope".to_owned()];

    let token_mock = server.mock("POST", "/oauth/token")
        .with_status(200)
        .with_body(r#"{"token_type": "bearer", "expires_in": 3000, "access_token": "_access-token-value_"}"#)
        // One request is manually, one is from strategy.
        .expect(2)
        .create_async()
        .await;

    let mock = server
        .mock("GET", "/authenticated")
        .with_status(200)
        .match_header("Authorization", "Bearer _access-token-value_")
        .expect(2)
        .create_async()
        .await;

    let url = server.url();

    let mut client = Drupalkit::new(&url, Some(&client_id));

    let auth_strategy =
        ClientCredentialsAuthStrategy::new(client_id, client_secret, scopes.clone());
    client.set_auth_strategy(auth_strategy);

    // Check that no stack overflow occures when calling `request_token` manually.
    _ = client
        .request_token(SimpleOauthGrant::RefreshToken {
            client_id: client_id.to_owned(),
            client_secret: client_secret.to_owned(),
            refresh_token: refresh_token.to_owned(),
            scopes,
        })
        .await;

    // Make sure locking works appropriately.
    let (first, sec) = tokio::join!(
        client.request(Method::GET, "/authenticated", "", vec![]),
        client.request(Method::GET, "/authenticated", "", vec![]),
    );

    first.expect("request token must not fail");
    sec.expect("request token must not fail");

    token_mock.assert_async().await;
    mock.assert_async().await;
}

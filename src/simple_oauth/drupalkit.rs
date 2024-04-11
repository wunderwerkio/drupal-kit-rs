use std::collections::BTreeMap;

use http::Method;
use serde::Deserialize;

use crate::{http_client::*, Drupalkit};

use super::grant::SimpleOauthGrant;

#[derive(Debug, Clone, Deserialize)]
pub struct SimpleOauthTokenResponse {
    pub token_type: String,
    pub expires_in: u32,
    pub access_token: String,
    pub refresh_token: Option<String>,
}

impl Drupalkit {
    pub async fn request_token(
        &self,
        grant: SimpleOauthGrant,
    ) -> Result<SimpleOauthTokenResponse, ClientError> {
        let mut body_parts = BTreeMap::new();

        match grant {
            SimpleOauthGrant::ClientCredentials {
                client_id,
                client_secret,
                scopes,
            } => {
                body_parts.insert("client_id", client_id);
                body_parts.insert("client_secret", client_secret);
                body_parts.insert("scopes", scopes.join(","));
            }
            SimpleOauthGrant::RefreshToken {
                client_id,
                client_secret,
                refresh_token,
                scopes,
            } => {
                body_parts.insert("client_id", client_id);
                body_parts.insert("client_secret", client_secret);
                body_parts.insert("refresh_token", refresh_token);
                body_parts.insert("scopes", scopes.join(","));
            }
        };

        let mut kv_pairs = Vec::new();

        // Create query param pairs.
        for (key, val) in body_parts.iter() {
            if val.is_empty() {
                continue;
            }

            kv_pairs.push(format!("{}={}", key, val));
        }

        // Create full body for use as application/x-www-form-urlencoded.
        let body = kv_pairs.join("&");

        let res = self
            .request_json::<SimpleOauthTokenResponse>(
                Method::POST,
                "/oauth/token",
                body,
                // This MUST be an anonymous request, otherwise auth strategies would introduce
                // stack overflows by calling the `request_token` method itself leading to an
                // infinite loop!
                vec![HttpRequestOption::Anonymous],
            )
            .await?;

        Ok(res)
    }
}

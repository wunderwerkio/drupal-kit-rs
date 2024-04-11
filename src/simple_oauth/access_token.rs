use chrono::{DateTime, Duration, Utc};

use super::drupalkit::SimpleOauthTokenResponse;

#[derive(Debug, Clone)]
pub struct AccessToken {
    pub expires_at: DateTime<Utc>,
    pub value: String,
}

impl AccessToken {
    /// Checks whether the access token is expired.
    /// This includes a one minute leeway, meaning the token
    /// is considered expired even if still valid for one minute (at max).
    pub fn is_expired(&self) -> bool {
        let now = Utc::now();

        let delta = self.expires_at - now;

        delta.num_minutes() < 1
    }
}

impl From<SimpleOauthTokenResponse> for AccessToken {
    fn from(payload: SimpleOauthTokenResponse) -> Self {
        let expires_at = Utc::now() + Duration::seconds(payload.expires_in.into());

        Self {
            expires_at,
            value: payload.access_token,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expired() {
        let expires_at = Utc::now();

        let access_token = AccessToken {
            expires_at,
            value: "some-val".to_owned(),
        };

        assert!(access_token.is_expired());
    }

    #[test]
    fn test_not_expired() {
        let expires_at = Utc::now() + Duration::seconds(61);

        let access_token = AccessToken {
            expires_at,
            value: "some-val".to_owned(),
        };

        assert!(!access_token.is_expired());
    }

    #[test]
    fn test_leeway() {
        let expires_at = Utc::now() + Duration::minutes(1);

        let access_token = AccessToken {
            expires_at,
            value: "some-val".to_owned(),
        };

        assert!(access_token.is_expired());
    }

    #[test]
    fn test_from_token_response() {
        let value = "_super-secret-token_";
        let expected_expires = Utc::now() + Duration::seconds(300);

        let res = SimpleOauthTokenResponse {
            access_token: value.to_owned(),
            token_type: "bearer".to_owned(),
            expires_in: 300,
            refresh_token: None,
        };

        let access_token: AccessToken = res.into();

        assert_eq!(access_token.value, value);

        // Compare the timestamp (seconds), because comparing the dates directly
        // is not equal, due to nanosecond differences in execution.
        assert_eq!(
            access_token.expires_at.timestamp(),
            expected_expires.timestamp()
        );
    }
}

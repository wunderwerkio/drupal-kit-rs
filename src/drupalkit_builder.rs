use crate::Drupalkit;

#[derive(Default)]
pub struct DrupalkitBuilder {
    http_client_builder: reqwest::ClientBuilder,

    base_url: Option<String>,
    #[cfg(feature = "consumer")]
    client_id: Option<String>,
}

impl DrupalkitBuilder {
    pub fn new() -> Self {
        let http_client_builder = reqwest::Client::builder();

        Self {
            http_client_builder,
            ..Default::default()
        }
    }

    pub fn set_base_url(mut self, base_url: &str) -> Self {
        self.base_url = Some(base_url.to_owned());

        self
    }

    #[cfg(feature = "consumer")]
    pub fn set_client_id(mut self, client_id: &str) -> Self {
        self.client_id = Some(client_id.to_owned());

        self
    }

    /// Customize the internal `reqwest::Client` instance using the `reqwest::ClientBuilder`.
    ///
    /// # Example: Allow insecure SSL
    ///
    /// ```rust
    /// use std::env;
    /// use drupal_kit::Builder;
    ///
    /// let allow_insecure = env::var("ALLOW_INSECURE").map_or(false, |v| v == "1");
    ///
    /// let drupalkit = Builder::new()
    ///     .set_base_url("https://example.com")
    ///     .build_http_client(|http_client_builder|
    ///         http_client_builder.danger_accept_invalid_certs(allow_insecure)
    ///     )
    ///     .build();
    /// ```
    pub fn build_http_client<F>(mut self, f: F) -> Self
    where
        F: Fn(reqwest::ClientBuilder) -> reqwest::ClientBuilder,
    {
        self.http_client_builder = f(self.http_client_builder);

        self
    }

    pub fn build(self) -> Drupalkit {
        Drupalkit {
            http_client: self.http_client_builder.build().unwrap(),

            base_url: self.base_url.expect("base_url must be set for drupalkit"),
            #[cfg(feature = "consumer")]
            client_id: self.client_id,

            auth_strategy: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASE_URL: &str = "https://example.com";

    #[test]
    fn test_build_minimal() {
        let dk = DrupalkitBuilder::new().set_base_url(BASE_URL).build();

        assert_eq!(dk.base_url, BASE_URL);

        #[cfg(feature = "consumer")]
        assert_eq!(dk.client_id, None);
    }

    #[test]
    fn test_build_max() {
        #[cfg(feature = "consumer")]
        let client_id = "_client-id_";

        let builder = DrupalkitBuilder::new()
            .set_base_url(BASE_URL)
            .build_http_client(|http_client_builder| http_client_builder.no_gzip());

        #[cfg(feature = "consumer")]
        let builder = builder.set_client_id(client_id);

        let dk = builder.build();

        assert_eq!(dk.base_url, BASE_URL);

        #[cfg(feature = "consumer")]
        assert_eq!(dk.client_id, Some(client_id.to_owned()));
    }
}

use core::fmt::{self, Debug};
use std::{
    error::{self, Error},
    future::Future,
};

use http::{HeaderMap, HeaderName, HeaderValue};
use reqwest::{Body, Client, Method, RequestBuilder, Response};
use serde::de::DeserializeOwned;

/// Additional option for request.
#[derive(Clone)]
pub enum HttpRequestOption {
    /// Adds a HTTP header with name and value to the request.
    Header(HeaderName, HeaderValue),
    /// Overwrites the base url for the request.
    /// Takes precedence over the value from `get_base_url`.
    BaseUrl(String),
    /// Disables invocation of `before_request` and `after_request`
    /// for this request.
    NoBeforeAfter,
    /// Disables the use of a authentication strategy therefore
    /// making it an anonymous request.
    Anonymous,
}

/// Defines an error coming from the HttpClient.
pub type ClientError = Box<dyn Error + Send + Sync>;

#[derive(Debug)]
pub struct FailedRequestError {
    response: Response,
}

impl From<Response> for FailedRequestError {
    fn from(value: Response) -> Self {
        Self { response: value }
    }
}

impl fmt::Display for FailedRequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "http request failed with status code {}",
            self.response.status()
        )
    }
}

impl error::Error for FailedRequestError {}

/// Provides basic HTTP Client capabilities.
///
/// Implement this in your struct when building a custom
/// HTTP client.
///
/// The methods `before_request` and `after_request` can be used
/// to alter the request / response of the `request` method.
pub trait HttpClient {
    /// Make an HTTP request.
    ///
    /// The URL is constructed using the base url from `HttpRequestOption::BaseUrl`or `self.get_base_url()`.
    /// The given path is appended to the base url to produce the full request URL.
    ///
    ///
    fn request(
        &self,
        method: Method,
        path: &str,
        body: impl Into<Body> + Send,
        options: Vec<HttpRequestOption>,
    ) -> impl Future<Output = Result<Response, ClientError>> + Send
    where
        Self: Sync,
    {
        async move {
            let mut base_url: String = self.get_base_url().to_string();
            let mut header_map = HeaderMap::new();
            let mut no_before_after = false;

            // Handle additional request options.
            for option in &options {
                match option {
                    HttpRequestOption::Header(key, value) => {
                        header_map.insert(key.clone(), value.clone());
                    }
                    HttpRequestOption::BaseUrl(url) => {
                        base_url = url.clone();
                    }
                    HttpRequestOption::NoBeforeAfter => {
                        no_before_after = true;
                    }
                    _ => {}
                }
            }

            let url = format!("{}{}", base_url, path);
            let client = self.get_http_client();

            // Create a request builder and add modified headers.
            let req_builder = client.request(method, url).headers(header_map).body(body);

            // Allow alteration of request in impl.
            // Only on requests without the `HttpRequestOptio::NoBeforeAfter` option.
            // This is to prevent infinite loops if a `before_request` handler
            // if the handler itself calls another `request`.
            let req_builder = if !no_before_after {
                match self.before_request(req_builder, path, options).await {
                    Ok(req_builder) => req_builder,
                    Err(err) => return Err(err),
                }
            } else {
                req_builder
            };

            // Build the request.
            let req = req_builder.build()?;

            // Execute the request.
            match client.execute(req).await {
                Ok(response) => {
                    // Allow alteration of response in impl.
                    // Only on requests without the `HttpRequestOptio::NoBeforeAfter` option.
                    let response = if !no_before_after {
                        match self.after_request(response).await {
                            Ok(response) => response,
                            Err(err) => return Err(err),
                        }
                    } else {
                        response
                    };

                    Ok(response)
                }
                Err(err) => Err(Box::new(err)),
            }
        }
    }

    /// The same as `request` but deserializes json response body
    /// into a struct.
    fn request_json<T>(
        &self,
        method: Method,
        path: &str,
        body: impl Into<Body> + Send,
        options: Vec<HttpRequestOption>,
    ) -> impl Future<Output = Result<T, ClientError>> + Send
    where
        Self: Sync,
        T: DeserializeOwned + Debug,
    {
        async move {
            match self.request(method, path, body, options).await {
                Ok(response) => {
                    if response.status().is_success() {
                        let bytes = response.bytes().await?;
                        let payload = serde_json::from_slice(&bytes)?;

                        Ok(payload)
                    } else {
                        Err(Box::new(FailedRequestError::from(response))
                            as Box<dyn Error + Send + Sync>)
                    }
                }
                Err(err) => Err(err),
            }
        }
    }

    /// Modify the request before being sent.
    ///
    /// Using the given `reqwest::RequestBuilder` you can add
    /// headers and do other stuff to the request.
    fn before_request(
        &self,
        req_builder: RequestBuilder,
        _path: &str,
        _options: Vec<HttpRequestOption>,
    ) -> impl Future<Output = Result<RequestBuilder, ClientError>> + Send {
        async { Ok(req_builder) }
    }

    /// Modify a successful response.
    ///
    /// This method alters the `reqwest::Response` returned
    /// from the `request` method.
    fn after_request(
        &self,
        response: Response,
    ) -> impl Future<Output = Result<Response, ClientError>> + Send {
        async { Ok(response) }
    }

    /// Return the underlying instance of `reqwest::Client`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use drupal_kit::http_client::HttpClient;
    ///
    /// struct MyHttpClient {
    ///     http_client: reqwest::Client,
    /// }
    ///
    /// impl MyHttpClient {
    ///     pub fn new() -> Self {
    ///         let http_client = reqwest::Client::new();
    ///
    ///         Self {
    ///             http_client,
    ///         }
    ///     }
    /// }
    ///
    /// impl HttpClient for MyHttpClient {
    ///     fn get_http_client(&self) -> &reqwest::Client {
    ///         &self.http_client
    ///     }
    ///
    ///     fn get_base_url(&self) -> &str {
    ///         todo!()
    ///     }
    /// }
    /// ```
    fn get_http_client(&self) -> &Client;

    /// Returns the baseurl used for every request,
    /// unless explicitly set with `HttpRequestOption::BaseUrl`.
    fn get_base_url(&self) -> &str;
}

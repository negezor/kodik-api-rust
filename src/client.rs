use reqwest::{Client as ReqwestClient, RequestBuilder};

/// The top-level struct of the SDK, representing a client
#[derive(Debug, Clone)]
pub struct Client {
    api_key: String,
    api_url: String,
    http_client: ReqwestClient,
}

impl Client {
    /// Create a client
    ///
    /// # Example
    ///
    /// ```
    /// # use kodik_api::Client;
    ///
    /// let api_key = std::env::var("KODIK_API_KEY").expect("KODIK_API_KEY is not set");
    ///
    /// let client = Client::new(api_key);
    /// ```
    pub fn new(api_key: impl Into<String>) -> Client {
        Client {
            api_key: api_key.into(),
            // TODO: Add client builder for api url
            api_url: "https://kodikapi.com".to_owned(),
            http_client: ReqwestClient::builder()
                // TODO: Add client builder for proxy
                .build()
                .expect("failed to build reqwest client"),
        }
    }

    pub(crate) fn init_post_request(&self, path: &str) -> RequestBuilder {
        self.http_client
            .post(self.api_url.clone() + path)
            .query(&[("token", &self.api_key)])
    }
}

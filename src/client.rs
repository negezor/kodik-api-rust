use reqwest::{Client as ReqwestClient, RequestBuilder};

/// The top-level struct of the SDK, representing a client
#[derive(Debug, Clone)]
pub struct Client {
    api_key: String,
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
            http_client: ReqwestClient::builder()
                // TODO: Add client builder for proxy
                .build()
                .expect("failed to build reqwest client"),
        }
    }

    pub(crate) fn init_post_request(&self, url: &str) -> RequestBuilder {
        self.http_client
            .post(url)
            .query(&[("token", &self.api_key)])
    }
}

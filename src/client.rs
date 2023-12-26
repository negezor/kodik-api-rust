use reqwest::{
    Client as ReqwestClient, ClientBuilder as ReqwestClientBuilder, Proxy, RequestBuilder,
};

#[derive(Debug)]
pub struct ClientBuilder {
    api_key: Option<String>,
    api_url: String,
    reqwest_client_builder: ReqwestClientBuilder,
}

impl ClientBuilder {
    /// Constructs a new `ClientBuilder`
    pub fn new() -> ClientBuilder {
        ClientBuilder {
            api_key: None,
            api_url: "https://kodikapi.com".to_owned(),
            reqwest_client_builder: ReqwestClientBuilder::new(),
        }
    }

    /// API key (token) for Kodik API
    ///
    /// ```
    /// use kodik_api::ClientBuilder;
    ///
    /// ClientBuilder::new()
    ///   .api_key("q8p5vnf9crt7xfyzke4iwc6r5rvsurv7");
    /// ```
    pub fn api_key(mut self, api_key: impl Into<String>) -> ClientBuilder {
        self.api_key = Some(api_key.into());
        self
    }

    /// Base URL for Kodik API
    ///
    /// Default: `https://kodikapi.com`
    ///
    /// ```
    /// use kodik_api::ClientBuilder;
    ///
    /// ClientBuilder::new()
    ///   .api_url("https://koooooooooooooodik.com/api");
    /// ```
    pub fn api_url(mut self, api_url: impl Into<String>) -> ClientBuilder {
        self.api_url = api_url.into();
        self
    }

    /// ```
    /// use kodik_api::ClientBuilder;
    ///
    /// ClientBuilder::new()
    ///   .proxy(reqwest::Proxy::http("https://my.prox").unwrap());
    /// ```
    pub fn proxy(mut self, proxy: Proxy) -> ClientBuilder {
        self.reqwest_client_builder = self.reqwest_client_builder.proxy(proxy);
        self
    }

    /// ```
    /// use kodik_api::ClientBuilder;
    ///
    /// ClientBuilder::new()
    ///   .custom_reqwest_builder(reqwest::ClientBuilder::new());
    /// ```
    pub fn custom_reqwest_builder(mut self, builder: ReqwestClientBuilder) -> ClientBuilder {
        self.reqwest_client_builder = builder;
        self
    }

    // TODO: Add handle errors
    /// # Panic
    /// If api_key is not set and if it was not possible to build http client
    ///
    /// ```
    /// use kodik_api::ClientBuilder;
    ///
    /// ClientBuilder::new().api_key("q8p5vnf9crt7xfyzke4iwc6r5rvsurv7").build();
    /// ```
    pub fn build(self) -> Client {
        Client {
            api_key: self.api_key.expect("api key is required"),
            api_url: self.api_url,
            http_client: self
                .reqwest_client_builder
                .build()
                .expect("failed to build reqwest client"),
        }
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

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
        ClientBuilder::new().api_key(api_key).build()
    }

    pub(crate) fn init_post_request(&self, path_or_url: &str) -> RequestBuilder {
        if !path_or_url.starts_with("http") {
            self.http_client
                .post(self.api_url.clone() + path_or_url)
                .query(&[("token", &self.api_key)])
        } else {
            self.http_client.post(path_or_url.to_owned())
        }
    }
}

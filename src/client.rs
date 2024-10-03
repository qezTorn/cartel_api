use reqwest::{
    Client as ReqwestClient, ClientBuilder as ReqwestClientBuilder, Proxy, RequestBuilder,
};

#[derive(Debug)]
pub struct ClientBuilder {
    api_key: Option<String>,
    api_url: String,
    reqwest_client_builder: ReqwestClientBuilder,
}

impl ClientBuilder{
    pub fn new() -> ClientBuilder{
        ClientBuilder{
            api_key: None,
            api_url: "https://cartelempire.online/api/".to_owned(),
            reqwest_client_builder: ReqwestClientBuilder::new(),
        }
    }

    pub fn api_key(mut self, api_key: impl Into<String>) -> ClientBuilder{
        self.api_key = Some(api_key.into());
        self
    }

    pub fn api_url(mut self, api_url: impl Into<String>) -> ClientBuilder{
        self.api_url = api_url.into();
        self
    }

    pub fn proxy(mut self, proxy: Proxy) -> ClientBuilder{
        self.reqwest_client_builder = self.reqwest_client_builder.proxy(proxy);
        self
    }

    pub fn custom_requwest_builder(mut self, builder: ReqwestClientBuilder) -> ClientBuilder{
        self.reqwest_client_builder = builder;
        self
    }

    pub fn build(self) -> Client {
        Client{
            api_key: self.api_key.expect("API key required"),
            api_url: self.api_url,
            http_client: self
                .reqwest_client_builder
                .build()
                .expect("Failed to build reqwest client"),
        }
    }
}

impl Default for ClientBuilder{
    fn default() -> Self {
         Self::new()
    }
}

#[derive(Debug)]
pub struct Client{
    pub api_key: String, 
    pub api_url: String,
    pub http_client: ReqwestClient,
}

impl Client{
    pub fn new(api_key: impl Into<String>) -> Client{
        ClientBuilder::new().api_key(api_key).build()
    }
}

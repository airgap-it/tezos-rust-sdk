use serde::{de::DeserializeOwned, Serialize};

use crate::error::Error;

pub struct TezosHttp {
    rpc_endpoint: String,
    client: reqwest::Client,
}

impl TezosHttp {
    /// Creates an Http client that will be used to send requests to the specified node.
    pub fn new(rpc_endpoint: &str) -> Self {
        TezosHttp {
            rpc_endpoint: rpc_endpoint.to_string(),
            client: reqwest::Client::new(),
        }
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.rpc_endpoint, path)
    }

    pub async fn get<T: DeserializeOwned>(&self, url: &str) -> Result<T, Error> {
        Ok(self.client.get(self.url(url)).send().await?.json::<T>().await?)
    }

    pub async fn post<B: Serialize, T: DeserializeOwned>(&self, url: &String, body: &B) -> Result<T, Error> {
        Ok(self.client
            .post(self.url(url))
            .json(body)
            .send()
            .await?
            .json::<T>()
            .await?)
    }
}

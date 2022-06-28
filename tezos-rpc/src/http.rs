use {
    reqwest::Response,
    serde::{de::DeserializeOwned, Serialize},
    crate::error::{self, Error}
};

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

    async fn handle_response<T: DeserializeOwned>(&self, response: Response) -> Result<T, Error> {
        if response.status() != 200 {
            // Do not parse JSON when the content type is `plain/text`
            if response.headers()["content-type"] == "application/json" {
                return Err(error::Error::RPCErrors(response.json().await?));
            }
            return Err(error::Error::RPCErrorPlain(response.text().await?));
        }

        Ok(response.json().await?)
    }

    pub fn change_rpc_endpoint(&mut self, rpc_endpoint: String) {
        self.rpc_endpoint = rpc_endpoint;
    }

    /// Convenience method to make a `GET` request to a URL.
    pub async fn get<T: DeserializeOwned>(&self, url: &str) -> Result<T, Error> {
        self.handle_response(self.client.get(self.url(url)).send().await?)
            .await
    }

    /// Convenience method to make a `GET` request with query parameters to a URL.
    pub async fn get_with_query<T: DeserializeOwned, Q: Serialize + ?Sized>(
        &self,
        url: &str,
        query: &Q,
    ) -> Result<T, Error> {
        self.handle_response(self.client.get(self.url(url)).query(query).send().await?)
            .await
    }

    /// Convenience method to make a `POST` request to a URL.
    pub async fn post<B: Serialize, T: DeserializeOwned, Q: Serialize>(
        &self,
        url: &str,
        body: &B,
        query: &Option<Q>,
    ) -> Result<T, Error> {
        self.handle_response(
            self.client
                .post(self.url(url))
                .query(query)
                .json(body)
                .send()
                .await?,
        )
        .await
    }

    /// Convenience method to make a `PATCH` request to a URL.
    pub async fn patch<B: Serialize, T: DeserializeOwned>(
        &self,
        url: &str,
        body: &Option<B>,
    ) -> Result<T, Error> {
        let mut req = self.client.patch(self.url(url));

        if let Some(json) = body {
            req = req.json(json);
        }

        self.handle_response(req.send().await?).await
    }

    /// Convenience method to make a `DELETE` request to a URL.
    pub async fn delete<B: Serialize, T: DeserializeOwned>(
        &self,
        url: &str,
        body: &Option<B>,
    ) -> Result<T, Error> {
        let mut req = self.client.delete(self.url(url));

        if let Some(json) = body {
            req = req.json(json);
        }

        self.handle_response(req.send().await?).await
    }
}

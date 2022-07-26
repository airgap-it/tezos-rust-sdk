use {
    crate::error::Error,
    async_trait::async_trait,
    serde::{de::DeserializeOwned, Serialize},
};

#[async_trait]
pub trait Http {
    fn new(rpc_endpoint: String) -> Self;

    fn change_rpc_endpoint(&mut self, rpc_endpoint: String);

    async fn get<T: DeserializeOwned>(&self, url: &str) -> Result<T, Error>;

    async fn get_with_query<T: DeserializeOwned, Q: Serialize + ?Sized + Sync>(
        &self,
        url: &str,
        query: &Q,
    ) -> Result<T, Error>;

    async fn post<B: Serialize + Sync, T: DeserializeOwned, Q: Serialize + Sync>(
        &self,
        url: &str,
        body: &B,
        query: Option<&Q>,
    ) -> Result<T, Error>;

    async fn patch<B: Serialize + Sync, T: DeserializeOwned>(
        &self,
        url: &str,
        body: Option<&B>,
    ) -> Result<T, Error>;

    async fn delete<B: Serialize + Sync, T: DeserializeOwned>(
        &self,
        url: &str,
        body: Option<&B>,
    ) -> Result<T, Error>;
}

#[cfg(feature = "http")]
pub mod default {
    use crate::models::error::RpcError;

    use super::*;
    use reqwest::{Client, Response};

    #[derive(Debug)]
    pub struct HttpClient {
        rpc_endpoint: String,
        client: Client,
    }

    impl HttpClient {
        fn url(&self, path: &str) -> String {
            format!("{}{}", self.rpc_endpoint, path)
        }

        async fn handle_response<T: DeserializeOwned>(
            &self,
            response: Response,
        ) -> Result<T, Error> {
            if response.status() != 200 {
                // Do not parse JSON when the content type is `plain/text`
                if response.headers()["content-type"] == "application/json" {
                    let errors: Vec<RpcError> = response.json().await?;
                    return Err(Error::RpcErrors(errors.into()));
                }
                return Err(Error::RpcErrorPlain {
                    description: response.text().await?,
                });
            }

            Ok(response.json().await?)
        }
    }

    #[cfg(feature = "http")]
    #[async_trait]
    impl Http for HttpClient {
        /// Creates an Http client that will be used to send requests to the specified node.
        fn new(rpc_endpoint: String) -> Self {
            Self {
                rpc_endpoint,
                client: Client::new(),
            }
        }

        fn change_rpc_endpoint(&mut self, rpc_endpoint: String) {
            self.rpc_endpoint = rpc_endpoint;
        }

        /// Convenience method to make a `GET` request to a URL.
        async fn get<T: DeserializeOwned>(&self, url: &str) -> Result<T, Error> {
            self.handle_response(self.client.get(self.url(url)).send().await?)
                .await
        }

        /// Convenience method to make a `GET` request with query parameters to a URL.
        async fn get_with_query<T: DeserializeOwned, Q: Serialize + ?Sized + Sync>(
            &self,
            url: &str,
            query: &Q,
        ) -> Result<T, Error> {
            self.handle_response(self.client.get(self.url(url)).query(query).send().await?)
                .await
        }

        /// Convenience method to make a `POST` request to a URL.
        async fn post<B: Serialize + Sync, T: DeserializeOwned, Q: Serialize + Sync>(
            &self,
            url: &str,
            body: &B,
            query: Option<&Q>,
        ) -> Result<T, Error> {
            self.handle_response(
                self.client
                    .post(self.url(url))
                    .query(&query)
                    .json(body)
                    .send()
                    .await?,
            )
            .await
        }

        /// Convenience method to make a `PATCH` request to a URL.
        async fn patch<B: Serialize + Sync, T: DeserializeOwned>(
            &self,
            url: &str,
            body: Option<&B>,
        ) -> Result<T, Error> {
            let mut req = self.client.patch(self.url(url));

            if let Some(json) = body {
                req = req.json(json);
            }

            self.handle_response(req.send().await?).await
        }

        /// Convenience method to make a `DELETE` request to a URL.
        async fn delete<B: Serialize + Sync, T: DeserializeOwned>(
            &self,
            url: &str,
            body: Option<&B>,
        ) -> Result<T, Error> {
            let mut req = self.client.delete(self.url(url));

            if let Some(json) = body {
                req = req.json(json);
            }

            self.handle_response(req.send().await?).await
        }
    }
}

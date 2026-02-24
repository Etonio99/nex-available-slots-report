use reqwest::{Client, Method};
use serde::{Serialize, de::DeserializeOwned};

use super::{types::NexApiResponse, error::ApiError};

pub struct NexApiClient {
    client: Client,
    token: Option<String>,
    base_url: String,
}

impl NexApiClient {
    pub fn new(token: Option<String>) -> Self {
        Self {
            client: Client::new(),
            token,
            base_url: "https://nexhealth.info/".into(),
        }
    }

    pub async fn request<T, B>(
        &self,
        path: &str,
        method: Method,
        body: Option<&B>,
        use_beta: bool,
    ) -> Result<NexApiResponse<T>, ApiError>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        let url = format!("{}{}", self.base_url, path);

        let mut req = self.client.request(method, &url);

        if let Some(token) = &self.token {
            req = req.header("Authorization", token);
        }

        req = req.header("Content-Type", "application/json");

        if use_beta {
            req = req.header("Nex-Api-Version", "v20240412");
        } else {
            req = req.header("Nex-Api-Version", "v2");
        }

        if let Some(body) = body {
            req = req.json(body);
        }

        let response = req.send().await?;

        if !response.status().is_success() {
            let text = response.text().await.unwrap_or_default();
            return Err(anyhow!("API call failed: {}", text));
        }

        let parsed = response.json::<NexApiResponse<T>>().await?;

        Ok(parsed)
    }
}
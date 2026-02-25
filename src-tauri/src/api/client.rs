use super::types::NexApiResponse;
use reqwest::{Client, Method};
use serde::{de::DeserializeOwned, Serialize};

use serde_urlencoded;

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

    pub async fn request<T, B, Q>(
        &self,
        path: &str,
        method: Method,
        body: Option<&B>,
        query: Option<&Q>,
        use_beta: bool,
    ) -> Result<NexApiResponse<T>, Box<dyn std::error::Error>>
    where
        T: DeserializeOwned,
        B: Serialize,
        Q: Serialize,
    {
        // let url = format!("{}{}", self.base_url, path);

        // let mut req = self.client.request(method, &url);

        // if let Some(token) = &self.token {
        //     // req = req.header("Authorization", token);

        //     println!("Token bytes: {:?}", token.as_bytes());

        //     // 2. Deep Clean: Remove ALL whitespace, newlines, and carriage returns
        //     let auth_header: String = token.chars().filter(|c| !c.is_whitespace()).collect();

        //     // 3. Manual Build Check
        //     match reqwest::header::HeaderValue::from_str(&auth_header) {
        //         Ok(v) => req = req.header("Authorization", v),
        //         Err(_) => {
        //             return Err(format!(
        //                 "CRITICAL: Token contains illegal characters. Hex: {:02X?}",
        //                 auth_header.as_bytes()
        //             )
        //             .into());
        //         }
        //     }
        // }

        // req = req.header("Content-Type", "application/json");

        // if use_beta {
        //     req = req.header("Nex-Api-Version", "v20240412");
        // } else {
        //     req = req.header("Nex-Api-Version", "v2");
        // }

        // if let Some(body) = body {
        //     req = req.json(body);
        // }

        // if let Some(query) = query {
        //     req = req.query(query);
        // }

        // if let Some(query) = query {
        //     req = req.query(query);
        // }

        // println!("Sending Query");
        // println!("{}", url);
        // // let response = req.send().await?;

        // let response = req.send().await.map_err(|e| {
        //     // If it still fails here, the issue is likely the URL structure or Query params
        //     format!("Reqwest Send Error: {:?} (Kind: {:?})", e, e.is_builder())
        // })?;

        // let response = match req.send().await {
        //     Ok(resp) => {
        //         println!("Request sent successfully, status: {}", resp.status());
        //         resp
        //     }
        //     Err(e) => {
        //         println!("Error sending request: {:?}", e);
        //         return Err(format!("Request failed: {}", e).into());
        //     }
        // };

        // println!("Response Received");

        // if !response.status().is_success() {
        //     let text = response.text().await.unwrap_or_default();
        //     println!("{}", text);
        //     return Err(Box::<dyn std::error::Error>::from(format!(
        //         "API call failed: {}",
        //         text
        //     )));
        // }

        // let parsed = response.json::<NexApiResponse<T>>().await?;

        // Ok(parsed)

        let url = format!("{}{}", self.base_url.trim(), path.trim());
        println!("1. URL formed: {}", url);

        // Create the builder
        let mut req = self.client.request(method, &url);
        println!("2. Builder created");

        // TEST 1: The Token
        if let Some(token) = &self.token {
            let clean_token: String = token.trim().to_string();
            // Standard format is "Bearer <token>"
            let auth_value = format!("Bearer {}", clean_token);

            println!("3. Adding Auth Header: {}", auth_value);
            req = req.header("Authorization", &auth_value);
        }

        // TEST 2: Content Type
        println!("4. Adding Content-Type");
        req = req.header("Content-Type", "application/json");

        // TEST 3: Version
        let ver = if use_beta { "v20240412" } else { "v2" };
        println!("5. Adding Version: {}", ver);
        req = req.header("Nex-Api-Version", ver);

        // TEST 4: Query (The most likely failure point for "unsupported value")
        if let Some(q) = query {
            // Use a simpler serialization check
            let query_string = serde_json::to_string(q)?;
            println!("JSON version of query: {}", query_string);

            let query_string = serde_urlencoded::to_string(q).map_err(|e| {
                format!("SERDE ERROR: {}", e) // This will tell us the EXACT reason
            })?;
        }

        // TEST 5: JSON Body
        if let Some(b) = body {
            println!("7. Adding Body");
            req = req.json(b);
        }

        println!("8. Attempting to .build()");
        let final_req = req.build()?;
        println!("9. Build SUCCESS! URL: {}", final_req.url());

        let response = self.client.execute(final_req).await?;
        println!("10. Execute SUCCESS! Status: {}", response.status());

        // ... parsing ...
        let parsed = response.json::<NexApiResponse<T>>().await?;
        Ok(parsed)
    }
}

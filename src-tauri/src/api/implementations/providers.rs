use reqwest::Method;

use crate::{
    LocationsQuery,
    NexApiClient,
    NexApiResponse,
    api::types::providers::{
        Provider,
        ProvidersQuery
    }
};

impl NexApiClient {
    pub async fn get_providers(
        &self,
        query: ProvidersQuery,
    ) -> Result<NexApiResponse<Vec<Provider>>, String> {
        let response = self
            .request::<Vec<Provider>, (), ProvidersQuery>(
                "providers",
                Method::GET,
                None,
                Some(&query),
                false,
            )
            .await
            .map_err(|e| e.to_string())?;

        Ok(response)
    }
}

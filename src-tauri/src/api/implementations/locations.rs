use reqwest::Method;

use crate::{
    api::types::locations::LocationsResponse,
    LocationsQuery,
    NexApiClient,
    NexApiResponse,
};

impl NexApiClient {
    pub async fn get_locations(
        &self,
        query: LocationsQuery,
    ) -> Result<NexApiResponse<Vec<LocationsResponse>>, String> {
        let response = self
            .request::<Vec<LocationsResponse>, (), LocationsQuery>(
                "locations",
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

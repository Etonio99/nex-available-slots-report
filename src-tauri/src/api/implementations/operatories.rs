use reqwest::Method;

use crate::{
    NexApiClient,
    NexApiResponse,
    api::types::operatories::{
        Operatory,
        OperatoriesQuery
    }
};

impl NexApiClient {
    pub async fn get_operatories(
        &self,
        query: OperatoriesQuery,
    ) -> Result<NexApiResponse<Vec<Operatory>>, String> {
        let response = self
            .request::<Vec<Operatory>, (), OperatoriesQuery>(
                "operatories",
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

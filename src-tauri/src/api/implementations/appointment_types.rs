use reqwest::Method;

use crate::{
    NexApiClient, NexApiResponse, api::types::appointment_types::{AppointmentType, AppointmentTypesQuery}
};

impl NexApiClient {
    pub async fn get_appointment_types(
        &self,
        subdomain: String,
        location_id: u32,
    ) -> Result<NexApiResponse<Vec<AppointmentType>>, String> {
        let query = AppointmentTypesQuery {
            subdomain,
            location_id,
        };

        let response = self
            .request::<Vec<AppointmentType>, (), AppointmentTypesQuery>(
                "appointment_types",
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

use chrono::NaiveDate;
use reqwest::Method;

use crate::{
    api::types::appointment_slots::AppointmentSlotsQuery,
    AppointmentSlotsResponse,
    NexApiClient,
    NexApiResponse,
    ProviderLocationMap,
};

impl NexApiClient {
    pub async fn get_appointment_slots(
        &self,
        subdomain: String,
        start_date: NaiveDate,
        days: u32,
        appointment_type_id: u32,
        provider_location_map: ProviderLocationMap,
    ) -> Result<NexApiResponse<Vec<AppointmentSlotsResponse>>, String> {
        let query = AppointmentSlotsQuery {
            subdomain,
            start_date,
            days,
            appointment_type_id,
            location_id: provider_location_map.location_id,
            provider_ids: provider_location_map.provider_ids,
        };

        let response = self
            .request::<Vec<AppointmentSlotsResponse>, (), AppointmentSlotsQuery>(
                "appointment_slots",
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

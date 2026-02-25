use reqwest::Method;

use super::{
    client::NexApiClient,
    types::Appointment,
    error::ApiError,
};

impl NexApiClient {
    pub async fn get_appointment_slots(
        &self,
        start_date: NaiveDate,
        days: u32,
        provider_location_map: ProviderLocationMap,
    ) -> Result<Vec<Appointment>, ApiError> {
        // Example: https://nexhealth.info/appointment_slots?subdomain=test&start_date=2026-02-23&days=7&lids[]=67890&pids[]=12345&slot_length=30&overlapping_operatory_slots=false

        let query = AppointmentSlotsQuery {
            start_date,
            days,
            provider_ids: provider_location_map.provider_ids.clone(),
        };

        let response = self
            .request::<Vec<Appointment>, ()>(
                "appointment_slots",
                Method::GET,
                None,
                false,
            )
            .query(&query)
            .await?;

        Ok(response.data.unwrap_or_default())
    }
}
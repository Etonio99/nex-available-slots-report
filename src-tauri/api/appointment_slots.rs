use reqwest::Method;

use super::{
    client::NexApiClient,
    types::Appointment,
    error::ApiError,
};

impl NexApiClient {
    pub async fn get_appointment_slots(
        &self,
        start_date: DateTime<Utc>,
        days: u32,
        provider_location_maps: Vec<ProviderLocationMap>,
    ) -> Result<Vec<Appointment>, ApiError> {
        let path = format!("appointment_slots?start_date={}&days={}", start_date, days);
        // Example: https://nexhealth.info/appointment_slots?subdomain=test&start_date=2026-02-23&days=7&lids[]=67890&pids[]=12345&slot_length=30&overlapping_operatory_slots=false

        let response = self
            .request::<Vec<Appointment>, ()>(
                &path,
                Method::GET,
                None,
                false,
            )
            .await?;

        Ok(response.data.unwrap_or_default())
    }
}
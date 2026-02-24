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
        for map in provider_location_maps.iter() {
            let provider_ids_strings: Vec<String> = map.provider_ids.iter().map(|&id| format("provider_id[]={}", id)).collect::<Vec<_>>().join("&");
            let provider_include_string = format("provider_id[]={}", provider_ids_strings.join("provider_id[]="));
    
            let path = format!("appointment_slots?{}", start_date, days, provider_include_string);
            // Example: https://nexhealth.info/appointment_slots?subdomain=test&start_date=2026-02-23&days=7&lids[]=67890&pids[]=12345&slot_length=30&overlapping_operatory_slots=false
    
            let response = self
                .request::<Vec<Appointment>, ()>(
                    &path,
                    Method::GET,
                    None,
                    false,
                )
                .query(&[
                    ("start_date", start_date.to_string()),
                    ("days", days.to_string())
                ])
                .await?;
        }

        Ok(response.data.unwrap_or_default())
    }
}
use chrono::NaiveDate;
use reqwest::Method;

use crate::{
    AppointmentSlots,
    NexApiClient,
    NexApiResponse,
    ProviderLocationMap
};

impl NexApiClient {
    pub async fn get_appointment_slots(
        &self,
        subdomain: String,
        start_date: NaiveDate,
        days: u32,
        appointment_type_id: u32,
        provider_location_map: ProviderLocationMap,
    ) -> Result<NexApiResponse<Vec<AppointmentSlots>>, String> {
        // Example: https://nexhealth.info/appointment_slots?subdomain=test&start_date=2026-02-23&days=7&lids[]=67890&pids[]=12345&slot_length=30&overlapping_operatory_slots=false

        let mut params = vec![
            ("subdomain".to_string(), subdomain),
            ("start_date".to_string(), start_date.format("%Y-%m-%d").to_string()),
            ("days".to_string(), days.to_string()),
            ("appointment_type_id".to_string(), appointment_type_id.to_string()),
            ("lids[]".to_string(), provider_location_map.location_id.to_string()),
        ];

        for pid in provider_location_map.provider_ids {
            params.push(("pids[]".to_string(), pid.to_string()));
        }

        let response = self
            .request::<Vec<AppointmentSlots>, (), Vec<(String, String)>>(
                "appointment_slots",
                Method::GET,
                None,
                Some(&params),
                false,
            )
            .await
            .map_err(|e| e.to_string())?;

        Ok(response)
    }
}

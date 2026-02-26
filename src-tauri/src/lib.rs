mod api;

use api::{
    NexApiClient,
    types::{
        AppointmentSlots,
        NexApiResponse,
        ProviderLocationMap,
    }
};
use chrono::NaiveDate;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_appointment_slots])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn get_appointment_slots() -> Result<NexApiResponse<Vec<AppointmentSlots>>, String> {
    let client = NexApiClient::new(Some("eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIxMjM1Iiwic2NwIjoiYXBpX3VzZXIiLCJpYXQiOjE3NzIwNzA2NDIsImV4cCI6MTc3MjA3NDI0MiwianRpIjoiZTZlZmRlNzgtNjZlYi00Y2RjLTg5NTEtMmZiZTdkZDFmNmVlIn0.JZMFefF7v4VNF0SSCZVkSFJnYrP_734XuKrUz-rjRWg".into()));

    let start_date = NaiveDate::from_ymd_opt(2026, 2, 23).ok_or("Invalid start date")?;

    let provider_location_map = ProviderLocationMap {
        location_id: 77812,
        provider_ids: vec![198875751],
    };

    let result = client
        .get_appointment_slots(
            "MB2".to_string(),
            start_date,
            7,
            183404,
            provider_location_map,
        )
        .await
        .map_err(|e| e.to_string())?;

    println!("Result:");
    println!("{:?}", result);

    Ok(result)
}
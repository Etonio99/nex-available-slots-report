mod api;
mod utils;
use std::sync::Mutex;

use chrono::NaiveDate;
use crate::{
    api::{
        NexApiClient,
        types::{
            appointment_slots::{
                AppointmentSlotsResponse,
                ProviderLocationMap
            },
            locations::{
                LocationsQuery,
                LocationsResponse
            },
            nex_api::NexApiResponse
        }
    },
    utils::AppData,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let client = NexApiClient::new(Some("eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIxMjM1Iiwic2NwIjoiYXBpX3VzZXIiLCJpYXQiOjE3NzIxNDM1NjIsImV4cCI6MTc3MjE0NzE2MiwianRpIjoiYzg2MzkwOTQtOWFlNi00ZWNkLThiMzEtMmEzYmZhYWQwOTljIn0.6jA-N7OMIcjGwxbqfoxcWQw_gPRaxBrLOdKl94L_z80".into()));
    let app_data = Mutex::new(AppData {
        location_ids: vec![],
        excluded_location_ids: vec![],
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(client)
        .manage(app_data)
        .invoke_handler(tauri::generate_handler![
            get_appointment_slots,
            get_locations,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn get_appointment_slots(
    client: tauri::State<'_, NexApiClient>,
) -> Result<NexApiResponse<Vec<AppointmentSlotsResponse>>, String> {
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

#[tauri::command]
async fn get_locations(
    client: tauri::State<'_, NexApiClient>,
) -> Result<NexApiResponse<Vec<LocationsResponse>>, String> {
    let query = LocationsQuery {
        subdomain: "ebreiny-demo-practice".to_string(),
        inactive: false,
    };

    let result = client
        .get_locations(query)
        .await
        .map_err(|e| e.to_string())?;

    println!("Locations Result:");
    println!("{:?}", result);

    Ok(result)
}
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
            nex_api::NexApiResponse, providers::{Provider, ProvidersQuery}
        }
    },
    utils::AppData,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let client = NexApiClient::new(Some("eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIxMjM1Iiwic2NwIjoiYXBpX3VzZXIiLCJpYXQiOjE3NzIyMzI0NDUsImV4cCI6MTc3MjIzNjA0NSwianRpIjoiZjRmMmE5NzktMTNlOC00MmM1LWEzMzktZDUxNjMyMTQ2YWM2In0.J-9xExvtibDBopDIHGUoMxv91MzsbIhgWpg41Krm6Tk".into()));
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
            test,
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
async fn test(
    client: tauri::State<'_, NexApiClient>,
) -> Result<NexApiResponse<Vec<Provider>>, String> {
    let query = ProvidersQuery {
        subdomain: "ebreiny-demo-practice".to_string(),
        location_id: 328347,
        inactive: false,
        per_page: 300,
    };

    let result = client
        .get_providers(query)
        .await
        .map_err(|e| e.to_string())?;

    println!("Locations Result:");
    println!("{:?}", result);

    Ok(result)
}
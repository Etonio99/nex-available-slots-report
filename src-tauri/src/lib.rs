mod api;
mod utils;
use std::sync::Mutex;

use chrono::NaiveDate;
use crate::{
    api::{
        NexApiClient,
        key::{
            get_api_key, save_api_key
        },
        types::{
            appointment_slots::{
                AppointmentSlotsQuery, AppointmentSlotsResponse, ProviderLocationMap
            }, appointment_types::{AppointmentType, AppointmentTypesQuery}, locations::{
                LocationsQuery,
                InstitutionLocations
            }, nex_api::NexApiResponse, operatories::{OperatoriesQuery, Operatory}, providers::{Provider, ProvidersQuery}
        }
    },
    utils::AppData,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let client = NexApiClient::new(Some("eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIxMjM1Iiwic2NwIjoiYXBpX3VzZXIiLCJpYXQiOjE3NzIyNTI0ODUsImV4cCI6MTc3MjI1NjA4NSwianRpIjoiM2IzMGUwM2YtMDhhNy00YmRjLWExYTAtMTQ0ZGE0ODIwYjE3In0.9-IAk1LNioY5CSuvrvB0c4VdNkE6egnPP_qeiYKNSkY".into()));
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
            save_api_key,
            get_api_key,
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

    let appointment_slots_query = AppointmentSlotsQuery {
        subdomain: "MB2".to_string(),
        start_date,
        days: 7,
        appointment_type_id: 1,
        location_id: provider_location_map.location_id,
        provider_ids: provider_location_map.provider_ids.clone(),
    };

    let result = client
        .get_appointment_slots(appointment_slots_query)
        .await
        .map_err(|e| e.to_string())?;

    println!("Result:");
    println!("{:?}", result);

    Ok(result)
}

#[tauri::command]
async fn test(
    client: tauri::State<'_, NexApiClient>,
) -> Result<NexApiResponse<Vec<InstitutionLocations>>, String> {
    let query = LocationsQuery {
        subdomain: "ebreiny-demo-practice".to_string(),
        // location_id: 328347,
        inactive: false,
        // per_page: 300,
    };

    let result = client
        .get_locations(query)
        .await
        .map_err(|e| e.to_string())?;

    println!("Locations Result:");
    println!("{:?}", result);

    Ok(result)
}
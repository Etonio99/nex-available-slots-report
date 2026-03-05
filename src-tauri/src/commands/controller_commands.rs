use crate::{
    api::NexApiClient,
    services::{
        controller::Controller,
        processors::{
            appointment_slots_processor::AppointmentSlotsProcessor,
            types::processor_advance_result::ProcessorAdvanceResult,
        },
    },
};

#[tauri::command]
pub fn set_processor(
    controller: tauri::State<'_, Controller>,
    processor_name: String,
) -> Result<(), String> {
    let mut guard = controller.processor.lock().unwrap();

    match processor_name.as_str() {
        "appointment_slots" => {
            *guard = Some(Box::new(AppointmentSlotsProcessor::new()));
        }
        _ => return Err("Unknown processor name".into()),
    }

    Ok(())
}

#[tauri::command]
pub fn advance_processor(
    app: tauri::AppHandle,
    controller: tauri::State<'_, Controller>,
    client: tauri::State<'_, NexApiClient>,
) -> Result<ProcessorAdvanceResult, String> {
    let mut guard = controller.processor.lock().unwrap();

    let processor = guard.as_mut().ok_or("No processor selected")?;

    processor.advance(&client, &app)
}

#[tauri::command]
pub fn update_processor_data(
    controller: tauri::State<'_, Controller>,
    data: serde_json::Value,
) -> Result<(), String> {
    let mut guard = controller.processor.lock().unwrap();

    if let Some(ref mut processor) = *guard {
        processor.update_data(data)
    } else {
        Err("No processor active".into())
    }
}

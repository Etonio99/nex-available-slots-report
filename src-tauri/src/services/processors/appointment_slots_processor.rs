use std::sync::Arc;

use async_trait::async_trait;
use serde::Deserialize;

use crate::{
    api::{
        types::{
            locations::{Location, LocationsQuery},
            operatories::Operatory,
            providers::Provider,
        },
        NexApiClient,
    },
    commands::keys::get_api_key,
    services::processors::{
        traits::Processor,
        types::{
            data_confirmation::DataConfirmation,
            process_steps::ProcessStep,
            processor_advance_result::ProcessorAdvanceResult,
            processor_error::{ErrorResolutionData, ProcessorError},
        },
    },
    utils::app_state::AppState,
};

pub struct AppointmentSlotsProcessor {
    pub app_state: Arc<AppState>,
    pub current_step: ProcessStep,
    pub data: AppointmentSlotsProcessorData,
}

#[derive(Debug, Deserialize)]
pub struct AppointmentSlotsProcessorData {
    pub confirmed: Option<bool>,
    pub locations: Option<Vec<Location>>,
    pub selected_location_ids: Option<Vec<u32>>,
    pub days: Option<u32>,
    pub appointment_type_name: Option<String>,
    pub operatories: Option<Vec<Operatory>>,
    pub providers: Option<Vec<Provider>>,
}

impl AppointmentSlotsProcessor {
    pub fn new(app_state: Arc<AppState>) -> Self {
        Self {
            app_state,
            current_step: ProcessStep::CheckApiKey,
            data: AppointmentSlotsProcessorData {
                confirmed: Some(false),
                locations: None,
                selected_location_ids: None,
                days: None,
                appointment_type_name: None,
                operatories: None,
                providers: None,
            },
        }
    }

    async fn step(
        &mut self,
        client: &NexApiClient,
        app: &tauri::AppHandle,
    ) -> Result<bool, ProcessorError> {
        match self.current_step {
            ProcessStep::CheckApiKey => {
                if get_api_key()
                    .map_err(|e| {
                        ProcessorError::InternalError(ErrorResolutionData::Message(e.to_string()))
                    })?
                    .is_none()
                {
                    return Err(ProcessorError::MissingApiKey);
                }

                let response = client.get_authenticates().await.map_err(|e| {
                    ProcessorError::InternalError(ErrorResolutionData::Message(e.to_string()))
                })?;
                if !response.code {
                    return Err(ProcessorError::InvalidApiKey);
                }

                self.current_step = ProcessStep::EnterSubdomain;
            }
            ProcessStep::EnterSubdomain => {
                let guard = self.app_state.data.lock().await;

                let _ = guard
                    .subdomain
                    .as_ref()
                    .ok_or(ProcessorError::MissingSubdomain)?;

                self.current_step = ProcessStep::FetchLocations;
            }
            ProcessStep::FetchLocations => {
                let guard = self.app_state.data.lock().await;

                let subdomain = guard
                    .subdomain
                    .as_ref()
                    .ok_or(ProcessorError::MissingSubdomain)?;

                let locations_response = client
                    .get_locations(LocationsQuery {
                        subdomain: subdomain.clone(),
                        inactive: false,
                    })
                    .await
                    .map_err(|e| {
                        ProcessorError::InternalError(ErrorResolutionData::Message(e.to_string()))
                    })?;

                if let Some(institution_locations) = locations_response.data {
                    self.data.locations = Some(institution_locations[0].locations.clone());
                    self.current_step = ProcessStep::SelectLocations;
                } else {
                    return Err(ProcessorError::NoLocationsFound);
                }
            }
            ProcessStep::SelectLocations => {
                let Some(_) = self.data.selected_location_ids else {
                    return Err(ProcessorError::LocationRequired(
                        ErrorResolutionData::Locations(
                            self.data.locations.clone().unwrap_or_default(),
                        ),
                    ));
                };
                self.current_step = ProcessStep::EnterDays;
            }
            ProcessStep::EnterDays => {
                let Some(_) = self.data.days else {
                    return Err(ProcessorError::MissingDays);
                };
                self.current_step = ProcessStep::EnterAppointmentTypeName;
            }
            ProcessStep::EnterAppointmentTypeName => {
                let Some(_) = self.data.appointment_type_name else {
                    return Err(ProcessorError::MissingAppointmentTypeName);
                };
                self.current_step = ProcessStep::Confirmation;
            }
            ProcessStep::Confirmation => {
                if !self.data.confirmed.unwrap_or(false) {
                    let guard = self.app_state.data.lock().await;

                    let subdomain = guard
                        .subdomain
                        .as_ref()
                        .ok_or(ProcessorError::MissingSubdomain)?;

                    let locations_count = Some(self.data.selected_location_ids.iter().len());

                    return Err(ProcessorError::NeedsConfirmation(
                        ErrorResolutionData::Confirmation(DataConfirmation {
                            subdomain: Some(subdomain.clone()),
                            locations_count,
                            days: self.data.days,
                            appointment_type_name: self.data.appointment_type_name.clone(),
                        }),
                    ));
                }
                self.current_step = ProcessStep::Processing;
            }
            _ => return Ok(false),
        }

        Ok(true)
    }
}

#[async_trait]
impl Processor for AppointmentSlotsProcessor {
    async fn advance(
        &mut self,
        client: &NexApiClient,
        app: &tauri::AppHandle,
    ) -> Result<ProcessorAdvanceResult, String> {
        let mut error = None;

        loop {
            match self.step(client, app).await {
                Ok(true) => continue,
                Ok(false) => break,
                Err(e) => {
                    error = Some(e);
                    break;
                }
            }
        }

        Ok(ProcessorAdvanceResult {
            step: self.current_step.clone(),
            error,
        })
    }

    fn update_data(&mut self, data: serde_json::Value) -> Result<(), String> {
        let input: AppointmentSlotsProcessorData = serde_json::from_value(data)
            .map_err(|e| format!("Invalid data for Appointment Slots Processor: {}", e))?;

        if let Some(c) = input.confirmed {
            self.data.confirmed = Some(c);
        }
        if let Some(l) = input.locations {
            self.data.locations = Some(l);
        }
        if let Some(l) = input.selected_location_ids {
            self.data.selected_location_ids = Some(l);
        }
        if let Some(d) = input.days {
            self.data.days = Some(d);
        }
        if let Some(a) = input.appointment_type_name {
            self.data.appointment_type_name = Some(a);
        }
        if let Some(o) = input.operatories {
            self.data.operatories = Some(o);
        }
        if let Some(p) = input.providers {
            self.data.providers = Some(p);
        }

        Ok(())
    }
}

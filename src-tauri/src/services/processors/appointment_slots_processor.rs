use std::{os::unix::process, sync::Arc};

use async_trait::async_trait;
use chrono::Local;
use rust_xlsxwriter::{workbook::Workbook, Format, FormatAlign, FormatBorder};
use serde::Deserialize;

use crate::{
    api::{
        types::{
            appointment_slots::AppointmentSlotsQuery,
            appointment_types::AppointmentTypesQuery,
            locations::{Location, LocationsQuery},
            operatories::Operatory,
            providers::{Provider, ProvidersQuery},
        },
        NexApiClient,
    },
    commands::keys::get_api_key,
    services::processors::{
        traits::Processor,
        types::{
            appointment_slots_data::LocationAvailableSlots,
            data_confirmation::DataConfirmation,
            process_steps::ProcessStep,
            processor_advance_result::ProcessorAdvanceResult,
            processor_interrupt::{
                InterruptResolutionData, LocationResolutionData, ProcessorInterrupt,
            },
        },
    },
    utils::app_state::AppState,
};

pub struct AppointmentSlotsProcessor {
    pub app_state: Arc<AppState>,
    pub current_step: ProcessStep,
    pub target_step: Option<ProcessStep>,
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
            target_step: None,
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
    ) -> Result<bool, ProcessorInterrupt> {
        if Some(self.current_step.clone()) == self.target_step {
            self.target_step = None;
            return Err(self.get_interrupt_for_current_step().await);
        }

        match self.current_step {
            ProcessStep::CheckApiKey => {
                if get_api_key()
                    .map_err(|e| {
                        ProcessorInterrupt::InternalError(InterruptResolutionData::String(
                            e.to_string(),
                        ))
                    })?
                    .is_none()
                {
                    return Err(ProcessorInterrupt::MissingApiKey);
                }

                let response = client.get_authenticates().await.map_err(|e| {
                    ProcessorInterrupt::InternalError(InterruptResolutionData::String(
                        e.to_string(),
                    ))
                })?;
                if !response.code {
                    return Err(ProcessorInterrupt::InvalidApiKey);
                }

                self.current_step = ProcessStep::EnterSubdomain;
            }
            ProcessStep::EnterSubdomain => {
                let guard = self.app_state.data.lock().await;

                if guard.subdomain.is_none() {
                    return Err(ProcessorInterrupt::MissingSubdomain(None));
                }

                self.current_step = ProcessStep::FetchLocations;
            }
            ProcessStep::FetchLocations => {
                if self.data.locations.is_none() {
                    let guard = self.app_state.data.lock().await;

                    let Some(subdomain) = guard.subdomain.as_ref() else {
                        return Err(ProcessorInterrupt::MissingSubdomain(None));
                    };

                    let locations_response = client
                        .get_locations(LocationsQuery {
                            subdomain: subdomain.clone(),
                            inactive: false,
                        })
                        .await
                        .map_err(|e| {
                            ProcessorInterrupt::InternalError(InterruptResolutionData::String(
                                e.to_string(),
                            ))
                        })?;

                    println!("{:#?}", locations_response);

                    if !locations_response.code {
                        if let Some(e) = locations_response.error {
                            if e.contains(
                                &"You don't have access to perform this action.".to_string(),
                            ) {
                                let permission_type = &Some("subdomain".to_string());
                                return Err(ProcessorInterrupt::PermissionDenied(
                                    self.wrap_str(permission_type)
                                        .unwrap_or(InterruptResolutionData::None),
                                ));
                            }
                        }
                    }

                    if let Some(institution_locations) = locations_response.data {
                        self.data.locations = Some(institution_locations[0].locations.clone());
                    } else {
                        return Err(ProcessorInterrupt::NoLocationsFound);
                    }
                }

                self.current_step = ProcessStep::SelectLocations;
            }
            ProcessStep::SelectLocations => {
                let Some(_) = self.data.selected_location_ids else {
                    return Err(ProcessorInterrupt::LocationRequired(Some(
                        InterruptResolutionData::Locations(LocationResolutionData {
                            locations: self.data.locations.clone().unwrap_or_default(),
                            selected_location_ids: None,
                        }),
                    )));
                };
                self.current_step = ProcessStep::EnterDays;
            }
            ProcessStep::EnterDays => {
                let Some(_) = self.data.days else {
                    return Err(ProcessorInterrupt::MissingDays(None));
                };
                self.current_step = ProcessStep::EnterAppointmentTypeName;
            }
            ProcessStep::EnterAppointmentTypeName => {
                let Some(_) = self.data.appointment_type_name else {
                    return Err(ProcessorInterrupt::MissingAppointmentTypeName(None));
                };
                self.current_step = ProcessStep::Confirmation;
            }
            ProcessStep::Confirmation => {
                if !self.data.confirmed.unwrap_or(false) {
                    let guard = self.app_state.data.lock().await;

                    let locations_count = self
                        .data
                        .selected_location_ids
                        .as_ref()
                        .map(|v| v.len() as u32);

                    return Err(ProcessorInterrupt::NeedsConfirmation(
                        InterruptResolutionData::Confirmation(DataConfirmation {
                            subdomain: guard.subdomain.clone(),
                            locations_count,
                            days: self.data.days,
                            appointment_type_name: self.data.appointment_type_name.clone(),
                        }),
                    ));
                }
                self.current_step = ProcessStep::Processing;
            }
            ProcessStep::Processing => {
                self.process(client).await?;
                self.current_step = ProcessStep::Complete;
            }
            _ => return Ok(false),
        }

        Ok(true)
    }

    async fn get_interrupt_for_current_step(&self) -> ProcessorInterrupt {
        match self.current_step {
            ProcessStep::CheckApiKey => ProcessorInterrupt::MissingApiKey,
            ProcessStep::EnterSubdomain => {
                let guard = self.app_state.data.lock().await;

                if guard.subdomain.is_none() {
                    return ProcessorInterrupt::MissingSubdomain(None);
                }

                ProcessorInterrupt::MissingSubdomain(self.wrap_str(&guard.subdomain))
            }
            ProcessStep::SelectLocations => ProcessorInterrupt::LocationRequired(Some(
                InterruptResolutionData::Locations(LocationResolutionData {
                    locations: self.data.locations.clone().unwrap_or_default(),
                    selected_location_ids: self.data.selected_location_ids.clone(),
                }),
            )),
            ProcessStep::EnterDays => {
                ProcessorInterrupt::MissingDays(self.wrap_num(self.data.days))
            }
            ProcessStep::EnterAppointmentTypeName => {
                ProcessorInterrupt::MissingAppointmentTypeName(
                    self.wrap_str(&self.data.appointment_type_name),
                )
            }
            _ => ProcessorInterrupt::InternalError(InterruptResolutionData::String(
                "Unknown step".into(),
            )),
        }
    }

    fn wrap_str(&self, value: &Option<String>) -> Option<InterruptResolutionData> {
        value
            .as_ref()
            .map(|s| InterruptResolutionData::String(s.clone()))
    }

    fn wrap_num(&self, value: Option<u32>) -> Option<InterruptResolutionData> {
        value.map(InterruptResolutionData::Number)
    }

    async fn process(&self, client: &NexApiClient) -> Result<(), ProcessorInterrupt> {
        let mut available_slot_data: Vec<LocationAvailableSlots> = vec![];

        let guard = self.app_state.data.lock().await;

        let Some(subdomain) = guard.subdomain.as_ref() else {
            return Err(ProcessorInterrupt::MissingSubdomain(None));
        };

        let Some(appointment_type_name) = self.data.appointment_type_name.as_ref() else {
            return Err(ProcessorInterrupt::InternalError(
                InterruptResolutionData::String("Appointment type name is missing".into()),
            ));
        };

        let Some(days) = self.data.days.as_ref() else {
            return Err(ProcessorInterrupt::InternalError(
                InterruptResolutionData::String("Days is missing".into()),
            ));
        };

        if let Some(location_ids) = self.data.selected_location_ids.clone() {
            for location_id in location_ids {
                let appointment_types_response = client
                    .get_appointment_types(AppointmentTypesQuery {
                        subdomain: subdomain.clone(),
                        location_id,
                    })
                    .await
                    .map_err(|e| {
                        ProcessorInterrupt::InternalError(InterruptResolutionData::String(
                            e.to_string(),
                        ))
                    })?;

                if !appointment_types_response.code {
                    println!("API call failed: {:#?}", appointment_types_response);
                    return Err(ProcessorInterrupt::InternalError(
                        InterruptResolutionData::None,
                    ));
                }

                let Some(appointment_types) = appointment_types_response.data else {
                    return Err(ProcessorInterrupt::InternalError(
                        InterruptResolutionData::String("Appointment types value is None".into()),
                    ));
                };

                let matched_appointment_type = appointment_types
                    .iter()
                    .find(|at| at.name.to_lowercase() == appointment_type_name.to_lowercase());

                let Some(appointment_type) = matched_appointment_type else {
                    println!(
                        "Could not find appointment type match for location {}",
                        location_id
                    );
                    continue;
                };

                let providers_response = client
                    .get_providers(ProvidersQuery {
                        subdomain: subdomain.clone(),
                        location_id,
                        inactive: false,
                        requestable: true,
                        per_page: 300,
                    })
                    .await
                    .map_err(|e| {
                        ProcessorInterrupt::InternalError(InterruptResolutionData::String(
                            e.to_string(),
                        ))
                    })?;

                if !providers_response.code {
                    println!("API call failed: {:#?}", providers_response);
                    return Err(ProcessorInterrupt::InternalError(
                        InterruptResolutionData::None,
                    ));
                }

                let Some(providers_list) = providers_response.data else {
                    return Err(ProcessorInterrupt::InternalError(
                        InterruptResolutionData::String("Providers value is None".into()),
                    ));
                };

                let provider_ids = providers_list.iter().map(|p| p.id).collect();

                let start_date = Local::now().date_naive();

                let appointment_slots_response = client
                    .get_appointment_slots(AppointmentSlotsQuery {
                        subdomain: subdomain.clone(),
                        start_date,
                        days: *days,
                        appointment_type_id: appointment_type.id,
                        location_id,
                        provider_ids,
                    })
                    .await
                    .map_err(|e| {
                        ProcessorInterrupt::InternalError(InterruptResolutionData::String(
                            e.to_string(),
                        ))
                    })?;

                if !appointment_slots_response.code {
                    println!("API call failed: {:#?}", appointment_slots_response);
                    return Err(ProcessorInterrupt::InternalError(
                        InterruptResolutionData::None,
                    ));
                }

                println!("Success! {:#?}", appointment_slots_response);
            }
        }

        // let mut workbook = Workbook::new();

        // let bold_format = Format::new().set_bold();
        // let decimal_format = Format::new().set_num_format("0.000");
        // let date_format = Format::new().set_num_format("yyyy-mm-dd");
        // let merge_format = Format::new()
        //     .set_border(FormatBorder::Thin)
        //     .set_align(FormatAlign::Center);

        // let worksheet = workbook.add_worksheet();

        Ok(())
    }
}

#[async_trait]
impl Processor for AppointmentSlotsProcessor {
    async fn advance(
        &mut self,
        client: &NexApiClient,
        app: &tauri::AppHandle,
    ) -> Result<ProcessorAdvanceResult, String> {
        let mut interrupt = None;

        loop {
            match self.step(client, app).await {
                Ok(true) => continue,
                Ok(false) => break,
                Err(e) => {
                    interrupt = Some(e);
                    break;
                }
            }
        }

        Ok(ProcessorAdvanceResult {
            step: self.current_step.clone(),
            interrupt,
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

    fn make_stale(&mut self) {
        self.data.locations = None;
        self.data.selected_location_ids = None;
    }

    fn jump_to_step(&mut self, step: ProcessStep) {
        self.current_step = step.clone();
        self.target_step = Some(step.clone());
    }
}

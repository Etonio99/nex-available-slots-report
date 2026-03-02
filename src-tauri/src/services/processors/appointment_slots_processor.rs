use crate::{api::{NexApiClient, types::{operatories::Operatory, providers::Provider}}, services::processors::{traits::Advancement, types::process_steps::ProcessStep}};

pub struct AppointmentSlotsProcessor {
    pub current_step: ProcessStep,
    pub subdomain: Option<String>,
    pub locations: Vec<u32>,
    pub days: Option<u32>,
    pub appointment_type_id: Option<u32>,
    pub operatories: Vec<Operatory>,
    pub providers: Vec<Provider>,
}

impl AppointmentSlotsProcessor {
    pub fn new() -> Self {
        Self {
            current_step: ProcessStep::CheckApiKey,
            subdomain: None,
            locations: vec![],
            days: None,
            appointment_type_id: None,
            operatories: vec![],
            providers: vec![],
        }
    }
}

impl Advancement for AppointmentSlotsProcessor {
    fn advance(&mut self, client: &NexApiClient) -> Result<(), Box<dyn std::error::Error>> {
        match self.current_step {
            ProcessStep::CheckApiKey => {
                
            },
            ProcessStep::EnterSubdomain => {
                if let Some(_) = self.subdomain {
                    return Err("Subdomain is required".into());
                }
                self.current_step = ProcessStep::SelectLocations;
                // app.emit_all("workflow-progress", self.current_step)?;
            },
            _ => {},
        }

        Ok(())
    }
}
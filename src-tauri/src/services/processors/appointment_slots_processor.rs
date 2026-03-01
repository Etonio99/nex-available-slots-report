pub struct AppointmentSlotsProcessor {
    pub current_step: ProcessStep,

    pub subdomain: String,
    pub locations: Vec<u32>,
    pub days: u32,
    pub appointment_type_id: u32,
    pub operatories: Vec<Operatory>,
    pub providers: Vec<Provider>,
}

impl Advancement for AppointmentSlotsProcessor {
    fn advance(&mut self, client: &NexApiClient) -> Result<(), Box<dyn std::error::Error>> {
        match self.current_step {
            ProcessStep::CheckApiKey => {
                
            },
            ProcessStep::EnterSubdomain => {
                if self.subdomain.is_none() {
                    Err("Subdomain is required".into())
                }
                self.current_step = ProcessStep.SelectLocations;
                // app.emit_all("workflow-progress", self.current_step)?;
            },
        }

        Ok(())
    }
}

pub enum ProcessStep {
    CheckApiKey,
    EnterSubdomain,
    SelectLocations,
    SelectAppointmentType,
    EnterDays,
    CollectContext,
    CollectAnalytics,
    Complete,
}
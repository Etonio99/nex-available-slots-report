use serde::Serialize;

#[derive(Clone, Serialize)]
pub enum ProcessStep {
    CheckApiKey,
    EnterSubdomain,
    FetchLocations,
    SelectLocations,
    EnterAppointmentTypeName,
    EnterDays,
    Confirmation,
    Processing,
    Complete,
}

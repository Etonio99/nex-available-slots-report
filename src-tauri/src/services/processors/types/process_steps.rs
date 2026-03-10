use serde::Serialize;

#[derive(Clone, Serialize)]
pub enum ProcessStep {
    CheckApiKey,
    EnterSubdomain,
    FetchLocations,
    SelectLocations,
    EnterAppointmentTypeName,
    EnterDays,
    Confirm,
    CollectContext,
    CollectAnalytics,
    Complete,
}

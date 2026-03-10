use serde::Serialize;

use crate::{
    api::types::locations::Location,
    services::processors::types::data_confirmation::DataConfirmation,
};

#[derive(Serialize, Debug, Clone)]
#[serde(
    tag = "type",
    content = "resolutionData",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum ProcessorError {
    MissingApiKey,
    InvalidApiKey,
    MissingSubdomain,
    LocationRequired(ErrorResolutionData),
    NoLocationsFound,
    MissingDays,
    MissingAppointmentTypeName,
    NeedsConfirmation(ErrorResolutionData),
    InternalError(ErrorResolutionData),
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "payload", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorResolutionData {
    Message(String),
    Locations(Vec<Location>),
    Confirmation(DataConfirmation),
    None,
}

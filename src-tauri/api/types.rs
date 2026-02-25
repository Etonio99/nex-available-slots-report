use chrono::{
    NaiveDate,
    DateTime,
    Utc,
};
use serde::{
    Serialize,
    Deserialize
};

#[derive(Debug, Deserialize)]
pub struct NexApiResponse<T> {
    pub code: bool,
    pub data: Option<T>,
    pub description: Option<Vec<String>>,
    pub error: Option<Vec<String>>,
    pub count: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct AppointmentSlots {
    pub lid: u32,
    pub pid: Option<u32>,
    pub operatory_id: Option<u32>,
    pub slots: Option<Vec<AppointmentSlot>>,
    pub next_available_date: Option<NaiveDate>,
}

#[derive(Debug, Deserialize)]
pub struct AppointmentSlot {
    pub time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub operatory_id: Option<u32>,
    pub provider_id: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct ProviderLocationMap {
    pub location_id: u32,
    pub provider_ids: Vec<u32>,
}

#[derive(Debug, Deserialize)]
pub struct LocationAvailableSlots {
    pub location_id: u32,
    pub provider_ids: Vec<u32>,
}

#[derive(Serialize)]
struct AppointmentSlotsQuery {
    start_date: NaiveDate,
    days: u32,

    #[serde(rename = "provider_id[]")]
    provider_ids: Vec<u32>,
}
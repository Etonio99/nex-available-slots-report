use chrono::{
    NaiveDate,
    DateTime,
    Utc,
};
use serde::{
    Serialize,
    Deserialize,
    Serializer,
};

#[derive(Debug, Deserialize)]
pub struct NexApiResponse<T> {
    pub code: bool,
    pub data: Option<T>,
    pub description: Option<Vec<String>>,
    pub error: Option<Vec<String>>,
    pub count: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppointmentSlots {
    pub lid: u32,
    pub pid: Option<u32>,
    pub operatory_id: Option<u32>,
    pub slots: Option<Vec<AppointmentSlot>>,
    pub next_available_date: Option<NaiveDate>,
}

#[derive(Debug, Serialize, Deserialize)]
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
pub struct AppointmentSlotsQuery {
    #[serde(serialize_with = "date_to_string")]
    pub start_date: NaiveDate,
    pub days: u32,
    pub appointment_type_id: u32,

    #[serde(rename = "lids")]
    pub location_id: u32,

    #[serde(rename = "pids")]
    pub provider_ids: Vec<u32>,
}

fn date_to_string<S>(date: &NaiveDate, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&date.format("%Y-%m-%d").to_string())
}
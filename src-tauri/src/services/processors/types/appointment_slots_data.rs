#[derive(Debug)]
pub struct LocationAvailableSlots {
    pub location_id: u32,
    pub error: Option<LocationAvailableSlotsError>,
    pub available_slots: Option<Vec<AvailableSlotsInTimeframe>>,
}

#[derive(Debug)]
pub enum LocationAvailableSlotsError {
    AppointmentTypeNotFound,
    NoSlotData,
}

#[derive(Debug)]
pub struct AvailableSlotsInTimeframe {
    pub day: String,
    pub available_slots_count: u32,
}

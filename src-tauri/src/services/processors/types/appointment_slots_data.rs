#[derive(Debug)]
pub struct LocationAvailableSlots {
    pub location_id: u32,
    pub available_slots: Vec<AvailableSlotsInTimeframe>,
}

#[derive(Debug)]
pub struct AvailableSlotsInTimeframe {
    pub day: String,
    pub available_slots_count: u32,
}

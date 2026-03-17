pub struct LocationAvailableSlots {
    pub location_id: u32,
    pub available_slots: Vec<AvailableSlotsInTimeframe>,
}

pub struct AvailableSlotsInTimeframe {
    pub week: u32,
    pub available_slots_count: u32,
}

use chrono::{DateTime, Utc};
use core::fmt;

pub struct Trip {
    // A Trip modelation
    pub id: u32,
    pub start: DateTime<Utc>,
    pub end: Option<DateTime<Utc>>,
    pub is_open: bool,
}

impl fmt::Debug for Trip {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Trip")
            .field("id", &self.id)
            .field("start", &self.start)
            .field("end", &self.end)
            .field("is_open", &self.is_open)
            .finish()
    }
}

impl Trip {
    pub fn add(id: u32, start: DateTime<Utc>) -> Trip {
        // Add a Trip who is open
        Trip {
            id: id,
            start: start,
            end: None,
            is_open: true,
        }
    }

    pub fn end_trip(&mut self, end: DateTime<Utc>) {
        // Add the final date and close a Trip
        self.end = Some(end);
        self.is_open = false
    }
}

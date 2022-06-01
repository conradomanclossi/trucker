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

#[allow(dead_code)]
pub struct Trips {
    // A Trips modelation
    pub vec: Vec<Trip>,
}

#[allow(dead_code)]
impl fmt::Debug for Trips {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Trips").field("vec", &self.vec).finish()
    }
}

#[allow(dead_code)]
impl Trips {
    pub fn new() -> Trips {
        // Create a new Trips
        Trips { vec: Vec::new() }
    }

    pub fn post(&mut self, id: u32, start: DateTime<Utc>, end: Option<DateTime<Utc>>) {
        // Add a Trip to the Trips
        match end {
            Some(end) => self.vec.push(Trip {
                id: id,
                start: start,
                end: Some(end),
                is_open: true,
            }),
            None => self.vec.push(Trip {
                id,
                start,
                end: None,
                is_open: true,
            }),
        }        
    }

    pub fn get(&self, id: u32) -> Option<&Trip> {
        // Get a Trip from the Trips
        self.vec.iter().find(|trip| trip.id == id)
    }

    pub fn put(&mut self, id: u32, start: Option<DateTime<Utc>>, end: Option<DateTime<Utc>>) {
        // Update a Trip in the Trips
        for trip in &mut self.vec {
            if trip.id == id {
                if start != None {
                    trip.start = start.unwrap();
                } else if end != None {
                    trip.end = Some(end.unwrap());
                    trip.is_open = false;
                } else if start != None && end != None {
                    trip.start = start.unwrap();
                    trip.end = Some(end.unwrap());
                    trip.is_open = false;
                }
            }
        }
    }

    pub fn delete(&mut self, id: u32) {
        // Delete a Trip from the Trips
        self.vec.retain(|trip| trip.id != id);
    }
}

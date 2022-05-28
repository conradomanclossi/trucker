use chrono::{DateTime, Utc};
use core::fmt;

pub struct Record {
    // A Record modelation
    pub id: u32,
    pub trip_id: u32,
    pub category_id: u32,
    pub name: &'static str,
    pub date: DateTime<Utc>,
    pub amount: u32,
    pub value_per_amount: f64,
    pub total: f64,
}

impl fmt::Debug for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Record")
            .field("id", &self.id)
            .field("trip_id", &self.trip_id)
            .field("category_id", &self.category_id)
            .field("name", &self.name)
            .field("date", &self.date)
            .field("amount", &self.amount)
            .field("value per amount", &self.value_per_amount)
            .field("total", &self.total)
            .finish()
    }
}

impl Record {
    pub fn add(id: u32, trip_id: u32, category_id: u32, name: &'static str, date: DateTime<Utc>, amount: u32, value_per_amount: f64, total: f64) -> Record {
        // Add a Record
        Record {
            id: id,
            trip_id: trip_id,
            category_id: category_id,
            name: name,
            date: date,
            amount: amount,
            value_per_amount: value_per_amount,
            total: total,
        }
    }
}
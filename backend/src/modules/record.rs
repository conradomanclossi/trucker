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

#[allow(dead_code)]
pub struct Records {
    // A Records modelation
    pub vec: Vec<Record>,
}

#[allow(dead_code)]
impl fmt::Debug for Records {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Records").field("vec", &self.vec).finish()
    }
}

#[allow(dead_code)]
impl Records {
    pub fn new() -> Records {
        // Create a new Records
        Records { vec: Vec::new() }
    }

    pub fn post(
        &mut self,
        id: u32,
        trip_id: u32,
        category_id: u32,
        name: &'static str,
        date: DateTime<Utc>,
        amount: u32,
        value_per_amount: f64,
        total: f64,
    ) {
        // Add a Record
        self.vec.push(Record {
            id: id,
            trip_id: trip_id,
            category_id: category_id,
            name: name,
            date: date,
            amount: amount,
            value_per_amount: value_per_amount,
            total: total,
        });
    }

    pub fn get(&self, id: u32) -> Option<&Record> {
        // Get a Record from the Records
        self.vec.iter().find(|&record| record.id == id)
    }

    pub fn put(
        &mut self,
        id: u32,
        trip_id: u32,
        category_id: u32,
        name: &'static str,
        date: DateTime<Utc>,
        amount: u32,
        value_per_amount: f64,
        total: f64,
    ) {
        // Update a Record from the Records
        for record in &mut self.vec {
            if record.id == id {
                record.trip_id = trip_id;
                record.category_id = category_id;
                record.name = name;
                record.date = date;
                record.amount = amount;
                record.value_per_amount = value_per_amount;
                record.total = total;
            }
        }
    }

    pub fn delete(&mut self, id: u32) {
        // Delete a Record from the Records
        self.vec.retain(|record| record.id != id);
    }
}

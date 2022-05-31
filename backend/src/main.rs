use chrono::{DateTime, Utc};

mod modules;
use modules::trip::Trips;
use modules::record::Record;
use modules::category::Categories;

fn main() {
    let now: DateTime<Utc> = Utc::now();

    let mut trips: Trips = Trips::new();
    trips.post(1, now, None);
    trips.post(2, now, None);
    trips.post(3, now, None);
    println!("{:?}", trips.get(2));
    trips.put(2, None, Some(now));
    println!("{:?}", trips.get(2));
    trips.delete(2);
    println!("{:?}", trips);

    let mut categories: Categories = Categories::new();
    categories.post(1, "Diesel");
    println!("{:?}", categories);

    let record_teste: Record = Record::add(1, 1, 1, "Diesel", now, 100, 1.0, 100.0);
    println!("{:?}", record_teste);
}

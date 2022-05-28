use chrono::{DateTime, Utc};

mod modules;
use modules::trip::Trip;
use modules::category::Category;
use modules::record::Record;

fn main() {
    let now: DateTime<Utc> = Utc::now();

    let mut trip_teste = Trip::add(1, now);
    println!("{:?}", trip_teste);

    trip_teste.end_trip(now);
    println!("{:?}", trip_teste);

    let category_teste: Category = Category::add(1, "Diesel");
    println!("{:?}", category_teste);

    let record_teste: Record = Record::add(1, trip_teste.id, category_teste.id, "Diesel", now, 100, 1.0, 100.0);
    println!("{:?}", record_teste);
}

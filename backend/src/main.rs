use chrono::{DateTime, Utc};

mod modules;
use modules::trip::Trip;
use modules::category::Category;

fn main() {
    let now: DateTime<Utc> = Utc::now();

    let mut trip_teste = Trip::add(1, now);
    println!("{:?}", trip_teste);

    trip_teste.end_trip(now);
    println!("{:?}", trip_teste);

    let category_teste: Category = Category::add(1, "Diesel");
    println!("{:?}", category_teste);
}

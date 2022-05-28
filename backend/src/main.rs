use chrono::{DateTime, Utc};

mod modules;
use modules::trip::Trip;

fn main() {
    let now: DateTime<Utc> = Utc::now();

    let mut trip_teste = Trip::add(1, now);
    println!("{:?}", trip_teste);

    trip_teste.end_trip(now);
    println!("{:?}", trip_teste);

}

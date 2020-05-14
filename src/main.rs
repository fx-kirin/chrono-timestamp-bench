use chrono::TimeZone;

use serde::{Deserialize, Serialize};
use chrono::serde::ts_nanoseconds;
use chrono::{DateTime, Utc};
use csv::Writer;
use easybench::{bench,bench_env};

#[macro_use]
extern crate serde_derive;

#[derive(Deserialize, Serialize)]
pub struct Order {
    #[serde(with = "ts_nanoseconds")]
    time: DateTime<Utc>,
}

#[derive(Deserialize, Serialize)]
pub struct Order2 {
    time: i64,
}

fn bencher(bb: i64){
    for _ in 0..bb{
        let mut wtr = Writer::from_writer(vec![]);
        let dt = Utc::now();
        wtr.serialize(Order{time:dt}).unwrap();
    }
}

fn bencher2(bb: i64){
    for _ in 0..bb{
        let mut wtr = Writer::from_writer(vec![]);
        let dt = Utc::now().timestamp_nanos();
        wtr.serialize(Order2{time:dt}).unwrap();
    }
}

fn main() {
    println!("1 100: {}", bench(|| bencher(1) ));
    println!("2 100: {}", bench(|| bencher2(1) ));
}

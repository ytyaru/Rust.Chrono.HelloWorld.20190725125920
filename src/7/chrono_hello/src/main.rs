//use chrono::prelude::*;
//use chrono::{NaiveDateTime, NaiveDate};
fn main() {
    let d1 = chrono::NaiveDateTime::parse_from_str("2019-07-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let d2 = chrono::NaiveDateTime::parse_from_str("2019-07-01 01:30:00", "%Y-%m-%d %H:%M:%S").unwrap();
    println!("{}", d1);
    println!("{}", d2);

    let du: chrono::Duration = d1 - d2;
    println!("{}", du);
    println!("{:?}", du);
    println!("{}", du.num_seconds());
    println!("{}", du.num_minutes());
    println!("{}", du.num_hours());

    let d3 = d2 + chrono::Duration::minutes(30);
    println!("{}", d3);
}

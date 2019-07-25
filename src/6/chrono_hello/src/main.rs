use chrono::prelude::*;
//use chrono::{NaiveDateTime, NaiveDate};
fn main() {
    let utc: DateTime<Utc> = Utc::now();
    let local: DateTime<Local> = Local::now();

    println!("{}", utc);
    println!("{}", local);
    println!("{}", utc.naive_utc());
    println!("{}", utc.naive_local());
    println!("{}", local.naive_utc());
    println!("{}", local.naive_local());

    /*
    let date_str = "2000-01-02 03:04:05";
    let dt = DateTime::parse_from_str(format!("{} {}", date_str, Local::now().format("%z")).as_str(), "%Y-%m-%d %H:%M:%S %z");
    println!("{}", dt.unwrap());
    println!("{:?}", dt.unwrap());
    */
    let dt = chrono::NaiveDateTime::parse_from_str("2000-01-02 03:04:05", "%Y-%m-%d %H:%M:%S");
    println!("{}", dt.unwrap());
    println!("{:?}", dt.unwrap());
}

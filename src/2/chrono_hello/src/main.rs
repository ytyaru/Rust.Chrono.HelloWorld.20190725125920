use chrono::prelude::*;
fn main() {
    let local: DateTime<Local> = Local::now();
    let utc: DateTime<Utc> = Utc::now();
    println!("{}", utc);
    println!("{}", local);
    
    println!("{}", local.format("%Y-%m-%d %H:%M:%S").to_string());
    println!("{}", local.to_rfc3339());

    let dt = DateTime::parse_from_rfc3339(local.to_rfc3339().as_str());
    println!("{}", dt.unwrap().to_rfc3339());

//    let dt = DateTime::parse_from_str("2000-01-02 03:04:05", "%Y-%m-%d %H:%M:%S"); // thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: ParseError(NotEnough)'
    let dt = DateTime::parse_from_str("2000-01-02 03:04:05 +09:00", "%Y-%m-%d %H:%M:%S %z");
    println!("{}", dt.unwrap());
    println!("{:?}", dt.unwrap());
}

use chrono::{Utc, Local, TimeZone};
fn main() {
    let utc = Utc::now();
    println!("{}", utc.timestamp());
    println!("{}", Utc.timestamp(utc.timestamp(), 0));

    let local = Local::now();
    println!("{}", local.timestamp());
    println!("{}", Local.timestamp(local.timestamp(), 0));
}

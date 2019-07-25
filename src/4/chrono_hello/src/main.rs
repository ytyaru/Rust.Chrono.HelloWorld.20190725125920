use chrono::prelude::*;
fn main() {
//    let dt = chrono::offset::FixedOffset::east(9*3600).parse_form_str("2000-01-02 03:04:05", "%Y-%m-%d %H:%M:%S"); // error[E0599]: no method named `parse_form_str` found for type `chrono::offset::fixed::FixedOffset` in the current scope
    let dt = chrono::offset::FixedOffset::east(9*3600).ymd(2000, 1, 2).and_hms(3, 4, 5);
    println!("{}", dt);

//    println!("{}", local.timezone()); // error[E0277]: `chrono::offset::local::Local` doesn't implement `std::fmt::Display`
    println!("{:?}", local.timezone()); // Local
    println!("{}", local.format("%z")); // +0900
    let date_str = "2000-01-02 03:04:05";
//    let dt = DateTime::parse_from_str(date_str + Local::now().format("%z"), "%Y-%m-%d %H:%M:%S %z"); // error[E0369]: binary operation `+` cannot be applied to type `&str`
    let dt = DateTime::parse_from_str(format!("{} {}", date_str, Local::now().format("%z")).as_str(), "%Y-%m-%d %H:%M:%S %z");
    println!("{}", dt.unwrap()); // 
}

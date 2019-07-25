fn main() {
    let utc = chrono::Utc::now();
    println!("{}", utc.timestamp());

    let local = chrono::Local::now();
    println!("{}", local.timestamp());
}

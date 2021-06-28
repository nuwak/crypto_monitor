use chrono::{Datelike, Timelike, Utc, Local};

fn main() {
    let now = Utc::now();
    let local_time = Local::now();
    if now.hour() < 21 && now.hour() > 6 {
        println!("send");
    } else {
        println!("not send");
    }
    println!("{}", now);
    println!("{}", local_time);

    let (is_pm, hour) = now.hour12();
    println!(
        "The current UTC time is {:02}:{:02}:{:02} {}",
        hour,
        now.minute(),
        now.second(),
        if is_pm { "PM" } else { "AM" }
    );
    println!(
        "And there have been {} seconds since midnight",
        now.num_seconds_from_midnight()
    );

    let (is_common_era, year) = now.year_ce();
    println!(
        "The current UTC date is {}-{:02}-{:02} {:?} ({})",
        year,
        now.month(),
        now.day(),
        now.weekday(),
        if is_common_era { "CE" } else { "BCE" }
    );
    println!(
        "And the Common Era began {} days ago",
        now.num_days_from_ce()
    );
}
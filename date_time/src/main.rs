use chrono::prelude::*;
use std::time::{UNIX_EPOCH, Duration};
fn main() {
    let sample_date1_in_utc: DateTime<Utc> = Utc::now();
    println!("sample_date1_in_utc: {}", sample_date1_in_utc);
    
    let sample_date2_in_utc = Utc.with_ymd_and_hms(2023, 3, 1, 1, 2, 3).unwrap();
    println!("sample date 2 (in utc): {sample_date2_in_utc}");
    
    let sample_date3_in_utc = DateTime::<Utc>::from(UNIX_EPOCH + Duration::from_secs(1524885322));
    println!("sample date 3 (in utc): {sample_date3_in_utc}");

    let sample_date4_in_utc = "2023-03-01 01:02:03 UTC".parse::<DateTime<Utc>>().unwrap();
    println!("sample date 4 (in utc): {sample_date4_in_utc}");

    let date1_in_utc: DateTime<Utc> = Utc::now();
    let date2_in_utc = Utc.with_ymd_and_hms(2023, 3, 1, 1, 2, 3).unwrap();
    let date3_in_utc = DateTime::<Utc>::from(UNIX_EPOCH + Duration::from_secs(1524885322));
    let date4_in_utc = "2023-03-01 01:02:03 UTC".parse::<DateTime<Utc>>().unwrap();

    println!("sample date 1 (in utc): {date1_in_utc}");
    println!("sample date 2 (in utc): {date2_in_utc}");
    println!("sample date 3 (in utc): {date3_in_utc}");
    println!("sample date 4 (in utc): {date4_in_utc}");


    let date1_in_local_tz: DateTime<Local> = Local::now();
    let date2_in_local_tz = Local.with_ymd_and_hms(2023, 3, 1, 1, 2, 3).unwrap();
    let date3_in_local_tz = DateTime::<Local>::from(UNIX_EPOCH + Duration::from_secs(1524885322));
    let date4_in_local_tz = "2023-03-01 01:02:03 UTC".parse::<DateTime<Local>>().unwrap();

    println!("sample date 1 (in local_tz): {date1_in_local_tz}");
    println!("sample date 2 (in local_tz): {date2_in_local_tz}");
    println!("sample date 3 (in local_tz): {date3_in_local_tz}");
    println!("sample date 4 (in local_tz): {date4_in_local_tz}");

    let date1_in_local: DateTime<Local> = Local::now();
    println!("sample date 1 (in local): {date1_in_local}");
    let date_in_utc = DateTime::<Utc>::from(date1_in_local);
    println!("sample date 1 (in utc): {date_in_utc}");
    let date_in_local_from_utc = DateTime::<Local>::from(date_in_utc);
    println!("sample date 1 (in local): {date_in_local_from_utc}");
    println!("sample date 1 in Unix timestamp: {}", date1_in_local.timestamp());
    println!("sampe date 1 in milisecond timestamp: {}", date1_in_local.timestamp_millis());
    let new_datetime = DateTime::<Local>::from(UNIX_EPOCH + Duration::from_millis(date1_in_local.timestamp_millis().unsigned_abs()));
    println!("new datetime: {new_datetime}");

}

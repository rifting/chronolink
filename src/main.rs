use binder_rust::{Parcel, ServiceManager};
use chrono::{Local, NaiveTime, Weekday};
use chrono_tz::Tz;
use inquire::{InquireError, Select};
use std::{collections::HashMap, str::FromStr};

// i copy and pasted a lot haha. i hate working with time
fn main() {
    let local_time = Local::now();
    let lines = get_timezones();

    let mut time_to_tz_map: HashMap<String, String> = HashMap::new();

    for line in get_logo() {
        println!("\x1b[93m{}\x1b[0m", line);
    }

    println!("\x1b[93mJ & K to scroll, Enter to select.\x1b[0m");

    // this is horrible
    for tz_name in lines.iter() {
        if let Ok(tz) = tz_name.parse::<Tz>() {
            let formatted_time = local_time
                .with_timezone(&tz)
                .format("%I:%M %p %A")
                .to_string();
            time_to_tz_map.insert(formatted_time, tz.name().to_string());
        }
    }

    let mut unique_times: Vec<String> = time_to_tz_map.keys().cloned().collect();
    unique_times.sort_by_key(|time_str| {
        let parts: Vec<&str> = time_str.rsplitn(2, ' ').collect();
        let time_part = parts[1];
        let day_part = parts[0];

        let day_order = match Weekday::from_str(day_part) {
            Ok(Weekday::Mon) => 1,
            Ok(Weekday::Tue) => 2,
            Ok(Weekday::Wed) => 3,
            Ok(Weekday::Thu) => 4,
            Ok(Weekday::Fri) => 5,
            Ok(Weekday::Sat) => 6,
            Ok(Weekday::Sun) => 7,
            _ => 8,
        };

        let time_obj = NaiveTime::parse_from_str(time_part, "%I:%M %p")
            .unwrap_or_else(|_| NaiveTime::from_hms_opt(0, 0, 0).unwrap());

        (day_order, time_obj)
    });

    let ans: Result<String, InquireError> = Select::new("Select Time", unique_times)
        .with_vim_mode(true)
        .without_help_message()
        .prompt();

    match ans {
        Ok(choice) => {
            if let Some(tz_name) = time_to_tz_map.get(&choice) {
                // Get the alarm service manager ready so we can write a parcel with our chosen time zone
                let service_manager = &mut ServiceManager::new();
                let mut alarm_manager =
                    service_manager.get_service("alarm", "android.app.IAlarmManager");

                let mut parcel = Parcel::empty();
                parcel.write_str16(tz_name);

                // 3 is the opcode for setting the time zone
                let res = alarm_manager.call(3, &mut parcel);
                println!("Successfully set time zone to \x1b[93m{}\x1b[0m", tz_name);
            }
        }
        // Hopefully this should never happen?
        Err(_) => println!("There was an error, please try again"),
    }
}

fn get_timezones() -> Vec<String> {
    include_str!("timezones")
        .lines()
        .map(String::from)
        .collect()
}

fn get_logo() -> Vec<String> {
    include_str!("logo").lines().map(String::from).collect()
}

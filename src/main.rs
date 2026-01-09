use std::collections::HashSet;
use chrono::{Utc};
use chrono_tz::Tz;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "chronolink")]
#[command(about = "Set timezone or list available timezones")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Timezone to set (ex. America/New_York)
    #[arg(value_name = "TIMEZONE")]
    timezone: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all available timezones with current time
    #[command(visible_alias = "ls")]
    List,
}

fn main() {
    let cli = Cli::parse();

    for line in get_logo() {
        println!("\x1b[93m{}\x1b[0m", line);
    }

    match cli.command {
        Some(Commands::List) => list_timezones(),
        None => {
            if let Some(tz_name) = cli.timezone {
                set_timezone(&tz_name);
            } else {
                println!("Usage: chronolink <TIMEZONE> or 'chronolink list' to get list of time zones");
                println!("Use --help for more information");
            }
        }
    }
}

fn set_timezone(tz_name: &str) {
    if tz_name.parse::<Tz>().is_err() {
        println!("'{}' is not a valid timezone", tz_name);
        println!("Use 'chronolink list' to see all available timezones");
        std::process::exit(1);
    }

    // Get the alarm service manager ready so we can write a parcel with our chosen time zone
    use binder_rust::{Parcel, ServiceManager};
    let service_manager = &mut ServiceManager::new();
    let mut alarm_manager = service_manager.get_service("alarm", "android.app.IAlarmManager");

    let mut parcel = Parcel::empty();
    parcel.write_str16(tz_name);

    // 3 is the opcode for setting the time zone
    let _res = alarm_manager.call(3, &mut parcel);
    println!("Successfully set time zone to \x1b[93m{}\x1b[0m", tz_name);
}
fn list_timezones() {
    let timezone_list = get_timezones();
    let now_utc = Utc::now();
    let mut seen_offsets = HashSet::new();

    let times: Vec<(i64, String, String)> = timezone_list
        .iter()
        .filter_map(|tz_name| {
            if let Ok(tz) = tz_name.parse::<Tz>() {
                let local_time = now_utc.with_timezone(&tz);

                let offset_str = local_time.format("%z").to_string();

                if seen_offsets.contains(&offset_str) {
                    None
                } else {
                    seen_offsets.insert(offset_str);

                    let formatted = local_time.format("%I:%M %p %m/%d/%Y").to_string();

                    let timestamp = now_utc.timestamp();

                    Some((timestamp, formatted, tz_name.to_string()))
                }
            } else {
                None
            }
        })
        .collect();

    let mut times_with_offset: Vec<(i32, String, String)> = Vec::new();
    for (_, formatted, tz_name) in times {
        if let Ok(tz) = tz_name.parse::<Tz>() {
            let local_time = now_utc.with_timezone(&tz);
            let offset_hours = local_time.format("%z").to_string();
            let hours: i32 = offset_hours[1..3].parse().unwrap_or(0);
            let mins: i32 = offset_hours[3..5].parse().unwrap_or(0);
            let offset_secs = if offset_hours.starts_with('-') {
                -(hours * 3600 + mins * 60)
            } else {
                hours * 3600 + mins * 60
            };
            times_with_offset.push((offset_secs, formatted, tz_name));
        }
    }

    times_with_offset.sort_by_key(|(offset, _, _)| *offset);

    for (_, formatted, tz_name) in times_with_offset {
        println!("{} - \x1b[93m{}\x1b[0m", formatted, tz_name);
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
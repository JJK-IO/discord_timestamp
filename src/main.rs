use chrono::{Local, TimeZone};
use chrono_tz::Tz;
use clap::Parser;
use iana_time_zone::{self as itz};

/// Utility to convert a timezone aware date time into a discord time tag <t:timestamp>
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The time you want to get the timestamp for
    time: String,

    /// Date as MM/DD/YYYY, defaults to today
    #[arg(short, long)]
    date: Option<String>,

    /// Timezone as an IANA string... E.G. "America/Denver". Defaults to your local timezone if possible, otherwise ETC/UTC
    #[arg(short, long)]
    timezone: Option<String>,
}

fn main() {
    // Get the command-line arguments
    let app = Cli::parse();
    let date_str: String = match &app.date {
        Some(date) => date.clone(),
        None => Local::now().format("%m/%d/%Y").to_string(),
    };

    // Parse the time argument
    let is_am_pm_time =
        app.time.to_lowercase().ends_with("am") || app.time.to_lowercase().ends_with("pm");

    let (mut hours, minutes): (u32, u32);

    // If it's AM/PM time, convert to 24-hour format
    if is_am_pm_time {
        // Split the string into hour, minutes, and am/pm parts
        let parts: Vec<&str> = app.time.split(':').collect();
        if parts.len() != 2 {
            eprintln!("Invalid time format.");
            return;
        }

        let hour_minute_part = parts[1];
        let hour_str = parts[0];

        // Extract hour and minutes
        hours = hour_str.parse().unwrap_or_else(|_| {
            eprintln!("Invalid hour");
            std::process::exit(1);
        });

        let minutes_str = &hour_minute_part[..hour_minute_part.len() - 2];
        minutes = minutes_str.parse().unwrap_or_else(|_| {
            eprintln!("Invalid minutes");
            std::process::exit(1);
        });

        // Extract AM/PM
        let am_pm_str = &hour_minute_part[hour_minute_part.len() - 2..].to_lowercase();
        let am_pm = match am_pm_str.as_str() {
            "am" | "pm" => am_pm_str,
            _ => {
                eprintln!("Invalid AM/PM");
                std::process::exit(1);
            }
        };

        // Adjust hours if PM
        if am_pm.to_lowercase().ends_with("pm") {
            hours = (hours + 12) % 24;
        }
    } else {
        // Parse the time in military time format
        let parts: Vec<&str> = app.time.split(':').collect();
        if parts.len() != 2 {
            eprintln!("Invalid time format. Please use HH:MM format.");
            return;
        }

        hours = parts[0].parse().unwrap_or_else(|_| {
            eprintln!("Invalid hour");
            std::process::exit(1);
        });
        minutes = parts[1].parse().unwrap_or_else(|_| {
            eprintln!("Invalid minute");
            std::process::exit(1);
        });
    }

    // Parse the date argument
    let date_parts: Vec<&str> = date_str.split('/').collect();
    if date_parts.len() != 3 {
        eprintln!("Invalid date format. Please use MM/DD/YYYY format.");
        return;
    }
    let year: i32 = date_parts[2].parse().unwrap_or_else(|_| {
        eprintln!("Invalid year");
        std::process::exit(1);
    });
    let month: u32 = date_parts[0].parse().unwrap_or_else(|_| {
        eprintln!("Invalid month");
        std::process::exit(1);
    });
    let day: u32 = date_parts[1].parse().unwrap_or_else(|_| {
        eprintln!("Invalid day");
        std::process::exit(1);
    });

    // Parse the timezone argument if provided
    let timezone = match &app.timezone {
        Some(timezone_str) => timezone_str.to_string(),
        None => match itz::get_timezone() {
            Ok(tz) => tz,
            Err(_err) => "Etc/UTC".to_string(),
        },
    };

    // Get the current time in the specified or default timezone
    let tz: Tz = timezone.parse::<Tz>().map_or_else(
        |_| {
            eprintln!("Error parsing timezone, defaulting to UTC.");
            Tz::UTC
        },
        |tz| tz,
    );

    // Combine the date and time components to form a DateTime<Tz> object
    let datetime_with_tz = match tz.with_ymd_and_hms(year, month, day, hours, minutes, 0) {
        chrono::LocalResult::None => {
            eprintln!("Invalid local time representation.");
            std::process::exit(1);
        }
        chrono::LocalResult::Single(datetime_with_tz) => datetime_with_tz,
        chrono::LocalResult::Ambiguous(min, max) => {
            eprintln!(
                "Ambiguous local time representation. Possible range: {} - {}",
                min, max
            );
            std::process::exit(1);
        }
    };

    println!("<t:{}:F>", datetime_with_tz.timestamp());
    println!("<t:{}:R>", datetime_with_tz.timestamp());
}

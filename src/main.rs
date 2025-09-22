use std::fs;
use std::io::BufReader;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::{Context, Result as AnyhowResult};
use clap::Parser as ClapParser;
use chrono::{DateTime, Duration, NaiveDateTime, Timelike, TimeZone, Utc};
use chrono_tz::Tz;
use colored::Colorize;
use ical::IcalParser;
use prettytable::{format, row, Table};

// CLI Args
#[derive(ClapParser, Debug)]
#[command(name = "interview-slot-suggester")]
#[command(about = "Suggests optimal 30-min interview slots from one or more .ics calendars. Gift from a future Flowdesk talent!")]
struct Args {
    /// Path to .ics calendar file(s) (repeat for multiples)
    #[arg(short, long, required = true, num_args = 1..)]
    ics_files: Vec<PathBuf>,
    /// Days ahead to look (default: 7)
    #[arg(short, long, default_value_t = 7)]
    days_ahead: i64,
    /// Start hour (24h, default: 9)
    #[arg(short = 's', long, default_value_t = 9)]
    start_hour: u32,
    /// End hour (24h, default: 18)
    #[arg(short = 'e', long, default_value_t = 18)]
    end_hour: u32,
    /// Buffer minutes for transitions (default: 15)
    #[arg(short = 'b', long, default_value_t = 15)]
    buffer_mins: i64,
    /// Timezone for output and search (IANA, e.g., America/New_York; default: UTC)
    #[arg(short, long, default_value = "UTC")]
    timezone: String,
}

// Helper to parse ICS datetime string (e.g., "19960918T143000Z") to NaiveDateTime (UTC)
fn parse_ics_datetime(s: &str) -> Option<NaiveDateTime> {
    if s.ends_with('Z') {
        let without_z = &s[0..s.len() - 1];
        if without_z.len() == 15 {  // YYYYMMDDTHHMMSS
            let year = &without_z[0..4];
            let month = &without_z[4..6];
            let day = &without_z[6..8];
            let hour = &without_z[9..11];
            let min = &without_z[11..13];
            let sec = &without_z[13..15];
            let formatted = format!("{}-{}-{}T{}:{}:{}Z", year, month, day, hour, min, sec);
            if let Ok(dt) = DateTime::parse_from_rfc3339(&formatted) {
                return Some(dt.with_timezone(&Utc).naive_utc());
            }
        }
    }
    None
}

fn main() -> AnyhowResult<()> {
    let args = Args::parse();

    // Parse timezone
    let tz: Tz = FromStr::from_str(&args.timezone).context("Invalid timezone—use IANA like 'Europe/London' for Adam's base.")?;

    // Start from now +1 day (skip today), in UTC
    let now = Utc::now().naive_utc();
    let start_search = now + Duration::days(1);
    let mut end_search = start_search + Duration::days(args.days_ahead);
    end_search = end_search.date().and_hms_opt(args.end_hour as u32, 59, 59).unwrap_or(now);

    // Collect all event intervals from multiple ICS files (in UTC), skipping past events
    let mut events: Vec<(NaiveDateTime, NaiveDateTime)> = Vec::new();
    for ics_path in &args.ics_files {
        let file = fs::File::open(ics_path).context(format!("Failed to open .ics file: {:?}", ics_path))?;
        let reader = BufReader::new(file);
        let parser = IcalParser::new(reader);

        for calendar in parser {
            let ical = calendar.context("Failed to parse .ics calendar—check format or try exporting again from Google/Outlook.")?;
            for event in ical.events {
                // Find DTSTART and DTEND properties
                let dtstart = event.properties.iter()
                    .find(|prop| prop.name == "DTSTART")
                    .and_then(|prop| prop.value.as_ref());
                let dtend = event.properties.iter()
                    .find(|prop| prop.name == "DTEND")
                    .and_then(|prop| prop.value.as_ref());

                if let (Some(start_str), Some(end_str)) = (dtstart, dtend) {
                    if let (Some(start_dt), Some(end_dt)) = (parse_ics_datetime(start_str), parse_ics_datetime(end_str)) {
                        // Skip if event ends before search starts (past/irrelevant)
                        if end_dt >= start_search {
                            events.push((start_dt, end_dt));
                        }
                    }
                }
            }
        }
    }

    // Generate candidate slots (in UTC)
    let mut candidates: Vec<(NaiveDateTime, i32)> = Vec::new(); // (slot_start_utc, score)
    let slot_duration = Duration::minutes(30);

    let mut current = start_search.date().and_hms_opt(args.start_hour as u32, 0, 0).unwrap_or(now);
    while current <= end_search {
        let slot_end = current + slot_duration;

        // Check for conflicts: overlap with buffered events
        let has_conflict = events.iter().any(|(e_start, e_end)| {
            let buffer_start = *e_start - Duration::minutes(args.buffer_mins);
            let buffer_end = *e_end + Duration::minutes(args.buffer_mins);
            current < buffer_end && slot_end > buffer_start
        });

        if !has_conflict {
            // Score: +1 for morning (9-12 in local time)
            let slot_utc_dt = Utc.from_utc_datetime(&current);
            let local_slot = slot_utc_dt.with_timezone(&tz);
            let score = if local_slot.hour() >= 9 && local_slot.hour() < 12 { 1 } else { 0 };
            candidates.push((current, score));
        }

        current = current + slot_duration;
    }

    // Sort by score desc, then by time asc
    candidates.sort_by(|a: &(NaiveDateTime, i32), b: &(NaiveDateTime, i32)| {
        b.1.cmp(&a.1).then(a.0.cmp(&b.0))
    });

    // Output with pretty table and colors
    let header = format!("Suggested 30-min interview slots (prioritizing mornings) in {}:", args.timezone);
    println!("{}", header.bright_blue().bold());

    if candidates.is_empty() {
        println!("{}", "No free slots found—try adjusting hours, range, or timezone!".yellow());
    } else {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_CLEAN);

        // Add header
        table.add_row(row![
            "Time (Local)".bright_green().bold(),
            "Label".bright_green().bold(),
            "Score".bright_green().bold()
        ]);

        for (slot_utc, score) in candidates.iter().take(5) {
            let slot_naive = *slot_utc;
            let slot_utc_dt = Utc.from_utc_datetime(&slot_naive);
            let local_dt = slot_utc_dt.with_timezone(&tz);
            let time_str = local_dt.format("%Y-%m-%d %H:%M %Z").to_string();
            let priority = if *score == 1 { " (Morning Peak!)" } else { "" };
            let label = if priority.is_empty() {
                String::from("30 mins")
            } else {
                format!("30 mins{}", priority.green())
            };
            let score_str = format!("(Score: {})", score).cyan();

            table.add_row(row![time_str, label, score_str]);
        }

        table.printstd();

        println!("\n{}", "Pick one for your next talent chat—remember, a genuine conversation is the best investment. (Dale Carnegie nod)".magenta().italic());
    }

    Ok(())
}
use std::{
    collections::HashMap,
    fs::{write, File},
    io::{BufRead, BufReader},
};

use chrono::{Days, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime};
use icalendar::{Calendar, Component, Event, EventLike};

fn main() {
    let file = File::open("setup.csv").unwrap();

    let lines = BufReader::new(file).lines();
    let lines: Vec<Vec<String>> = lines
        .map(|l| l.expect("failed reading line"))
        .map(|l| l.split(",").map(|s| s.to_string()).collect())
        .collect();

    let boat = &lines[0][0];
    let start_date = NaiveDate::parse_from_str(lines[0][1].as_str(), "%Y-%m-%d")
        .expect("Failed to parse end date");
    let end_date = NaiveDate::parse_from_str(lines[0][2].as_str(), "%Y-%m-%d")
        .expect("Failed to parse start date");

    let even_week: HashMap<_, _> = [
        ("v", (lines[2][0].as_str(), lines[2][1].as_str())),
        ("f", (lines[2][2].as_str(), lines[2][3].as_str())),
        ("h", (lines[2][4].as_str(), lines[2][5].as_str())),
    ]
    .into_iter()
    .collect();

    let odd_week: HashMap<_, _> = [
        ("f", (lines[3][2].as_str(), lines[3][3].as_str())),
        ("h", (lines[3][4].as_str(), lines[3][5].as_str())),
        ("v", (lines[3][0].as_str(), lines[3][1].as_str())),
    ]
    .into_iter()
    .collect();

    let mut current_date = start_date;
    let mut calendar = Calendar::new();
    let mut is_even = true;

    while current_date <= end_date {
        let current_week = if is_even { &even_week } else { &odd_week };
        for d in &lines[1] {
            let (start, end) = *current_week.get(d.as_str()).unwrap_or_else(|| panic!("Could not find day with {}", d));
            let start_time =
                NaiveTime::parse_from_str(start, "%H:%M").expect("Failed to parse start time");
            let end_time =
                NaiveTime::parse_from_str(end, "%H:%M").expect("Failed to parse end time");

            let end_date = if end_time < start_time {
                current_date.checked_add_days(Days::new(1)).unwrap()
            } else {
                current_date
            };

            let start_dt = NaiveDateTime::new(current_date, start_time)
                .and_local_timezone(Local::now().timezone())
                .unwrap();

            let end_dt = NaiveDateTime::new(end_date, end_time)
                .and_local_timezone(Local::now().timezone())
                .unwrap();

            calendar.push(
                Event::new()
                    .summary(boat)
                    .starts(start_dt.to_utc())
                    .ends(end_dt.to_utc())
                    .done(),
            );

            current_date += Duration::days(1)
        }
        current_date += Duration::days(7);
        is_even = !is_even;
    }

    write("out.ics", calendar.done().to_string()).unwrap();
}

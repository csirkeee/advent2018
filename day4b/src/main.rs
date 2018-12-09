extern crate chrono;
extern crate regex;

use chrono::naive::NaiveDateTime;
use chrono::Timelike;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), Box<Error>> {
    let stdin = io::stdin();

    let mut actions: Vec<(NaiveDateTime, String)> = Vec::new();

    for line_result in stdin.lock().lines() {
        if let Ok(line) = line_result {
            let parts: Vec<&str> = line.split("] ").collect();

            let time = NaiveDateTime::parse_from_str(parts[0], "[%Y-%m-%d %H:%M")?;
            actions.push((time, parts[1].to_string()));
        }
    }

    actions.sort();

    let mut sleep = HashMap::new();

    let guard_number_re = Regex::new(r"[0-9]+").unwrap();
    let mut guard = 0;
    let mut start_time = 0;

    for action in actions {
        let (time, action) = action;

        if action.starts_with("Guard") {
            guard = guard_number_re.find(&action).unwrap().as_str().parse().unwrap();
            if !sleep.contains_key(&guard) {
                sleep.insert(guard, vec![0;60]);
            }
        } else if action.starts_with("falls") {
            start_time = time.minute();
        } else if action.starts_with("wakes") {
            let mins = sleep.get_mut(&guard).unwrap();
            for min in start_time .. time.minute() {
                mins[min as usize] += 1;
            }
        }
    }

    let mut best_ret = 0;
    let mut best_sleeps = 0;

    for (guard, minutes) in sleep {
        for (minute, sleeps) in minutes.iter().enumerate() {
            if *sleeps > best_sleeps {
                best_ret = minute * guard;
                best_sleeps = *sleeps;
                println!("{} = {} x {}", best_ret, minute, guard);
            }
        }
    }

    println!("{}", best_ret);

    Ok(())
}

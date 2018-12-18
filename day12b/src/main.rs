use std::collections::HashSet;
use std::error::Error;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), Box<Error>> {
    let stdin = io::stdin();
    let mut stdin_handle = stdin.lock();

    let mut initial_state_line = String::new();

    stdin_handle.read_line(&mut initial_state_line).ok();

    let mut state = Vec::new();

    for ch in initial_state_line.bytes() {
        if ch == b'.' {
            state.push(false);
        } else if ch == b'#' {
            state.push(true);
        }
    }

    stdin_handle.read_line(&mut String::new()).ok();

    let mut create_rules = HashSet::new();

    for line_result in stdin_handle.lines() {
        if let Ok(line) = line_result {
            let parts: Vec<&str> = line.split(" => ").collect();
            if parts[1].starts_with('#') {
                let mut rule = 0;
                for pattern_ch in parts[0].bytes() {
                    rule *= 2;
                    if pattern_ch == b'#' {
                        rule += 1;
                    }
                }
                create_rules.insert(rule);
            }
        }
    }

    let mut start_position = 0;
    let mut time = 0;
    let mut movement = 0;

    print!(" 0: ");
    for _ in -5..start_position {
        print!(".");
    }
    for &plant in state.iter() {
        print!("{}", if plant { '#' } else { '.' });
    }
    println!();

    loop {
        time += 1;
        let mut new_state = Vec::new();
        let mut pattern = 0;
        let mut started = false;
        let mut new_start_position = 0;

        for (idx, &plant) in state.iter().enumerate() {
            pattern = (pattern * 2) % 32;
            if plant {
                pattern += 1;
            }

            if create_rules.contains(&pattern) {
                if !started {
                    started = true;
                    new_start_position = start_position - 2 + (idx as i32);
                }

                new_state.push(true);
            } else if started {
                new_state.push(false);
            }
        }

        for _ in 0..4 {
            pattern = (pattern * 2) % 32;

            if create_rules.contains(&pattern) {
                new_state.push(true);
            } else if started {
                new_state.push(false);
            }
        }

        while !new_state.last().unwrap() {
            new_state.pop();
        }

        if state.eq(&new_state) {
            movement = new_start_position - start_position;
            state = new_state;
            start_position = new_start_position;
            break;
        } else {
            state = new_state;
            start_position = new_start_position;

            print!("{:2}: ", time);
            for _ in -5..start_position {
                print!(".");
            }
            for &plant in state.iter() {
                print!("{}", if plant { '#' } else { '.' });
            }
            println!();
        }
    }

    let mut score = 0;
    let mut plant_num = 0;
    for (idx, &plant) in state.iter().enumerate() {
        if plant {
            score += start_position + (idx as i32);
            plant_num += 1;
        }
    }

    let final_score = (score as i64)
        + (plant_num as i64) * ((50000000000 as i64) - (time as i64)) * (movement as i64);

    println!("{}", final_score);

    return Ok(());
}

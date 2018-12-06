use std::collections::HashSet;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let mut v = Vec::new();

    for line in stdin.lock().lines() {
        if let Ok(i) = line?.parse() {
            v.push(i);
        }
    }

    println!("{}", find_repeating(&v));

    return Ok(());
}

fn find_repeating(v: &Vec<i32>) -> i32 {
    let mut sum = 0;

    let mut history = HashSet::new();

    loop {
        for change in v {
            sum += change;
            if history.contains(&sum) {
                return sum;
            }
            history.insert(sum);
        }
    }
}
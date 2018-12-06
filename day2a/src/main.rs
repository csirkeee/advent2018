extern crate counter;

use counter::Counter;
use std::collections::HashSet;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let mut double = 0;
    let mut triple = 0;

    for line in stdin.lock().lines() {
        let char_counts = line?.chars().collect::<Counter<_>>();

        let count_values = char_counts.values().collect::<HashSet<_>>();

        if count_values.contains(&2) {
            double += 1;
        }
        if count_values.contains(&3) {
            triple += 1;
        }
    }

    println!("{}", double*triple);

    return Ok(());
}

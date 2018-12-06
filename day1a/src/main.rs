use std::io;
use std::io::BufRead;

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let mut sum = 0;

    for line in stdin.lock().lines() {
        if let Ok(i) = line?.parse::<i32>() {
            sum += i;
        }
    }

    println!("{}", sum);

    return Ok(());
}

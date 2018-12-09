use std::error::Error;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), Box<Error>> {
    let stdin = io::stdin();
    let mut count = 0;

    let mut used = vec![vec![0; 1000]; 1000];

    for line_result in stdin.lock().lines() {
        if let Ok(line) = line_result {
            let parts: Vec<&str> = line.split(&[' ', ',', 'x', ':'][..]).collect();
            let x : usize = parts[2].parse()?;
            let y : usize = parts[3].parse()?;
            let w : usize = parts[5].parse()?;
            let h : usize = parts[6].parse()?;

            for xx in x .. x+w {
                for yy in y .. y+h {
                    used[xx][yy] += 1;
                    if used[xx][yy] == 2 {
                        count += 1;
                    }
                }
            }
        }
    }

    println!("{}", count);

    return Ok(());
}

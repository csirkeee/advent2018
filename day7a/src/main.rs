use std::error::Error;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), Box<Error>> {
    let stdin = io::stdin();

    let mut needs = vec![vec![false; 30]; 30];
    let mut appears = vec![false; 30];
    let mut done = vec![false; 30];

    for line_result in stdin.lock().lines() {
        if let Ok(line) = line_result {
            let parts: Vec<&str> = line.split(' ').collect();
            let req : usize = (parts[1].bytes().next().unwrap() - b'A') as usize;
            let task : usize = (parts[7].bytes().next().unwrap() - b'A') as usize;

            needs[task][req] = true;
            appears[task] = true;
            appears[req] = true;
        }
    }

    loop {
        let mut did = false;
        for i in 0..26 {
            if appears[i] && !done[i] {
                let mut can_do = true;
                for j in 0..26 {
                    if needs[i][j] && !done[j] {
                        can_do = false;
                        break;
                    }
                }

                if can_do {
                    print!("{}", (b'A' + i as u8) as char);
                    done[i] = true;
                    did = true;
                    break;
                }
            }
        }

        if !did {
            return Ok(());
        }
    }
} 
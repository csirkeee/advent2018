use std::error::Error;
use std::io;
use std::io::BufRead;

static WORKERS : i32 = 4;

fn main() -> Result<(), Box<Error>> {
    let stdin = io::stdin();

    let mut needs = vec![vec![false; 30]; 30];
    let mut appears = vec![false; 30];
    let mut started = vec![None; 30];
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

    let mut time = 0;

    loop {
        let mut working = 0;
        let mut all_done = true;

        print!("{}: ", time);

        for i in 0..26 {
            if let Some(start) = started[i] {
                if start + 61 + (i as i32) > time {
                    working += 1;
                    print!("{} ", (b'A' + i as u8) as char);
                } else {
                    done[i] = true;
                }
            }

            if appears[i] && !done[i] {
                all_done = false;
            }
        }

        if all_done {
            println!("All done!");
            return Ok(());
        }

        if working < WORKERS {
            for i in 0..26 {
                if appears[i] && None == started[i] {
                    let mut can_do = true;
                    for j in 0..26 {
                        if needs[i][j] && !done[j] {
                            can_do = false;
                            break;
                        }
                    }

                    if can_do {
                        started[i] = Some(time);
                        working += 1;
                        print!("{} ", (b'A' + i as u8) as char);
                        if working == WORKERS {
                            break;
                        }
                    }
                }
            }
        }

        for _dots in working .. WORKERS {
            print!(". ");
        }

        println!();

        time += 1;
    }
} 
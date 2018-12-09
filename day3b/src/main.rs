use std::error::Error;
use std::io;
use std::io::BufRead;

type Claim = (usize, usize, usize, usize, usize);

fn main() -> Result<(), Box<Error>> {
    let stdin = io::stdin();

    let mut claims : Vec<Claim> = Vec::new();

    let mut used = vec![vec![0; 1000]; 1000];

    for line_result in stdin.lock().lines() {
        if let Ok(line) = line_result {
            let parts: Vec<&str> = line.split(&['#', ' ', ',', 'x', ':'][..]).collect();
            let id : usize = parts[1].parse()?;
            let x : usize = parts[3].parse()?;
            let y : usize = parts[4].parse()?;
            let w : usize = parts[6].parse()?;
            let h : usize = parts[7].parse()?;

            for xx in x .. x+w {
                for yy in y .. y+h {
                    used[xx][yy] += 1;
                }
            }

            claims.push((id, x, y, w, h));
        }
    }

    println!("{}", find_good_claim(&claims, &used).unwrap());

    Ok(())
}

fn find_good_claim(claims: & Vec<Claim>, used: &Vec<Vec<i32>>) -> Result<usize, ()> {
    for (id, x, y, w, h) in claims {
        if is_good_claim(&used, *x, *y, *w, *h) {
            return Ok(*id);
        }
    }

    Err(())
}

fn is_good_claim(used: &Vec<Vec<i32>>, x: usize, y: usize, w: usize, h: usize) -> bool {
    for xx in x .. x+w {
        for yy in y .. y+h {
            if used[xx][yy] > 1 {
                return false;
            }
        }
    }

    true
}

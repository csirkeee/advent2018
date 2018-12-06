use std::io;
use std::io::BufRead;

fn main() -> Result<(), ()> {
    let stdin = io::stdin();

    let ids : Vec<String> = stdin.lock().lines().filter_map(|s| s.ok()).collect();

    println!("{}", find_similar_id(&ids)?);

    Ok(())
}

fn find_similar_id(ids: &Vec<String>) -> Result<String, ()> {
    for id1 in ids {
        for id2 in ids {
            let mut diff = 0;
            let mut common_part = String::new();
            for (i, char1) in id1.chars().enumerate() {
                if char1 == id2.chars().nth(i).unwrap() {
                    common_part.push(char1);
                } else {
                    diff += 1;
                }
            }

            if diff == 1 {
                return Ok(common_part);
            }
        }
    }
    Err(())
}

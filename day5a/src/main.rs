use std::error::Error;
use std::io;
use std::io::Read;

fn main() -> Result<(), Box<Error>> {
    let stdin = io::stdin();

    let mut chain : Vec<u8> = stdin.lock().bytes().map(|b| b.unwrap()).collect();
    let mut new_chain = Vec::new();

    let mut change = true;
    while change {
        change = false;
        {
            let mut iter = chain.iter();
            let mut cur_char = iter.next();
            while let Some(char_val) = cur_char {
                let mut next_char = iter.next();
                if let Some(next_char_val) = next_char {
                    if char_val.is_ascii_lowercase() && char_val.to_ascii_uppercase() == *next_char_val {
                        next_char = iter.next();
                        change = true;
                    } else if char_val.is_ascii_uppercase() && char_val.to_ascii_lowercase() == *next_char_val {
                        next_char = iter.next();
                        change = true;
                    } else {
                        new_chain.push(*char_val)
                    }
                } else {
                    new_chain.push(*char_val)
                }

                cur_char = next_char;
            }
        }
        println!("{} to {}", chain.len(), new_chain.len());

        chain = new_chain;
        new_chain = Vec::new();
    }

    println!("{}", chain.len());

    Ok(())
}
use std::error::Error;
use std::io;
use std::io::BufRead;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Light {
    coords: (i32, i32),
    speed: (i32, i32),
}

fn main() -> Result<(), Box<Error>> {
    let stdin = io::stdin();

    let mut lights = Vec::new();

    for line_result in stdin.lock().lines() {
        if let Ok(line) = line_result {
            let parts: Vec<&str> = line.split(&['<', '>', ','][..]).collect();
            let x = parts[1].trim().parse()?;
            let y = parts[2].trim().parse()?;
            let vx = parts[4].trim().parse()?;
            let vy = parts[5].trim().parse()?;

            lights.push(Light {
                coords: (x, y),
                speed: (vx, vy),
            });
        }
    }

    let mut x_span = 10000000;
    let mut y_span = 10000000;
    let mut time = 0;

    loop {
        let mut min_x = lights.get(0).unwrap().coords.0;
        let mut max_x = lights.get(0).unwrap().coords.0;
        let mut min_y = lights.get(0).unwrap().coords.1;
        let mut max_y = lights.get(0).unwrap().coords.1;

        for l in &lights {
            min_x = i32::min(min_x, l.coords.0);
            max_x = i32::max(max_x, l.coords.0);
            min_y = i32::min(min_y, l.coords.1);
            max_y = i32::max(max_y, l.coords.1);
        }

        let new_x_span = max_x - min_x;
        let new_y_span = max_y - min_y;

        if new_x_span > x_span && new_y_span > y_span {
            break;
        }

        x_span = new_x_span;
        y_span = new_y_span;

        println!("{}:", time);
        if x_span < 100 && y_span < 100 {
            for y in min_y..max_y + 1 {
                for x in min_x..max_x + 1 {
                    let mut appears = false;
                    for l in &lights {
                        if l.coords.0 == x && l.coords.1 == y {
                            appears = true;
                            break;
                        }
                    }

                    if appears {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!();
            }
        }

        for l in &mut lights {
            l.coords.0 += l.speed.0;
            l.coords.1 += l.speed.1;
        }

        time += 1;
    }

    return Ok(());
}

use std::error::Error;
use std::io;
use std::io::BufRead;

type Point = (i32, i32);

fn main() -> Result<(), Box<Error>> {
    let stdin = io::stdin();

    let mut points: Vec<Point> = Vec::new();

    for line_result in stdin.lock().lines() {
        if let Ok(line) = line_result {
            let parts: Vec<&str> = line.split(", ").collect();

            points.push((parts[0].parse().unwrap(), parts[1].parse().unwrap()))
        }
    }

    let mut num = 0;

    for x in -100..600 {
        for y in -100..600 {
            let mut total = 0;

            for point in &points {
                total += manhattan(*point, (x, y));
            }

            if total < 10000 {
                num += 1;
            }
        }
    }

    println!("{}", num);

    Ok(())
}

fn manhattan(a: Point, b: Point) -> i32 {
    let (x1, y1) = a;
    let (x2, y2) = b;

    (x2 - x1).abs() + (y2 - y1).abs()
}
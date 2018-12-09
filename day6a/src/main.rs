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

    let mut on_edge = vec![false; points.len()];
    let mut sizes = vec![0; points.len()];

    for x in -100..600 {
        for y in -100..600 {
            let mut min_dist = 1000;
            let mut closest = 0;
            let mut double = false;

            for (idx, point) in points.iter().enumerate() {
                let dist = manhattan(*point, (x, y));
                if dist < min_dist {
                    double = false;
                    min_dist = dist;
                    closest = idx;
                } else if dist == min_dist {
                    double = true;
                }
            }

            if !double {
                sizes[closest] += 1;

                if x == -100 || y == -100 || x == 599 || y == 599 {
                    on_edge[closest] = true;
                }
            }
        }
    }

    let mut biggest_size = 0;

    for (idx, size) in sizes.iter().enumerate() {
        if !on_edge[idx] && *size > biggest_size {
            biggest_size = *size;
        }
    }

    println!("{}", biggest_size);

    Ok(())
}

fn manhattan(a: Point, b: Point) -> i32 {
    let (x1, y1) = a;
    let (x2, y2) = b;

    (x2 - x1).abs() + (y2 - y1).abs()
}
const SIZE: usize = 300;
const SERIAL: i32 = 18;

fn main() {
    let mut power = [[0; SIZE]; SIZE];

    for y in 0..SIZE {
        for x in 0..SIZE {
            let rack_id = (x as i32) + 11;
            power[y][x] = ((((rack_id * ((y as i32) + 1)) + SERIAL) * rack_id) % 1000) / 100 - 5;
        }
    }

    let mut max_power = -1000;
    let mut corner = (0, 0, 0);

    for c_y in 0..SIZE - 1 {
        println!("{}", c_y);
        for c_x in 0..SIZE - 1 {
            let mut sum = 0;
            for square_size in 1..SIZE {
                if c_x + square_size > SIZE || c_y + square_size > SIZE {
                    continue;
                }

                for y in c_y..c_y + square_size {
                    sum += power[y][c_x + square_size - 1];
                }

                for x in c_x..c_x + square_size - 1 {
                    sum += power[c_y + square_size - 1][x];
                }

                if sum > max_power {
                    max_power = sum;
                    corner = (c_x, c_y, square_size);
                }
            }
        }
    }

    println!("{},{},{}", corner.0 + 1, corner.1 + 1, corner.2);
}

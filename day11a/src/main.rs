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
    let mut corner = (0, 0);

    for c_y in 0..SIZE - 3 {
        for c_x in 0..SIZE - 3 {
            let mut sum = 0;
            for y in c_y..c_y + 3 {
                for x in c_x..c_x + 3 {
                    sum += power[y][x];
                }
            }

            if sum > max_power {
                max_power = sum;
                corner = (c_x, c_y);
            }
        }
    }

    println!("{}, {}", corner.0 + 1, corner.1 + 1);
}

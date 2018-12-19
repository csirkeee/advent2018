use std::error::Error;
use std::io;
use std::io::BufRead;

#[derive(Copy, Clone, Debug)]
enum Dir {
    N,
    W,
    S,
    E,
}
use crate::Dir::*;

fn to_left(dir: Dir) -> Dir {
    match dir {
        N => W,
        W => S,
        S => E,
        E => N,
    }
}

fn to_right(dir: Dir) -> Dir {
    match dir {
        N => E,
        W => N,
        S => W,
        E => S,
    }
}

#[derive(Copy, Clone, Debug)]
enum NextTurn {
    Left,
    Forward,
    Right,
}
use crate::NextTurn::*;

#[derive(Copy, Clone, Debug)]
struct Cart {
    dir: Dir,
    next_turn: NextTurn,
    moved: bool,
}

enum Track {
    Empty,
    Vertical,
    Horizontal,
    Intersection,
    NWCorner,
    NECorner,
}
use crate::Track::*;

struct Field {
    track: Track,
    cart: Option<Cart>,
}

fn print_map(map: &Vec<Vec<Field>>) {
    for line in map {
        for field in line {
            match field {
                Field { cart: Some(Cart { dir: N, .. }), .. } => print!("{}", '^'),
                Field { cart: Some(Cart { dir: S, .. }), .. } => print!("{}", 'v'),
                Field { cart: Some(Cart { dir: W, .. }), .. } => print!("{}", '<'),
                Field { cart: Some(Cart { dir: E, .. }), .. } => print!("{}", '>'),
                Field { cart: None, track: Empty } => print!("{}", ' '),
                Field { cart: None, track: Vertical } => print!("{}", '|'),
                Field { cart: None, track: Horizontal } => print!("{}", '-'),
                Field { cart: None, track: Intersection } => print!("{}", '+'),
                Field { cart: None, track: NWCorner } => print!("{}", '/'),
                Field { cart: None, track: NECorner } => print!("{}", '\\'),
            }
        }
        println!();
    }

    println!();
}

fn main() -> Result<(), Box<Error>> {
    let stdin = io::stdin();

    let mut map = Vec::new();

    for line in stdin.lock().lines() {
        let mut map_line = Vec::new();

        for ch in line.unwrap().bytes() {
            if ch == b' ' {
                map_line.push(Field { track: Empty, cart: None });
            } else if ch == b'|' {
                map_line.push(Field { track: Vertical, cart: None });
            } else if ch == b'-' {
                map_line.push(Field { track: Horizontal, cart: None });
            } else if ch == b'+' {
                map_line.push(Field { track: Intersection, cart: None });
            } else if ch == b'/' {
                map_line.push(Field { track: NWCorner, cart: None });
            } else if ch == b'\\' {
                map_line.push(Field { track: NECorner, cart: None });
            } else if ch == b'^' {
                map_line.push(Field {
                    track: Vertical,
                    cart: Some(Cart {
                        dir: N,
                        next_turn: Left,
                        moved: false,
                    }),
                });
            } else if ch == b'v' {
                map_line.push(Field {
                    track: Vertical,
                    cart: Some(Cart {
                        dir: S,
                        next_turn: Left,
                        moved: false,
                    }),
                });
            } else if ch == b'<' {
                map_line.push(Field {
                    track: Horizontal,
                    cart: Some(Cart {
                        dir: W,
                        next_turn: Left,
                        moved: false,
                    }),
                });
            } else if ch == b'>' {
                map_line.push(Field {
                    track: Horizontal,
                    cart: Some(Cart {
                        dir: E,
                        next_turn: Left,
                        moved: false,
                    }),
                });
            }
        }

        map.push(map_line);
    }

    //    print_map(&map);

    let height = map.len();
    let width = map[0].len();

    'tick: loop {
        let mut cart_num = 0;
        for y in 0..height {
            for x in 0..width {
                if let Field { cart: Some(ref mut cart), .. } = map[y][x] {
                    cart.moved = false;
                }
            }
        }

        for y in 0..height {
            for x in 0..width {
                if let Field { cart: Some(cart), .. } = map[y][x] {
                    if cart.moved {
                        continue;
                    }

                    cart_num += 1;

                    let (new_y, new_x) = match cart {
                        Cart { dir: N, .. } => (y - 1, x),
                        Cart { dir: S, .. } => (y + 1, x),
                        Cart { dir: W, .. } => (y, x - 1),
                        Cart { dir: E, .. } => (y, x + 1),
                    };

                    match map[new_y][new_x] {
                        Field { cart: Some(hit_cart), .. } => {
                            cart_num -= 1;
                            if hit_cart.moved {
                                cart_num -= 1;
                            }
                            map[new_y][new_x].cart = None;
                            println!("Collision: {},{}", new_x, new_y);
                        }
                        Field { cart: None, track: Empty } => panic!(),
                        Field { cart: None, track: Vertical } => map[new_y][new_x].cart = Some(cart),
                        Field { cart: None, track: Horizontal } => map[new_y][new_x].cart = Some(cart),
                        Field { cart: None, track: NWCorner } => match cart.dir {
                            N => map[new_y][new_x].cart = Some(Cart { dir: E, ..cart }),
                            S => map[new_y][new_x].cart = Some(Cart { dir: W, ..cart }),
                            E => map[new_y][new_x].cart = Some(Cart { dir: N, ..cart }),
                            W => map[new_y][new_x].cart = Some(Cart { dir: S, ..cart }),
                        },
                        Field { cart: None, track: NECorner } => match cart.dir {
                            N => map[new_y][new_x].cart = Some(Cart { dir: W, ..cart }),
                            S => map[new_y][new_x].cart = Some(Cart { dir: E, ..cart }),
                            E => map[new_y][new_x].cart = Some(Cart { dir: S, ..cart }),
                            W => map[new_y][new_x].cart = Some(Cart { dir: N, ..cart }),
                        },
                        Field { cart: None, track: Intersection } => match cart.next_turn {
                            Left => {
                                map[new_y][new_x].cart = Some(Cart {
                                    dir: to_left(cart.dir),
                                    next_turn: Forward,
                                    ..cart
                                })
                            }
                            Forward => {
                                map[new_y][new_x].cart = Some(Cart {
                                    dir: cart.dir,
                                    next_turn: Right,
                                    ..cart
                                })
                            }
                            Right => {
                                map[new_y][new_x].cart = Some(Cart {
                                    dir: to_right(cart.dir),
                                    next_turn: Left,
                                    ..cart
                                })
                            }
                        },
                    }

                    if let Field { cart: Some(ref mut cart), .. } = map[new_y][new_x] {
                        cart.moved = true;
                    }

                    map[y][x].cart = None;
                }
            }
        }

        if cart_num < 2 {
            break;
        }

        //        print_map(&map);
    }

    let (mut cart_x, mut cart_y) = (0, 0);

    'walk: for y in 0..height {
        for x in 0..width {
            if let Field { cart: Some(..), .. } = map[y][x] {
                cart_x = x;
                cart_y = y;
                break 'walk;
            }
        }
    }

    println!("{},{}", cart_x, cart_y);

    Ok(())
}

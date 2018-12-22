use std::collections::HashSet;
use std::error::Error;
use std::io;
use std::io::BufRead;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Faction {
    Elf,
    Goblin,
}
use crate::Faction::*;

#[derive(Copy, Clone, Debug)]
struct Unit {
    faction: Faction,
    health: i32,
    pos: (usize, usize), // y, x
    moved: bool,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Field {
    Empty,
    Wall,
    Unit(usize),
}
use crate::Field::*;

const MOVES: [(i32, i32); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];
const GOBLIN_ATTACK: i32 = 3;

fn print_map(map: &Vec<Vec<Field>>, units: &Vec<Unit>) {
    for line in map {
        for field in line {
            match *field {
                Empty => print!("{}", '.'),
                Wall => print!("{}", '#'),
                Unit(idx) => match units[idx] {
                    Unit { faction: Goblin, .. } => print!("{}", 'G'),
                    Unit { faction: Elf, .. } => print!("{}", 'E'),
                },
            }
        }
        println!();
    }

    println!();
}

fn read_map() -> (Vec<Vec<Field>>, Vec<Unit>) {
    let stdin = io::stdin();

    let mut map = Vec::new();
    let mut units = Vec::new();

    for (y, line) in stdin.lock().lines().enumerate() {
        let mut map_line = Vec::new();

        for (x, ch) in line.unwrap().bytes().enumerate() {
            if ch == b'.' {
                map_line.push(Empty);
            } else if ch == b'#' {
                map_line.push(Wall);
            } else if ch == b'G' {
                let new_idx = units.len();
                units.push(Unit {
                    faction: Goblin,
                    health: 200,
                    pos: (y, x),
                    moved: false,
                });
                map_line.push(Unit(new_idx));
            } else if ch == b'E' {
                let new_idx = units.len();
                units.push(Unit {
                    faction: Elf,
                    health: 200,
                    pos: (y, x),
                    moved: false,
                });
                map_line.push(Unit(new_idx));
            }
        }

        map.push(map_line);
    }

    (map, units)
}

fn choose_target(
    from: (usize, usize),
    map: &Vec<Vec<Field>>,
    potential_targets: &Vec<(usize, usize)>,
) -> Option<(usize, usize)> {
    let mut visited = HashSet::new();
    let mut current_steps = vec![from];
    let mut visited_targets = Vec::new();

    if potential_targets.contains(&from) {
        return Some(from);
    }

    while visited_targets.is_empty() && !current_steps.is_empty() {
        let mut next_steps = Vec::new();

        for step in current_steps {
            for &dir in MOVES.iter() {
                let s = ((dir.0 + step.0 as i32) as usize, (dir.1 + step.1 as i32) as usize);
                if map[s.0][s.1] == Empty && !visited.contains(&s) {
                    visited.insert(s);
                    next_steps.push(s);
                    if potential_targets.contains(&s) {
                        visited_targets.push(s);
                    }
                }
            }
        }

        current_steps = next_steps;
    }

    // Chosen target is the minimal with (y, x) coordinate ordering
    visited_targets.iter().min().cloned()
}

fn choose_move(from: (usize, usize), map: &Vec<Vec<Field>>, potential_targets: &Vec<(usize, usize)>) -> (usize, usize) {
    let target = choose_target(from, map, potential_targets);

    if let Some(target) = target {
        let mut neighbors = Vec::new();

        for &dir in MOVES.iter() {
            let n = ((dir.0 + from.0 as i32) as usize, (dir.1 + from.1 as i32) as usize);
            if map[n.0][n.1] == Empty {
                neighbors.push(n);
            }
        }

        choose_target(target, map, &neighbors).unwrap()
    } else {
        from
    }
}

fn currently_hits(idx: usize, map: &Vec<Vec<Field>>, units: &Vec<Unit>) -> Option<usize> {
    let unit_copy = units[idx];

    let mut enemy_neighbors = Vec::new();

    for &dir in MOVES.iter() {
        let (n_y, n_x) = (
            (dir.0 + unit_copy.pos.0 as i32) as usize,
            (dir.1 + unit_copy.pos.1 as i32) as usize,
        );
        if let Unit(other_idx) = map[n_y][n_x] {
            if unit_copy.faction != units[other_idx].faction {
                enemy_neighbors.push((units[other_idx].health, units[other_idx].pos, other_idx));
            }
        }
    }

    enemy_neighbors.iter().min().map(|enemy| enemy.2)
}

fn run_simulation(mut map: Vec<Vec<Field>>, mut units: Vec<Unit>, elf_attack: i32) -> Option<(i32, i32)> {
    let height = map.len();
    let width = map[0].len();
    let mut ticks = 0;

    'tick: loop {
        for y in 0..height {
            for x in 0..width {
                if let Unit(idx) = map[y][x] {
                    let unit_copy = units[idx];

                    if unit_copy.moved {
                        continue;
                    }

                    // Check whether we need to move, or already next to enemy
                    let mut current_hit = currently_hits(idx, &map, &units);
                    if current_hit.is_none() {
                        // Collect potential targets
                        let mut found_enemy = false;
                        let mut target_locations = Vec::new();
                        for potential_target in &units {
                            if potential_target.faction != unit_copy.faction && potential_target.health > 0 {
                                found_enemy = true;
                                for &dir in MOVES.iter() {
                                    let (n_y, n_x) = (
                                        (dir.0 + potential_target.pos.0 as i32) as usize,
                                        (dir.1 + potential_target.pos.1 as i32) as usize,
                                    );
                                    if map[n_y][n_x] == Empty {
                                        target_locations.push((n_y, n_x))
                                    }
                                }
                            }
                        }

                        if !found_enemy {
                            break 'tick;
                        }

                        let new_pos = choose_move(unit_copy.pos, &map, &target_locations);

                        map[y][x] = Empty;
                        map[new_pos.0][new_pos.1] = Unit(idx);

                        units[idx] = Unit {
                            pos: new_pos,
                            moved: true,
                            ..unit_copy
                        };

                        current_hit = currently_hits(idx, &map, &units);
                    }

                    if let Some(hit) = current_hit {
                        let attack = if unit_copy.faction == Elf {
                            elf_attack
                        } else {
                            GOBLIN_ATTACK
                        };
                        if units[hit].health > attack {
                            units[hit].health -= attack;
                        } else {
                            if units[hit].faction == Elf {
                                return None;
                            }
                            units[hit].health = 0;
                            let dead_pos = units[hit].pos;
                            map[dead_pos.0][dead_pos.1] = Empty;
                        }
                    }
                }
            }
        }

        for unit in &mut units {
            unit.moved = false;
        }

        ticks += 1;
    }

    print_map(&map, &units);

    let total_health: i32 = units.iter().map(|unit| unit.health).sum();

    Some((ticks, total_health))
}

fn main() -> Result<(), Box<Error>> {
    let (map, units) = read_map();

    let mut elf_attack = 3;

    loop {
        println!("Elf attack {}", elf_attack);

        let result = run_simulation(map.clone(), units.clone(), elf_attack);

        if let Some((ticks, total_health)) = result {
            println!("Elves win! {} * {} = {}", ticks, total_health, ticks * total_health);
            break;
        } else {
            println!("Elf died :(");
        }

        elf_attack += 1;
    }

    Ok(())
}

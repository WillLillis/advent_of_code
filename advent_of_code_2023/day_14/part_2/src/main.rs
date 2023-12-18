use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum MapItem {
    RoundRock, // O
    CubeRock,  // #
    Empty,     // .
}

impl Display for MapItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::RoundRock => 'O',
            Self::CubeRock => '#',
            Self::Empty => '.',
        };

        write!(f, "{}", c)?;
        Ok(())
    }
}

type Map = Vec<Vec<MapItem>>;

#[allow(dead_code)]
fn print_map(map: &Map) {
    for row in map.iter() {
        for c in row.iter() {
            print!("{}", c);
        }
        println!();
    }
}

fn get_map(input: &str) -> Map {
    input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| match c {
                    'O' => MapItem::RoundRock,
                    '#' => MapItem::CubeRock,
                    '.' => MapItem::Empty,
                    _ => {
                        panic!("Invalid input character.");
                    }
                })
                .collect::<Vec<MapItem>>()
        })
        .collect::<Map>()
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    North,
    South,
    East,
    West,
}

fn roll_map(map: &mut Map, roll_dir: Dir) {
    let map_len = map.len();
    let map_width = map[0].len();
    match roll_dir {
        Dir::North => {
            for roll_row in 1..map_len {
                for col in 0..map_width {
                    if map[roll_row][col] == MapItem::RoundRock {
                        let mut rolled_to_row = roll_row;
                        loop {
                            rolled_to_row -= 1;
                            if map[rolled_to_row][col] == MapItem::Empty {
                                map[rolled_to_row][col] = MapItem::RoundRock;
                                map[rolled_to_row + 1][col] = MapItem::Empty;
                            } else {
                                break;
                            }

                            if rolled_to_row == 0 {
                                break;
                            }
                        }
                    }
                }
            }
        }
        Dir::East => {
            for roll_col in (0..map_width - 1).rev() {
                for row in 0..map_len {
                    if map[row][roll_col] == MapItem::RoundRock {
                        let mut rolled_to_col = roll_col;
                        loop {
                            rolled_to_col += 1;
                            if map[row][rolled_to_col] == MapItem::Empty {
                                map[row][rolled_to_col] = MapItem::RoundRock;
                                map[row][rolled_to_col - 1] = MapItem::Empty;
                            } else {
                                break;
                            }

                            if rolled_to_col == map_width - 1 {
                                break;
                            }
                        }
                    }
                }
            }
        }
        Dir::South => {
            for roll_row in (0..map_len - 1).rev() {
                for col in 0..map_width {
                    if map[roll_row][col] == MapItem::RoundRock {
                        let mut rolled_to_row = roll_row;
                        loop {
                            rolled_to_row += 1;
                            if map[rolled_to_row][col] == MapItem::Empty {
                                map[rolled_to_row][col] = MapItem::RoundRock;
                                map[rolled_to_row - 1][col] = MapItem::Empty;
                            } else {
                                break;
                            }

                            if rolled_to_row == map_len - 1 {
                                break;
                            }
                        }
                    }
                }
            }
        }
        Dir::West => {
            for roll_col in 1..map_width {
                for row in 0..map_len {
                    if map[row][roll_col] == MapItem::RoundRock {
                        let mut rolled_to_col = roll_col;
                        loop {
                            rolled_to_col -= 1;
                            if map[row][rolled_to_col] == MapItem::Empty {
                                map[row][rolled_to_col] = MapItem::RoundRock;
                                map[row][rolled_to_col + 1] = MapItem::Empty;
                            } else {
                                break;
                            }

                            if rolled_to_col == 0 {
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
}

fn roll_cycle(map: &mut Map) {
    roll_map(map, Dir::North);
    roll_map(map, Dir::West);
    roll_map(map, Dir::South);
    roll_map(map, Dir::East);
}

fn calc_load(map: &Map, beam_dir: Dir) -> usize {
    let width = map[0].len();
    let n_rows = map.len();
    let mut sum = 0usize;

    match beam_dir {
        Dir::North => {
            for row in 0..map.len() {
                for col in 0..width {
                    if map[row][col] == MapItem::RoundRock {
                        sum += n_rows - row;
                    }
                }
            }
        }
        _ => {
            todo!()
        }
    }

    sum
}

fn main() {
    let input = std::fs::read_to_string("../input").expect("Failed to read the input file.");
    let mut map = get_map(&input);
    let mut map_states = HashMap::new();
    const TOTAL_CYCLES: usize = 1_000_000_000;

    let mut cycle_len = None;
    let mut curr_cycle = 0;

    for cycle_num in 0..TOTAL_CYCLES {
        roll_cycle(&mut map);
        if let Some(cycle_start) = map_states.get(&map) {
            cycle_len = Some(cycle_num - cycle_start);
            curr_cycle = cycle_num;
            break;
        } else {
            map_states.insert(map.clone(), cycle_num);
        }
    }

    if let Some(len) = cycle_len {
        let n_remaining = (TOTAL_CYCLES - curr_cycle) % len;
        for _ in 0..n_remaining - 1 {
            roll_cycle(&mut map);
        }
    }

    let load = calc_load(&map, Dir::North);
    println!("Load: {}", load);
}

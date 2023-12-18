use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    let width = map[0].len();
    match roll_dir {
        Dir::North => {
            for roll_row in 1..map.len() {
                for col in 0..width {
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
        _ => {
            todo!()
        }
    }
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

    roll_map(&mut map, Dir::North);
    let load = calc_load(&map, Dir::North);
    println!("Load: {}", load);
}

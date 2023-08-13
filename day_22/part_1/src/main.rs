use std::{fs, cmp};

enum Rotation {
    Clockwise,
    CounterClockwise
}

enum Move {
    Move(usize),
    Turn(Rotation)

}

enum Facing {
    Right, // >
    Down, // v
    Left, // <
    Up // ^
}

impl Facing {
    fn turn(&mut self, way: Rotation) {
        match way {
            Rotation::Clockwise => {
                Facing::turn_clockwise(self);
            },
            Rotation::CounterClockwise => {
                Facing::turn_counterclockwise(self);
            }
        }
    }

    fn turn_clockwise(&mut self) {
        *self = match self {
            Facing::Right => Facing::Down,
            Facing::Down => Facing::Left,
            Facing::Left => Facing::Up,
            Facing::Up => Facing::Right
        };
    }

    fn turn_counterclockwise(&mut self) {
        *self = match self {
            Facing::Right => Facing::Up,
            Facing::Down => Facing::Right,
            Facing::Left => Facing::Down,
            Facing::Up => Facing::Left
        };
    }
}

struct Pos {
   orientation: Facing,
   row: usize,
   row_idx: usize
}

fn get_notes(file_name: &str) -> (Vec<Vec<char>>, String) {
    let input = fs::read_to_string(file_name)
        .expect("Failed to read the input file");

    // need to get count and add padding
    let mut map:Vec<Vec<char>> = input
        .lines()
        .take_while(|s| !s.is_empty())
        .map(|s| s.chars().collect())
        .collect();

    // padding added, not sure if this will be helpful or not
    let max_len = map.iter().fold(0, |max, x| cmp::max(max, x.len()));
    
    for i in 0..map.len() {
        let len = map[i].len();
        if len < max_len {
            map[i].append(&mut vec![' '; max_len - len]);
        }
    
    }


    println!("Max len: {:?}", max_len);

    for thing in &map {
        println!("{:?}", thing);
    }

    let path = String::from(input.lines()
                            .into_iter()
                            .skip_while(|s| !s.is_empty())
                            .skip(1)
                            .collect::<String>());

    println!("\n\n\n{}", path);

    return (map, path);
}

// find the first valid index in a given row
fn row_first_idx(map: &Vec<Vec<char>>, row: usize, from_left: bool) -> usize {


    0
}

fn col_first_idx(map: &Vec<Vec<char>>, col: usize, from_top: bool) -> usize {


    0
}

fn make_move(map: &Vec<Vec<char>>, mut pos: Pos, next_move: Move) -> Pos {
    match next_move {
        Move::Move(num) => {
            match pos.orientation {
                Facing::Right => {
                    let curr_row = pos.row;
                    let mut curr_idx = pos.row_idx;
                    for incr in 1..=num {
                        // first check if we're running off the right edge
                        if curr_idx + incr >= map[curr_row].len() {
                            // if we can wrap around do it
                            
                            // otherwise we're done
                            
                            break;
                        }
                        // next check if we're blocked
                        if map[curr_row][curr_idx + incr] == '#' {
                            pos.row_idx = curr_idx + incr;
                            break;
                        } else if map[curr_row][curr_idx + incr] == ' ' { // or if we can wrap around
                        
                        } else if map[curr_row][curr_idx + incr] == ',' { // or if we can just move one

                        } else {
                            panic!("Unexpected move!");
                        }
                    }
                    
                },
                Facing::Down => {

                    
                },
                Facing::Left => {

                    
                },
                Facing::Up => {
                    
                }
            }

            pos
        },
        Move::Turn(dir) => {
            pos.orientation.turn(dir);
            pos
        }
    }
    
}


fn main() {

    let (map, path) = get_notes("test_input.txt"); 
    
}

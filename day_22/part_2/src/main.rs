use std::fs;
use std::iter::Peekable;

const FACE_WIDTH: usize = 50;
const FACE_LEN: usize = 50;

#[derive(Debug)]
enum Rotation {
    Clockwise,
    CounterClockwise
}

#[derive(Debug)]
enum Move {
    Move(usize),
    Turn(Rotation)
}

impl Move {
    fn turn_move(input: char) -> Self {
        match input {
            'L' => Move::Turn(Rotation::CounterClockwise),
            'R' => Move::Turn(Rotation::Clockwise),
            _ => {
                panic!("Invalid turn character!");
            }
        }
    }

}

#[derive(Debug)]
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

#[derive(Debug)]
struct Pos {
   orientation: Facing,
   cube_face: usize,
   row: usize,
   row_idx: usize
}

// Do we want to back through and generalize this?
    // get non-empty length of each 50 block of lines and determine number of
    // rows from that
    // somehow insert into the maps Vec<Vec<Vec<char>>> in a general manner???
fn get_notes(file_name: &str) -> (Vec<Vec<Vec<char>>>, String) {
    let input = fs::read_to_string(file_name)
        .expect("Failed to read the input file");

    let map: Vec<Vec<char>> = input
        .lines()
        .take_while(|s| !s.is_empty())
        .map(|s| s.chars().collect())
        .collect();

    // could come up with a general solution, but let's just hard code this for now...

    let mut maps: Vec<Vec<Vec<char>>> = Vec::new(); 
    maps.push(Vec::new()); // Face 1
    maps.push(Vec::new()); // Face 2
    
    let block = map.clone().into_iter().take(FACE_LEN);
    for line in block {
        let line = line.iter().skip_while(|c| **c == ' ');
        
        maps[0].push(
            line
            .clone()
            .take(FACE_WIDTH)
            .map(|c| *c)
            .collect());
        
        maps[1].push(
            line
            .skip(FACE_WIDTH)
            .take(FACE_WIDTH)
            .map(|c| *c)
            .collect());
    }

    maps.push(Vec::new()); // Face 3

    let block = map.clone().into_iter().skip(FACE_LEN).take(FACE_LEN);
    for line in block {
        let line = line.iter().skip_while(|c| **c == ' ');

        maps[2].push(
            line
            .take(FACE_WIDTH)
            .map(|c| *c)
            .collect());
    }

    maps.push(Vec::new()); // Face 4
    maps.push(Vec::new()); // Face 5

    let block = map.clone().into_iter().skip(2 * FACE_LEN).take(FACE_LEN);
    for line in block {
        let line = line.iter().skip_while(|c| **c == ' ');

        maps[3].push(
            line
            .clone()
            .take(FACE_WIDTH)
            .map(|c| *c)
            .collect());

        maps[4].push(
            line
            .skip(FACE_WIDTH)
            .take(FACE_WIDTH)
            .map(|c| *c)
            .collect());
    }

    maps.push(Vec::new()); // Face 6
    
    let block = map.clone().into_iter().skip(3 * FACE_LEN) .take(FACE_LEN);
    for line in block {
        let line = line.iter().skip_while(|c| **c == ' ');

        maps[5].push(
            line
            .take(FACE_WIDTH)
            .map(|c| *c)
            .collect());
    }


    let path = String::from(input
                            .lines()
                            .into_iter()
                            .skip_while(|s| !s.is_empty())
                            .skip(1)
                            .collect::<String>());

    return (maps, path);
}

// take in current position, assumes we take one more step in the current facing direction,
// return the first position you'll land on the next face
// there's definitely an elegant, cool, very general way to do this but I'm just not gonna do that
// right now
//
// Arbitrary conventions based on the puzzle input, facing the cube head on
    // 1 is the top face of the cube
    // 2 is is the right face
    // 3 is the front face
    // 4 is the left face
    // 5 is the bottom face
    // 6 is the back face
    // and all that minus 1 because of 0-based indexing
fn get_next_face(pos: &Pos) -> Pos {
    let new_pos: Pos =
    match pos.cube_face {
        0 => {
            match pos.orientation {
                Facing::Right => {
                   Pos {
                        orientation: Facing::Right,
                        cube_face: 1,
                        row: pos.row,
                        row_idx: 0
                   }
                },
                Facing::Down => {
                    Pos {
                       orientation: Facing::Down,
                       cube_face: 2,
                       row: 0,
                       row_idx: pos.row_idx
                    }
                },
                Facing::Left => {
                    Pos {
                        orientation: Facing::Right,
                        cube_face: 3,
                        row: FACE_LEN - pos.row - 1,
                        row_idx: 0
                    }
                },
                Facing::Up => {
                   Pos {
                        orientation: Facing::Right,
                        cube_face: 5,
                        row: pos.row_idx,
                        row_idx: 0
                   }
                }
            }
        },
        1 => {
            match pos.orientation {
                Facing::Right => {
                    Pos {
                        orientation: Facing::Left,
                        cube_face: 4,
                        row: FACE_LEN - pos.row - 1,
                        row_idx: FACE_WIDTH - 1
                    }
                },
                Facing::Down => {
                    Pos {
                        orientation: Facing::Left,
                        cube_face: 2,
                        row: pos.row_idx,
                        row_idx: FACE_WIDTH - 1
                    }
                },
                Facing::Left => {
                    Pos {
                        orientation: Facing::Left,
                        cube_face: 0,
                        row: pos.row,
                        row_idx: FACE_WIDTH - 1
                    }
                },
                Facing::Up => {
                    Pos {
                        orientation: Facing::Up,
                        cube_face: 5,
                        row: FACE_LEN - 1,
                        row_idx: pos.row_idx
                    }
                }
            }
        },
        2 => {
            match pos.orientation {
                Facing::Right => {
                    Pos {
                        orientation: Facing::Up,
                        cube_face: 1,
                        row: FACE_LEN - 1,
                        row_idx: pos.row
                    }
                },
                Facing::Down => {
                    Pos {
                        orientation: Facing::Down,
                        cube_face: 4,
                        row: 0,
                        row_idx: pos.row_idx
                    }
                },
                Facing::Left => {
                    Pos {
                        orientation: Facing::Down,
                        cube_face: 3,
                        row: 0,
                        row_idx: pos.row
                    }
                },
                Facing::Up => {
                    Pos {
                        orientation: Facing::Up,
                        cube_face: 0,
                        row: FACE_LEN - 1,
                        row_idx: pos.row_idx
                    }
                }
            }
        },
        3 => {
            match pos.orientation {
                Facing::Right => {
                    Pos {
                        orientation: Facing::Right,
                        cube_face: 4,
                        row: pos.row,
                        row_idx: 0
                    }
                },
                Facing::Down => {
                    Pos {
                        orientation: Facing::Down,
                        cube_face: 5,
                        row: 0,
                        row_idx: pos.row_idx
                    }
                },
                Facing::Left => {
                    Pos {
                        orientation: Facing::Right,
                        cube_face: 0,
                        row: FACE_LEN - pos.row - 1,
                        row_idx: 0
                    }
                },
                Facing::Up => {
                    Pos {
                        orientation: Facing::Right,
                        cube_face: 2,
                        row: pos.row_idx,
                        row_idx: 0
                    }
                }
            }
        },
        4 => {
            match pos.orientation {
                Facing::Right => {
                    Pos {
                        orientation: Facing::Left,
                        cube_face: 1,
                        row: FACE_LEN - pos.row - 1,
                        row_idx: FACE_WIDTH - 1
                    }
                },
                Facing::Down => {
                    Pos {
                        orientation: Facing::Left,
                        cube_face: 5,
                        row: pos.row_idx,
                        row_idx: FACE_WIDTH - 1
                    }
                },
                Facing::Left => {
                    Pos {
                        orientation: Facing::Left,
                        cube_face: 3,
                        row: pos.row,
                        row_idx: FACE_WIDTH - 1
                    }
                },
                Facing::Up => {
                    Pos {
                        orientation: Facing::Up,
                        cube_face: 2,
                        row: FACE_LEN - 1,
                        row_idx: pos.row_idx
                    }
                }
            }
        },
        5 => {
            match pos.orientation {
                Facing::Right => {
                    Pos {
                        orientation: Facing::Up,
                        cube_face: 4,
                        row: FACE_LEN - 1,
                        row_idx: pos.row
                    }
                },
                Facing::Down => {
                    Pos {
                        orientation: Facing::Down,
                        cube_face: 1,
                        row: 0,
                        row_idx: pos.row_idx
                    }
                },
                Facing::Left => {
                    Pos {
                        orientation: Facing::Down,
                        cube_face: 0,
                        row: 0,
                        row_idx: pos.row
                    }
                },
                Facing::Up => {
                    Pos {
                        orientation: Facing::Up,
                        cube_face: 3,
                        row: FACE_LEN - 1,
                        row_idx: pos.row_idx
                    }
                }
            }
        },
        _ => {
            panic!("Invalid cube face value!");
        }
    };

    return new_pos;
}

fn make_move(maps: &Vec<Vec<Vec<char>>>, mut pos: Pos, next_move: Move) -> Pos {
    match next_move {
        Move::Move(num) => {
            match pos.orientation {
                Facing::Right => {
                    let curr_row = pos.row;
                    let curr_idx = pos.row_idx;
                    for incr in 1..=num {
                        // first check if we're running off the right edge
                        if curr_idx + incr >= FACE_WIDTH {
                            let check_pos = get_next_face(&pos);
                            // if we can wrap around do it
                            if maps[check_pos.cube_face][check_pos.row][check_pos.row_idx] == '.' {    
                                let tmp_move = Move::Move(num - incr);
                                return make_move(maps, check_pos, tmp_move);
                            } else { // otherwise we're done
                                pos.row_idx = curr_idx + incr - 1;
                                return pos;
                            }
                        } else if maps[pos.cube_face][curr_row][curr_idx + incr] == '#' { // next check if we're blocked
                            pos.row_idx = curr_idx + incr - 1;
                            return pos;
                        } else if maps[pos.cube_face][curr_row][curr_idx + incr] == '.' { // or if we can just move one
                            continue;
                        } else {
                            panic!("Unexpected map conditions!");
                        }
                    }
                    // if we made it all the way through the loop, we need to return the final
                    // position
                    pos.row_idx = curr_idx + num;
                },
                Facing::Down => {
                    let curr_row = pos.row;
                    let curr_idx = pos.row_idx;
                    for incr in 1..=num {
                        // first check if we're running off the bottom edge
                        if curr_row + incr >= FACE_LEN {
                            let check_pos = get_next_face(&pos);
                            // if we can wrap around do it
                            if maps[check_pos.cube_face][check_pos.row][check_pos.row_idx] == '.' {
                                let tmp_move = Move::Move(num - incr);
                                return make_move(maps, check_pos, tmp_move);
                            } else { // otherwise we're done
                                pos.row = curr_row + incr - 1;
                                return pos;
                            }
                        } else if maps[pos.cube_face][curr_row + incr][curr_idx] == '#' { // next check if we're blocked
                            pos.row = curr_row + incr - 1;
                            return pos;
                        } else if maps[pos.cube_face][curr_row + incr][curr_idx] == '.' { // or if we can just move one
                            continue;
                        } else {
                            panic!("Unexpected map conditions!");
                        }
                    }
                    // if we made it all the way through the loop, we need to return the final
                    // position
                    pos.row = curr_row + num;
                },
                Facing::Left => {
                    let curr_row = pos.row;
                    let curr_idx = pos.row_idx;
                    for incr in 1..=num {
                        // first check if we're running off the left edge
                        if (curr_idx as i32 - incr as i32) < 0 {
                            let check_pos = get_next_face(&pos);
                            // if we can wrap around do it
                            if maps[check_pos.cube_face][check_pos.row][check_pos.row_idx] == '.' {
                                let tmp_move = Move::Move(num - incr);
                                return make_move(maps, check_pos, tmp_move);
                            } else { // otherwise we're done
                                pos.row_idx = (curr_idx as i32 - incr as i32 + 1) as usize;
                                return pos;
                            }
                        } else if maps[pos.cube_face][curr_row][curr_idx - incr] == '#' { // next check if we're blocked
                            pos.row_idx = curr_idx - incr + 1;
                            return pos;
                        } else if maps[pos.cube_face][curr_row][curr_idx - incr] == '.' { // or if we can just move one
                            continue;
                        } else {
                            panic!("Unexpected map conditions!");
                        }
                    }
                    // if we made it all the way through the loop, we need to return the final
                    // position
                    pos.row_idx = curr_idx - num; 
                },
                Facing::Up => {
                    let curr_row = pos.row;
                    let curr_idx = pos.row_idx;
                    for incr in 1..=num {
                        // first check if we're running off the top edge
                        if (curr_row as i32 - incr as i32) < 0 {
                            let check_pos = get_next_face(&pos);
                            // if we can wrap around do it
                            if maps[check_pos.cube_face][check_pos.row][check_pos.row_idx] == '.' {
                                let tmp_move = Move::Move(num - incr);
                                return make_move(maps, check_pos, tmp_move);
                            } else { // otherwise we're done
                                pos.row = (curr_row as i32 - incr as i32 + 1) as usize;
                                return pos;
                            }
                        } else if maps[pos.cube_face][curr_row - incr][curr_idx] == '#' { // next check if we're blocked
                            pos.row = curr_row - incr + 1;
                            return pos;
                        } else if maps[pos.cube_face][curr_row - incr][curr_idx] == '.' { // or if we can just move one
                            continue;
                        } else {
                            panic!("Unexpected map conditions!");
                        }
                    }
                    // if we made it all the way through the loop, we need to return the final
                    // position
                    pos.row = curr_row - num;
                }
            }
            return pos;
        },
        Move::Turn(dir) => {
            pos.orientation.turn(dir);
            return pos;
        }
    }
}

fn get_move<T>(instr: &mut Peekable<T>) -> Option<Move> 
where 
T: Iterator<Item = char>
{
        let next_move;

        let c = match instr.next() {
            Some(x) => x,
            None => { return None; }
        };
        if c == 'L' || c == 'R' { // if it's a spin direction then do that
            next_move = Some(Move::turn_move(c))
        } else { // else it's a number instruction...
            let mut num_str = String::from(c); // grab the first digit
            loop { // and all the digits that follow
                match instr.peek() {
                    Some('L')| Some('R') => {
                        break;
                    },
                    Some(_) => {
                        let num = instr.next().unwrap();
                        num_str.push(num);
                    },
                    None => {
                        break;
                    }
                }

            }
            let num = num_str.parse::<usize>().unwrap();
            next_move = Some(Move::Move(num));
        }

    return next_move;
}

fn main() {

    let (maps, path) = get_notes("input.txt");
 
    let mut pos = Pos {
       orientation: Facing::Right,
       cube_face: 0,
       row: 0,
       row_idx: 0
    };

    // parse path...
    let mut iter = path.trim().chars().peekable();
    loop {
        let next_move = match get_move(&mut iter) {
            Some(instr) => instr,
            None => { break; }
        };

        //println!("{:#?}", next_move);
        pos = make_move(&maps, pos, next_move);
    }

    println!("Final position: {:#?}", pos);
    let mut final_row = pos.row + 1;
    final_row += match pos.cube_face {
        1|2 => 0,
        3 => FACE_LEN,
        4|5 => 2 * FACE_LEN,
        6 => 3 * FACE_LEN,
        _ => {
            panic!("Invalid final cube face number!");
        }
    };
    let mut final_col = pos.row_idx + 1;
    final_col += match pos.cube_face {
        1|3|5 => FACE_WIDTH,
        2 => 2 * FACE_WIDTH,
        4|6 => 0,
        _ => {
            panic!("Invalid cube face number!");
        }
    };
    println!("Adjusted: Row: {}, Col: {}", final_row, final_col);
    
    let mut password = (1000 * final_row) + (4 * final_col);
    password += match pos.orientation {
        Facing::Right => 0,
        Facing::Down => 1,
        Facing::Left => 2,
        Facing::Up => 3
    };
    println!("Password: {password}"); 
}

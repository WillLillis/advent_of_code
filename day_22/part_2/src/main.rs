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

// - TODO: need to redo map reading to get the 6 cube faces, store
// as Vector of maps (Vec<Vec<Vec<char>>>)
// - Need to get some other data structure to track which cube face
// you're on, and dictate the transitions (which face and which direction)
// - code in make_move shouldn't need too much refactoring, just need to 
// explicitly change faces when we run over one of the edges
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

// find the first valid index in a given row
// assumes there is a valid index, which is fine for this problem
// but for more general use we'd have to be more careful
fn row_first_idx(map: &Vec<Vec<char>>, row: usize, from_left: bool) -> usize { 
    if from_left {
        for idx in 0..map[row].len() {
            match map[row][idx] {
                '.'|'#' => {
                    return idx;
                },
                _ => {
                    continue;
                }
            }
        }
    } else { // from the right
        for idx in (0..map[row].len()).rev() {
            match map[row][idx] {
                '.'|'#' => {
                    return idx;
                },
                _ => {
                    continue;
                }
            }
        }
    }
    
    panic!("No valid index found!");
}

fn col_first_idx(map: &Vec<Vec<char>>, col: usize, from_top: bool) -> usize {
    if from_top {
        for idx in 0..map.len() {
            match map[idx][col] {
                '.'|'#' => {
                    return idx;
                },
                _ => {
                    continue;
                }
            }
        }
    } else { // from the right
        for idx in (0..map.len()).rev() {
            match map[idx][col] {
                '.'|'#' => {
                    return idx;
                },
                _ => {
                    continue;
                }
            }
        }
    }
    
    panic!("No valid index found!");
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
fn get_next_face(pos: Pos) -> Pos {
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

                },
                Facing::Down => {


                },
                Facing::Left => {

                },
                Facing::Up => {

                }
            }
        },
        3 => {
            match pos.orientation {
                Facing::Right => {

                },
                Facing::Down => {


                },
                Facing::Left => {

                },
                Facing::Up => {

                }
            }
        },
        4 => {
            match pos.orientation {
                Facing::Right => {

                },
                Facing::Down => {


                },
                Facing::Left => {

                },
                Facing::Up => {

                }
            }
        },
        5 => {
            match pos.orientation {
                Facing::Right => {

                },
                Facing::Down => {


                },
                Facing::Left => {

                },
                Facing::Up => {

                }
            }
        },
        _ => {
            panic!("Invalid cube face value!");
        }
    };

    return new_pos;
}

fn make_move(map: &Vec<Vec<char>>, mut pos: Pos, next_move: Move) -> Pos {
    match next_move {
        Move::Move(num) => {
            match pos.orientation {
                Facing::Right => {
                    let curr_row = pos.row;
                    let curr_idx = pos.row_idx;
                    for incr in 1..=num {
                        // first check if we're running off the right edge
                        if curr_idx + incr >= map[curr_row].len() || (map[curr_row][curr_idx + incr] == ' ') {
                            let first_idx = row_first_idx(map, curr_row, true);
                            // if we can wrap around do it
                            if map[curr_row][first_idx] == '.' {
                                pos.row_idx = first_idx;
                                let tmp_move = Move::Move(num - incr);
                                return make_move(map, pos, tmp_move);
                            } else { // otherwise we're done
                                pos.row_idx = curr_idx + incr - 1;
                                return pos;
                            }
                        } else if map[curr_row][curr_idx + incr] == '#' { // next check if we're blocked
                            pos.row_idx = curr_idx + incr - 1;
                            return pos;
                        } else if map[curr_row][curr_idx + incr] == '.' { // or if we can just move one
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
                        if curr_row + incr >= map.len() || map[curr_row + incr][curr_idx] == ' ' {
                            let first_row = col_first_idx(map, curr_idx, true);
                            // if we can wrap around do it
                            if map[first_row][curr_idx] == '.' {
                                pos.row = first_row;
                                let tmp_move = Move::Move(num - incr);
                                return make_move(map, pos, tmp_move);
                            } else { // otherwise we're done
                                pos.row = curr_row + incr - 1;
                                return pos;
                            }
                        } else if map[curr_row + incr][curr_idx] == '#' { // next check if we're blocked
                            pos.row = curr_row + incr - 1;
                            return pos;
                        } else if map[curr_row + incr][curr_idx] == '.' { // or if we can just move one
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
                        if ((curr_idx as i32 - incr as i32) < 0) || map[curr_row][curr_idx - incr] == ' ' {
                            let first_idx = row_first_idx(map, curr_row, false);
                            // if we can wrap around do it
                            if map[curr_row][first_idx] == '.' {
                                pos.row_idx = first_idx;
                                let tmp_move = Move::Move(num - incr);
                                return make_move(map, pos, tmp_move);
                            } else { // otherwise we're done
                                pos.row_idx = (curr_idx as i32 - incr as i32 + 1) as usize;
                                return pos;
                            }
                        } else if map[curr_row][curr_idx - incr] == '#' { // next check if we're blocked
                            pos.row_idx = curr_idx - incr + 1;
                            return pos;
                        } else if map[curr_row][curr_idx - incr] == '.' { // or if we can just move one
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
                        if ((curr_row as i32 - incr as i32) < 0) || map[curr_row - incr][curr_idx] == ' ' {
                            let first_row = col_first_idx(map, curr_idx, false);
                            // if we can wrap around do it
                            if map[first_row][curr_idx] == '.' {
                                pos.row = first_row;
                                let tmp_move = Move::Move(num - incr);
                                return make_move(map, pos, tmp_move);
                            } else { // otherwise we're done
                                pos.row = (curr_row as i32 - incr as i32 + 1) as usize;
                                return pos;
                            }
                        } else if map[curr_row - incr][curr_idx] == '#' { // next check if we're blocked
                            pos.row = curr_row - incr + 1;
                            return pos;
                        } else if map[curr_row - incr][curr_idx] == '.' { // or if we can just move one
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
 
    // TODO: work out how face to face translations go, make sure you're starting on the right
    // face, update Pos data struct accordingly...
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

        //pos = make_move(&maps, pos, next_move);
    }

    println!("Final position: {:#?}", pos);
    let final_row = pos.row + 1;
    let final_col = pos.row_idx + 1;
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

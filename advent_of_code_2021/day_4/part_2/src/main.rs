use std::fs;

#[derive(Debug, Clone, Copy)]
struct BingoSquare {
    num: u32,
    marked: bool,
}

impl BingoSquare {
    fn new() -> Self {
        BingoSquare {
            num: 0,
            marked: false,
        }
    }
    fn set_val(&mut self, val: u32) {
        self.num = val;
    }
}

#[derive(Debug, Clone, Copy)]
struct BingoBoard {
    board: [[BingoSquare; 5]; 5],
}

impl BingoBoard {
    fn new() -> Self {
        BingoBoard {
            board: [[BingoSquare::new(); 5]; 5],
        }
    }
    fn mark_num(&mut self, val: u32) {
        for row in self.board.iter_mut() {
            for square in row.iter_mut() {
                if square.num == val {
                    square.marked = true;
                }
            }
        }
    }
    fn is_winner(&self) -> bool {
        for row in self.board.iter() {
            if row.iter().fold(true, |accum, sq| accum && sq.marked) {
                return true;
            }
        }

        for col in 0..5 {
            let mut col_check = true;
            for row in self.board.iter() {
                col_check &= row[col].marked;
            }
            if col_check {
                return true;
            }
        }

        return false;
    }
    fn sum_unmarked(&self) -> u32 {
        let mut sum = 0;
        for row in self.board.iter() {
            sum += row
                .iter()
                .fold(0, |accum, sq| accum + if !sq.marked { sq.num } else { 0 });
        }

        return sum;
    }
}

fn get_input(file_name: &str) -> (Vec<u32>, Vec<BingoBoard>) {
    let input = fs::read_to_string(file_name).expect("Failed to read the input file");

    let nums: Vec<u32> = input
        .lines()
        .take_while(|line| line.len() > 0)
        .collect::<String>()
        .split(',')
        .filter_map(|x| x.parse::<u32>().ok())
        .collect();

    let mut boards: Vec<BingoBoard> = Vec::new();

    let mut board_iter = input
        .lines()
        .skip_while(|line| line.len() > 0) // skip bingo numbers
        .skip(1) // skip the blank line between the numbers and the boards
        .peekable();

    while let Some(_) = board_iter.peek() {
        let mut new_board = BingoBoard::new();

        for i in 0..5 {
            let row = board_iter.next().unwrap();
            let row_nums: Vec<u32> = row
                .split_whitespace()
                .filter_map(|x| x.parse::<u32>().ok())
                .collect();

            for (j, &x) in row_nums.iter().enumerate() {
                new_board.board[i][j].set_val(x);
            }
        }
        boards.push(new_board);
        board_iter.next(); // skip blank line
    }

    return (nums, boards);
}

fn main() {
    let (nums, mut boards) = get_input("input.txt");

    let mut score: u32 = 0;
    let mut last_idx: Option<usize> = None;

    'num_loop: for &num in nums.iter() {
        let mut winner_count = 0;
        for board in boards.iter_mut() {
            board.mark_num(num);
            if board.is_winner() {
                winner_count += 1;
            }
        }
        if last_idx == None && winner_count == boards.len() - 1 {
            for (i, board) in boards.iter().enumerate() {
                if !board.is_winner() {
                    last_idx = Some(i);
                    break;
                }
            }
        }
        match last_idx {
            Some(idx) => {
                if boards[idx].is_winner() {
                    score = boards[idx].sum_unmarked() * num;
                    break 'num_loop;
                }
            }
            None => {}
        }
    }

    println!("Score: {score}");
}

use std::{cmp, fs};

#[derive(Debug, Clone, Copy)]
enum FoldAxis {
    X,
    Y,
}

#[derive(Debug, Clone, Copy)]
struct FoldInstr {
    axis: FoldAxis,
    val: usize,
}

fn get_manual(file_name: &str) -> (Vec<Vec<bool>>, Vec<FoldInstr>) {
    let input = fs::read_to_string(file_name).expect("Failed to read the input file.");
    let nums: Vec<(usize, usize)> = input
        .lines()
        .take_while(|s| s.len() > 0)
        .map(|s| s.split(',').collect::<Vec<&str>>())
        .filter_map(|x| {
            Some((
                x[0].parse::<usize>().unwrap(),
                x[1].parse::<usize>().unwrap(),
            ))
        })
        .collect();

    let x_max = nums
        .iter()
        .fold(usize::MIN, |accum, &(x, _)| cmp::max(accum, x))
        + 1;
    let y_max = nums
        .iter()
        .fold(usize::MIN, |accum, &(_, y)| cmp::max(accum, y))
        + 1;

    let mut dots: Vec<Vec<bool>> = vec![vec![false; x_max]; y_max];

    for &(x, y) in nums.iter() {
        dots[y][x] = true;
    }

    let folds: Vec<FoldInstr> = input
        .lines()
        .skip_while(|s| s.len() > 0)
        .skip(1)
        .map(|line| line.split(&[' ', '='][..]).collect::<Vec<&str>>())
        .map(|x| {
            let axis = match x[2] {
                "x" => FoldAxis::X,
                "y" => FoldAxis::Y,
                _ => {
                    panic!("Parsing error!");
                }
            };
            let val = x[3].parse::<usize>().unwrap();
            FoldInstr { axis, val }
        })
        .collect();

    return (dots, folds);
}

fn do_fold(dots: &mut Vec<Vec<bool>>, fold: FoldInstr) {
    match fold.axis {
        // fold bottom up to top
        FoldAxis::Y => {
            for (i, row) in (fold.val + 1..dots.len()).enumerate() {
                for j in 0..dots[0].len() {
                    dots[fold.val - 1 - i][j] |= dots[row][j];
                }
            }
            dots.resize(fold.val, Vec::new());
        }
        // fold right side to left
        FoldAxis::X => {
            for row in 0..dots.len() {
                for (i, col) in (fold.val + 1..dots[row].len()).enumerate() {
                    dots[row][fold.val - 1 - i] |= dots[row][col];
                }
                // shrink to specified size func?
                dots[row].resize(fold.val, false);
            }
        }
    }
}

fn main() {
    let (mut dots, folds) = get_manual("input.txt");

    for &fold in folds.iter() {
        do_fold(&mut dots, fold);
    }
    
    for i in 0..dots.len() {
        for j in 0..dots[i].len() {
            if dots[i][j] {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }

}

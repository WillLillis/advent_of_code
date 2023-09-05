use std::{cmp, fs};

#[derive(Debug)]
enum FoldAxis {
    X,
    Y,
}

#[derive(Debug)]
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
        .fold(usize::MIN, |accum, &(x, _)| cmp::max(accum, x)) + 1;
    let y_max = nums
        .iter()
        .fold(usize::MIN, |accum, &(_, y)| cmp::max(accum, y)) + 1;

    let mut dots: Vec<Vec<bool>> = vec![vec![false; x_max]; y_max];

    for &(x, y) in nums.iter() {
        dots[y][x] = true;
    }

    let mut folds: Vec<FoldInstr> = input
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

fn main() {
    let (mut dots, folds) = get_manual("test_input.txt");

    for row in dots.iter() {
        println!("{:?}", row);
    }

    println!("Folds:");
    for fold in folds.iter() {
        println!("{:?}", fold);
    }

    // TODO: implement folding functionality, count up the dots
}

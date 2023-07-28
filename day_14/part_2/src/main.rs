use std::fs;
use std::cmp;

fn fill_in_blocks(env: &mut Vec<Vec<char>>, pos_1: (usize, usize), pos_2: (usize, usize)) {
    if pos_1.0 == pos_2.0 { // vertical line
        for idx in cmp::min(pos_1.1, pos_2.1)..=cmp::max(pos_1.1, pos_2.1) {
            env[idx][pos_1.0] = '#';
        }
    } else if pos_1.1 == pos_2.1 { // horizontal line    
        for idx in cmp::min(pos_1.0, pos_2.0)..=cmp::max(pos_1.0, pos_2.0) {
            env[pos_1.1][idx] = '#';
        }
    } else {
        panic!("Invalid coordinate pair!");
    }

}

fn file_to_env() -> Vec<Vec<char>> {
    let input = fs::read_to_string("input.txt").unwrap();

    // need to read through the entire file and find the highest y point value
    // and then allocate our vector to have 2 rows past that, AND THEN fill the last row
    // with rocks ('#')
    let mut max_y: usize = 0;
    
    for line in input.lines() {
        let points = line.trim().split("->");
        let points: Vec<(usize, usize)> = points.map(|x| {
            let (col, elev) = x.split_once(',').unwrap();
            let col = col.trim().parse::<usize>().unwrap();
            let elev = elev.trim().parse::<usize>().unwrap();
            (col, elev)
        }
        )
            .collect();

        for coord in points {
            let (_, y) = coord;
            max_y = cmp::max(max_y, y);
        }

    }

    let mut env: Vec<Vec<char>> = Vec::new();
    for _ in 0..=max_y+1 {
        env.push(vec!['.'; 1024]);
    }
    env.push(vec!['#'; 1024]); // last row is the cave floor-> all rock

    for line in input.lines() {
        let points = line.trim().split("->");
        let points: Vec<(usize, usize)> = points.map(|x| {
            let (col, elev) = x.split_once(',').unwrap();
            let col = col.trim().parse::<usize>().unwrap();
            let elev = elev.trim().parse::<usize>().unwrap();
            (col, elev)
        }
        )
            .collect();

        let mut iter_1 = points.iter();
        let mut iter_2 = points.iter().skip(1);

        loop {
            let first = iter_1.next().unwrap();
            let second = match iter_2.next() {
                Some(x) => x,
                None => {break;}
            };

            fill_in_blocks(&mut env, first.clone(), second.clone());
        }
    }
 
    env
}

fn simulate(env: &mut Vec<Vec<char>>) -> bool {
    let mut sand_pos = (0usize, 500usize);    
    
    loop {
        // first try going straight down
        if env[sand_pos.0 + 1][sand_pos.1] == '.' {
            sand_pos.0 += 1;
            continue;
        }
        // then diagonally down and left
        if env[sand_pos.0 + 1][sand_pos.1 - 1] == '.' {
            sand_pos.0 += 1;
            sand_pos.1 -= 1;
            continue;
        }
        // then diagonally down and right
        if env[sand_pos.0 + 1][sand_pos.1 + 1] == '.' {
            sand_pos.0 += 1;
            sand_pos.1 += 1;
            continue;
        }
        // if we've reached this point we're stuck
        env[sand_pos.0][sand_pos.1] = 'o';
        return !(sand_pos.0 == 0 && sand_pos.1 == 500);
    }
}

fn main() {
    let mut env = file_to_env();
    let mut sand_count = 0;

    while simulate(&mut env) {
        sand_count += 1;
    }
    sand_count += 1; // account for the last grain that broke the while loop

    println!("Sand count: {}", sand_count);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SpaceItem {
    Empty,
    Galaxy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn new(row: usize, col: usize) -> Self {
        Point { row, col }
    }
}

fn get_map(input: &str) -> Vec<Vec<SpaceItem>> {
    let mut map: Vec<Vec<SpaceItem>> = Vec::new();

    for line in input.lines() {
        map.push(
            line.trim()
                .chars()
                .map(|c| match c {
                    '.' => SpaceItem::Empty,
                    '#' => SpaceItem::Galaxy,
                    _ => unreachable!(),
                })
                .collect(),
        );
    }

    map
}

fn get_expansions(map: &Vec<Vec<SpaceItem>>) -> (Vec<usize>, Vec<usize>) {
    let mut expand_rows = Vec::new();
    for (i, row) in map.iter().enumerate() {
        if !row.iter().any(|x| *x != SpaceItem::Empty) {
            expand_rows.push(i);
        }
    }
    let mut expand_cols = Vec::new();
    for col in 0..map[0].len() {
        let mut empty = true;
        for row in 0..map.len() {
            if map[row][col] != SpaceItem::Empty {
                empty = false;
                break;
            }
        }
        if empty {
            expand_cols.push(col);
        }
    }

    (expand_rows, expand_cols)
}

fn get_galaxies(map: &Vec<Vec<SpaceItem>>) -> Vec<Point> {
    let mut galaxies = Vec::new();

    for (i, row) in map.iter().enumerate() {
        for (j, _col) in row.iter().enumerate() {
            if map[i][j] == SpaceItem::Galaxy {
                galaxies.push(Point::new(i, j));
            }
        }
    }

    galaxies
}

fn dist(loc_1: Point, loc_2: Point) -> usize {
    loc_1.row.abs_diff(loc_2.row) + loc_1.col.abs_diff(loc_2.col)
}

fn sum_distances(
    map: &Vec<Vec<SpaceItem>>,
    expand_rows: &Vec<usize>,
    expand_cols: &Vec<usize>,
) -> usize {
    let galaxies = get_galaxies(map);
    let mut sum = 0usize;
    const EXPANSION_FACTOR: usize = 1000000;

    for i in 0..galaxies.len() - 1 {
        for j in i + 1..galaxies.len() {
            let min_row = galaxies[i].row.min(galaxies[j].row);
            let max_row = galaxies[i].row.max(galaxies[j].row);
            let min_col = galaxies[i].col.min(galaxies[j].col);
            let max_col = galaxies[i].col.max(galaxies[j].col);
            let total: usize = dist(galaxies[i], galaxies[j])
                + expand_rows
                    .iter()
                    .map(|row_idx| {
                        if *row_idx >= min_row && *row_idx <= max_row {
                            EXPANSION_FACTOR - 1
                        } else {
                            0
                        }
                    })
                    .sum::<usize>()
                + expand_cols
                    .iter()
                    .map(|col_idx| {
                        if *col_idx >= min_col && *col_idx <= max_col {
                            EXPANSION_FACTOR - 1
                        } else {
                            0
                        }
                    })
                    .sum::<usize>();
            sum += total;
        }
    }

    sum
}

fn main() {
    let input = std::fs::read_to_string("../input").expect("Failed to read the input file");

    let map = get_map(&input);
    let (expand_rows, expand_cols) = get_expansions(&map);

    let dist = sum_distances(&map, &expand_rows, &expand_cols);

    println!("Dist: {}", dist);
}

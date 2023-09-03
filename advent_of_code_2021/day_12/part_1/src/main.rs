use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
enum CaveSize {
    Small,
    Large
}

impl CaveSize {
    fn size_from_name(name: &str) -> Option<Self> {
        let first = name.chars().nth(0).unwrap();
        if first.is_lowercase() {
            return Some(CaveSize::Small);
        } else if first.is_uppercase() {
            return Some(CaveSize::Large);
        } else {
            return None;
        }
    }
}

#[derive(Debug)]
struct CaveInfo {
    name: String,
    size: CaveSize
}


// returns adjacency matrix and a list of info of the caves
// indexing should be consistent between the two
fn get_caves(file_name: &str) -> (Vec<Vec<bool>>, Vec<CaveInfo>) {
    let input = fs::read_to_string(file_name).expect("Failed to read the input file");
    
    let mut cave_info: Vec<CaveInfo> = Vec::new();
    let mut name_to_idx: HashMap<String, usize> = HashMap::new();
    let mut cave_idx = 0;

    for line in input.lines() {
        let names = line.split('-').collect::<Vec<&str>>();
        for name in names {
            if !name_to_idx.contains_key(name) {
                name_to_idx.insert(String::from(name), cave_idx);
                cave_idx += 1;
                let size = CaveSize::size_from_name(name).unwrap();
                cave_info.push(CaveInfo { name: String::from(name), size });
            }
        }
    }

    let n_caves = cave_info.len();

    let mut adj_matrix: Vec<Vec<bool>> = vec![vec![false; n_caves]; n_caves];

    for line in input.lines() {
        let names: Vec<&str> = line.split('-').collect();
        assert!(names.len() == 2);
        let i = *name_to_idx.get(names[0]).unwrap();
        let j = *name_to_idx.get(names[1]).unwrap();
        adj_matrix[i][j] = true;
        adj_matrix[j][i] = true;
    }

    return (adj_matrix, cave_info);
}

fn valid_visit(curr_cave_idx: usize, next_cave_idx: usize, visited: &Vec<bool>, adj_matrix: &Vec<Vec<bool>>, cave_info: &Vec<CaveInfo) -> bool {
    if !adj_matrix[curr_cave_idx][next_cave_idx] {
        return false;
    }
    if cave_info[next_cave_idx].size == CaveSize::Large {
        return true;
    } else if cave_info[next_cave_idx].size == CaveSize::Small && !visited[next_cave_idx] {
        return true;
    } else {
        return false;
    }
}

fn visit_valid_paths(curr_cave_idx: usize, visited: &mut Vec<bool>, adj_matrix: &Vec<Vec<bool>>, cave_info: &Vec<CaveInfo>) -> u32 {
    
    let mut n_paths = 0;

    for j in 0..adj_matrix.len() {
        if valid_visit(curr_cave_idx, j, visited, adj_matrix, cave_info) {
            // do the visit stuff, count up the total paths
        }
    }

    0
}

fn main() {
    let (mut adj_matrix, mut cave_info) = get_caves("test_input_1.txt");
}

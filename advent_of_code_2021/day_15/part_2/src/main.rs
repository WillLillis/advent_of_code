use priority_queue::PriorityQueue;
use std::fs;

fn get_map(file_name: &str) -> Vec<Vec<u8>> {
    let input = fs::read_to_string(file_name).expect("Failed to read the input file.");

    let mut map: Vec<Vec<u8>> = Vec::new();

    for line in input.lines() {
        let mut row: Vec<u8> = line
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|x| x as u8)
            .collect();
        let mut accum: Vec<u8> = Vec::new();
        for i in 1..5 {
            let mut tmp = row.clone();
            tmp.iter_mut().for_each(|x| {
                if *x + i <= 9 {
                    *x += i
                } else {
                    *x = (*x + i) % 9
                }
            });
            accum.append(&mut tmp);
        }
        row.append(&mut accum);
        map.push(row);
    }

    let mut accum: Vec<Vec<u8>> = Vec::new();
    for i in 1..5 {
        let mut tmp = map.clone();
        tmp.iter_mut().flatten().for_each(|x| {
            if *x + i <= 9 {
                *x += i
            } else {
                *x = (*x + i) % 9
            }
        });
        accum.append(&mut tmp);
    }
    
    map.append(&mut accum);

    return map;
}

// easy application of Djikstra...
fn safest_path_risk(map: &Vec<Vec<u8>>) -> u32 {
    let map_width = map[0].len();
    let map_len = map.len();

    let mut pq = PriorityQueue::new();
    let mut dist: Vec<Vec<u32>> = vec![vec![u32::MAX; map_width]; map_len];
    let mut prev: Vec<Vec<Option<(usize, usize)>>> = vec![vec![None; map_width]; map_len];

    dist[0][0] = 0;
    for i in 0..map_len {
        for j in 0..map_width {
            pq.push((i, j), u32::MAX);
        }
    }

    while let Some(((i,j), _)) = pq.pop() {
        let mut neighbors: [Option<(usize, usize)>; 4] = [None; 4];
        let mut idx = 0usize;
        // check up
        if i > 0 {
            neighbors[idx] = Some((i - 1, j));
            idx += 1;
        }
        // check down
        if i < map_len - 1 {
            neighbors[idx] = Some((i + 1, j));
            idx += 1;
        }
        // check left
        if j > 0 {
            neighbors[idx] = Some((i, j - 1));
            idx += 1;
        }
        // check right
        if j < map_width - 1 {
            neighbors[idx] = Some((i, j + 1));
        }
        for &neigh in neighbors.iter() {
            // could just go off of idx, but it shouldn't make
            // much of a difference
            match neigh {
                Some((a, b)) => {
                    let alt_dist = dist[i][j]
                        + if dist[i][j] == u32::MAX {
                            0
                        } else {
                            map[a][b] as u32
                        };
                    if alt_dist < dist[a][b] {
                        dist[a][b] = alt_dist;
                        prev[a][b] = Some((i, j));
                        pq.push_decrease((a, b), alt_dist);
                    }
                }
                None => {
                    break;
                }
            }
        }
    }

    return dist[map_len - 1][map_width - 1];
}

fn main() {
    let map = get_map("input.txt");

    let lowest_risk = safest_path_risk(&map);

    println!("Lowest risk: {}", lowest_risk);
}

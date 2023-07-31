// Got stuck on this problem as well, took a lot of inspiration from the reddit thread, both to use 
// floyd-warshall as well as creating the partitions of paths for part 2

use std::{cmp, fs};
use std::collections::HashMap;
use bitmaps;
use std::time::Instant;

#[derive(Debug)]
struct ValveInfo {
    idx: usize,
    flow_rate: u32,
    neighbors: Vec<String>, 
}

impl ValveInfo {
    fn new(idx: usize, flow_rate: u32, neighbors: &Vec<String>) -> Self {
        ValveInfo {
            idx,
            flow_rate,
            neighbors: neighbors.clone(),
        }
    }
}

fn parse_data(file_name: &str) -> (HashMap<usize, ValveInfo>, HashMap<String, usize>) {
    let input = fs::read_to_string(file_name).unwrap();

    let mut valves: HashMap<usize, ValveInfo> = HashMap::new();
    let mut v_to_i: HashMap<String, usize> = HashMap::new();
    
    for (idx, line) in input.lines().enumerate() {
        let name = line.trim().split(' ').skip(1).next().unwrap().to_string();
        let flow_rate: u32 = line.trim().split(&[' ', '=', ';', ','][..])
            .find_map(|x| x.parse::<u32>().ok()).unwrap();
        let neighbors: Vec<String> = line.trim().split(&[' ', ','])
            .skip(9) // skip "Valve", "<name>", "has", "flow" "rate=<flow_rate>;", "tunnels", "lead", "to", "valves"
            .map(|s| s.to_string())
            .filter(|s| s.len() > 0)
            .collect();
        valves.insert(idx, ValveInfo::new(idx, flow_rate, &neighbors));
        v_to_i.insert(name.to_owned(), idx);
    }

    (valves, v_to_i)
}

fn floyd_warshall(valves: &HashMap<usize, ValveInfo>, v_to_i: &HashMap<String, usize>) -> Vec<Vec<i32>> {
    let num_valves = valves.len();
    let mut dist: Vec<Vec<i32>> = vec![vec![i32::MAX / 2; num_valves]; num_valves];
    
    for i in 0..num_valves {
        dist[i][i] = 0;    
    }

    for valve in valves {
        let curr_valve_idx = valve.1.idx;
        let neighbors = valves.get(&curr_valve_idx).unwrap().neighbors.clone(); // copy of the
                                                                                // neighbors by
                                                                                // their string
                                                                                // names
        for neighbor in neighbors {
            dist[curr_valve_idx][v_to_i.get(&neighbor).unwrap().clone()] = 1;
        }
        
    }

    for k in 0..num_valves {
        for i in 0..num_valves {
            for j in 0..num_valves {
                if dist[i][j] > dist[i][k] + dist[k][j] {
                    dist[i][j] = dist[i][k] + dist[k][j]
                }
            }
        }
    }

    dist
}

fn get_max_flow(valves: &HashMap<usize, ValveInfo>, dist: &Vec<Vec<i32>>,
                curr_valve_idx: usize, pending_valves: &mut bitmaps::Bitmap<64>, 
                curr_time: i32, cache: &mut HashMap<(usize, bitmaps::Bitmap<64>, i32), u32>) -> u32 {
    match cache.get(&(curr_valve_idx, pending_valves.clone(), curr_time)) {
        Some(flow) => return *flow,
        None => {}
    }

    let mut max_flow = u32::MIN;

    for i in 0..valves.len() {
        if pending_valves.get(i) { // if the valve has already been marked as used...
            continue;
        }
        if valves.get(&i).unwrap().flow_rate == 0 {
            continue;
        }

        let travel_time = dist[curr_valve_idx][i];
        let time_left: i32 = curr_time - travel_time - 1; // -1 to account for turning the valve on 
                                                     
        if time_left > 0 {
            pending_valves.set(i, true);
            let next_flow = get_max_flow(valves, dist, i, pending_valves, time_left, cache);
            let total_flow = next_flow + (valves.get(&i).unwrap().flow_rate * time_left as u32);

            max_flow = cmp::max(max_flow, total_flow);

            pending_valves.set(i, false);
        } else {
            continue;
        }
    }

    cache.insert((curr_valve_idx, pending_valves.clone(), curr_time), max_flow);
    max_flow
}

fn main() {
    let (valves, v_to_i) = parse_data("input.txt");
    let start = Instant::now();
    let dist = floyd_warshall(&valves, &v_to_i);
    let mut pending_valves: bitmaps::Bitmap<64> = bitmaps::Bitmap::new();

    let starting_valve = v_to_i.get("AA").unwrap();

    let mut cache: HashMap<(usize, bitmaps::Bitmap<64>, i32), u32> = HashMap::new();

    let max_flow = get_max_flow(&valves, &dist, *starting_valve, &mut pending_valves, 30, &mut cache);
    
    let duration = start.elapsed().as_millis();

    println!("Max flow: {max_flow} ({duration} ms)");
}

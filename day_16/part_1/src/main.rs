use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
struct Valve {
    name: String,
    flow_rate: u32,
    neighbors: Vec<String>, 
    turned_on: bool
}

impl Valve {
    fn new(name: &str, flow_rate: u32, neighbors: &Vec<String>) -> Self {
        Valve {
            name: String::from(name),
            flow_rate,
            neighbors: neighbors.clone(),
            false
        }
    }
}

fn parse_data(file_name: &str) -> HashMap<String, Valve> {
    let input = fs::read_to_string(file_name).unwrap();

    let mut valves: HashMap<String, Valve> = HashMap::new();
    
    for line in input.lines() {
        let name = line.trim().split(' ').skip(1).next().unwrap().to_string();
        let flow_rate: u32 = line.trim().split(&[' ', '=', ';', ','][..])
            .find_map(|x| x.parse::<u32>().ok()).unwrap();
        let neighbors: Vec<String> = line.trim().split(&[' ', ','])
            .skip(9) // skip "Valve", "<name>", "has", "flow" "rate=<flow_rate>;", "tunnels", "lead", "to", "valves"
            .map(|s| s.to_string())
            .filter(|s| s.len() > 0)
            .collect();
        valves.insert(name.clone(), Valve::new(&name, flow_rate, &neighbors));
    }

    valves
}

fn main() {
    let valves = parse_data("input.txt");

    // start at valve AA
     // at each valve, get the max of all your choices
        // turn valve on (if flow_rate > 0) and then proceed to neighbors
        // skip valve and proceed to all neighbors
    // Need to check for cycles????
        // mark valve as turned_on if you're going to use it, can check later on 
    for entry in valves {
        println!("{:?}", entry);
    }
}

use std::{fs, cmp, collections::HashMap};

#[derive(Debug)]
struct Cost {
    ore_cost: i32,
    clay_cost: i32,
    obsid_cost: i32
}

impl Cost {
    fn new(ore_cost: i32, clay_cost: i32, obsid_cost: i32) -> Self {
        Cost {
            ore_cost,
            clay_cost,
            obsid_cost
        }
    }
}

#[derive(Debug)]
struct Blueprint {
    ore_cost: Cost,
    clay_cost: Cost,
    obsid_cost: Cost,
    geode_cost: Cost
}

impl Blueprint {
    fn new(ore_cost: Cost, clay_cost: Cost,
           obsid_cost: Cost, geode_cost: Cost) -> Self {
        Blueprint {
            ore_cost,
            clay_cost,
            obsid_cost,
            geode_cost
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct SimState {
    time: i32,
    n_ore: i32,
    n_clay: i32,
    n_obsid: i32,
    ore_robot: i32,
    clay_robot: i32,
    obsid_robot: i32,
    geode_robot: i32
}

impl SimState {
    fn new(time: i32, n_ore: i32, n_clay: i32, n_obsid: i32, ore_robot: i32,
           clay_robot: i32, obsid_robot: i32, geode_robot: i32) -> Self {
        SimState {
            time,
            n_ore,
            n_clay,
            n_obsid,
            ore_robot,
            clay_robot,
            obsid_robot,
            geode_robot
        }
    }
}

fn get_blueprints(file_name: &str) -> Vec<Blueprint> {
    let input = fs::read_to_string(file_name)
        .expect("Failed to read the input file");

    let mut blueprints: Vec<Blueprint> = Vec::new();

    for blueprint in input.lines().take(3) {
        let params: Vec<&str> = blueprint
            .trim()
            .split(&[':', '.'][..])
            .skip(1)
            .filter(|s| s.len() > 0)
            .collect();

        assert!(params.len() == 4);

        let mut params = params.iter();
        let costs: Vec<i32> = params
            .next()
            .unwrap()
            .split_whitespace()
            .filter_map(|x| x.parse::<i32>().ok())
            .collect();
        assert!(costs.len() == 1);
        let ore_cost = Cost::new(costs[0], 0, 0);

        let costs: Vec<i32> = params
            .next()
            .unwrap()
            .split_whitespace()
            .filter_map(|x| x.parse::<i32>().ok())
            .collect();
        assert!(costs.len() == 1);
        let clay_cost = Cost::new(costs[0], 0, 0);

        let costs: Vec<i32> = params
            .next()
            .unwrap()
            .split_whitespace()
            .filter_map(|x| x.parse::<i32>().ok())
            .collect();
        assert!(costs.len() == 2);
        let obsid_cost = Cost::new(costs[0], costs[1], 0);

        let costs: Vec<i32> = params
            .next()
            .unwrap()
            .split_whitespace()
            .filter_map(|x| x.parse::<i32>().ok())
            .collect();
        assert!(costs.len() == 2);
        let geode_cost = Cost::new(costs[0], 0, costs[1]);

        blueprints.push(Blueprint::new(ore_cost, clay_cost, obsid_cost, geode_cost));
    }

    blueprints
}

fn mine_resources(state: &mut SimState) {
    state.n_ore += state.ore_robot;
    state.n_clay += state.clay_robot;
    state.n_obsid += state.obsid_robot;
}

fn unmine_resources(state: &mut SimState) {
    state.n_ore -= state.ore_robot;
    state.n_clay -= state.clay_robot;
    state.n_obsid -= state.obsid_robot;
}

fn max_sim(blueprint: &Blueprint, state: &mut SimState, 
                            cache: &mut HashMap<SimState, u32>) -> u32 {
    if state.time > 32 {
        return 0;
    }
    if let Some(num) = cache.get(&state) {
        return *num;
    }
   
    let mut geodes: u32 = 0; // push all the recursive results onto here
    // resource state after mining is done for this time step
    state.time += 1;

    // mine/unmine resources in state over recursive calls
    // go through all the choices we have at the present moment...
    // ...and find the max geodes we can get
    // can we afford an ore-collecting robot?
    if blueprint.ore_cost.ore_cost <= state.n_ore {
        mine_resources(state);
        state.ore_robot += 1;
        state.n_ore -= blueprint.ore_cost.ore_cost;
        geodes = cmp::max(geodes, max_sim(blueprint, state, cache));
        state.ore_robot -= 1;
        state.n_ore += blueprint.ore_cost.ore_cost;
        unmine_resources(state);
    }
    // can we afford a clay-collecting robot?
    if blueprint.clay_cost.ore_cost <= state.n_ore {
        mine_resources(state);
        state.clay_robot += 1;
        state.n_ore -= blueprint.clay_cost.ore_cost;
        geodes = cmp::max(geodes, max_sim(blueprint, state, cache));
        state.clay_robot -= 1;
        state.n_ore += blueprint.clay_cost.ore_cost;
        unmine_resources(state);
    }
    // Can we afford an obsidian-collecting robot?
    if blueprint.obsid_cost.ore_cost <= state.n_ore && 
        blueprint.obsid_cost.clay_cost <= state.n_clay {
        mine_resources(state);
        state.obsid_robot += 1;
        state.n_ore -= blueprint.obsid_cost.ore_cost;
        state.n_clay -= blueprint.obsid_cost.clay_cost;
        geodes = cmp::max(geodes, max_sim(blueprint, state, cache));
        state.obsid_robot -= 1;
        state.n_ore += blueprint.obsid_cost.ore_cost;
        state.n_clay += blueprint.obsid_cost.clay_cost;
        unmine_resources(state);
    }
    // Can we afford a geode-cracking robot?
    if blueprint.geode_cost.ore_cost <= state.n_ore && 
        blueprint.geode_cost.obsid_cost <= state.n_obsid {
        mine_resources(state);
        state.geode_robot += 1;
        state.n_ore -= blueprint.geode_cost.ore_cost;
        state.n_obsid -= blueprint.geode_cost.obsid_cost;
        geodes = cmp::max(geodes, max_sim(blueprint, state, cache));
        state.geode_robot -= 1;
        state.n_ore += blueprint.geode_cost.ore_cost;
        state.n_obsid += blueprint.geode_cost.obsid_cost;
        unmine_resources(state);
    }
    // What if we just saved?
    {
        mine_resources(state);
        geodes = cmp::max(geodes, max_sim(blueprint, state, cache));
        unmine_resources(state);
    }

    // most you can get by your choices, plus the amount as a result of 
    // your mining capabilities at the start of the time step
    let n_geodes = geodes + state.geode_robot as u32;
    state.time -= 1;
    cache.insert(state.clone(), n_geodes);

    return n_geodes;
}

fn get_max_geode(blueprint: &Blueprint) -> u32 {
    let mut state = SimState::new(1, 0, 0, 0, 1, 0, 0, 0);
    let mut cache: HashMap<SimState, u32> = HashMap::new();

    return max_sim(blueprint, &mut state, &mut cache);
}

fn main() {
    let blueprints = get_blueprints("input.txt");

    let mut max_geodes: Vec<u32> = Vec::new();
    for (i, blueprint) in blueprints.iter().enumerate() {
        println!("Running Blueprint #{}", i + 1);
        max_geodes.push(get_max_geode(&blueprint));
    }

    let total: u32 = max_geodes.iter().product();

    println!("Total: {}", total);
}

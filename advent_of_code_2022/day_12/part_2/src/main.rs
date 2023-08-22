// With significant help from https://doc.rust-lang.org/std/collections/binary_heap/index.html#

use std::fs;
use std::cmp::{Ordering, min};
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct State {
    cost: usize,
    position: usize
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct XYPos {
    pub x: usize,
    pub y: usize
}

#[derive(Debug, Clone, Copy)]
pub struct Node {
    pub node_id: usize,
    pub pos: XYPos,
    pub height: char,
}

impl Node {
    pub fn new(node_id: usize, pos: XYPos, height: char) -> Self {
        Node {
            node_id,
            pos,
            height
        }
    }
}

fn load_map() -> (Vec<Vec<char>>, Node, Node) {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut map: Vec<Vec<char>> = Vec::new();
    let mut start = Node {
                        node_id: 0,
                        pos: XYPos {
                            x: 0,
                            y: 0
                        },
                        height: 'A'
    };
    let mut end = Node {
                        node_id: 0,
                        pos: XYPos {
                            x: 0,
                            y: 0
                        },
                        height: 'A'
    };
    let mut node_id: usize = 0;


    for (i, line) in input.lines().enumerate() {
        map.push(Vec::new());
        for (j, point) in line.trim().chars().enumerate() {
            map[i].push( match point {
                'S' => {
                    start = Node {
                        node_id,
                        pos: XYPos {
                            x: j,
                            y: i
                        },
                        height: point
                    };
                    'a'
                },
                'E' => {
                    end = Node {
                        node_id,
                        pos: XYPos {
                            x: j,
                            y: i
                        },
                        height: point
                    };
                    'z'
                },
                height => height
            }
            );
            node_id += 1; 
        }
    }

    (map, start, end)
}

fn reachable(map: &Vec<Vec<char>>, src: XYPos, dest: XYPos) -> bool {
    let src_height = map[src.y][src.x];
    let dest_height = map[dest.y][dest.x];

    return src_height as i32 >= dest_height as i32 - 1;
}

fn get_neighbors(map: &Vec<Vec<char>>, pos: usize) -> Vec<usize> {
    let height = map.len();
    let width = map.first().unwrap().len();
    let grid_pos = XYPos {
        x: pos % width,
        y: pos / width
    };
    let mut neighbors = Vec::new();
    
    let mut neighbor_pos = grid_pos.clone();

    // up
    if neighbor_pos.y > 0 {
        neighbor_pos.y -= 1;
        if reachable(map, grid_pos, neighbor_pos) {
            neighbors.push(neighbor_pos.y * width + neighbor_pos.x); // recreate node_id from x,y
                                                                     // pos
        }
        neighbor_pos.y += 1;
    }

    // down
    if neighbor_pos.y < height - 1 {
        neighbor_pos.y += 1;
        if reachable(map, grid_pos, neighbor_pos) {
            neighbors.push(neighbor_pos.y * width + neighbor_pos.x); // recreate node_id from x,y
                                                                     // pos
        }
        neighbor_pos.y -= 1;
    }

    // left
    if neighbor_pos.x > 0 {
        neighbor_pos.x -= 1;        
        if reachable(map, grid_pos, neighbor_pos) {
            neighbors.push(neighbor_pos.y * width + neighbor_pos.x); // recreate node_id from x,y
                                                                     // pos
        }
        neighbor_pos.x += 1;
    }

    // right
    if neighbor_pos.x < width - 1 {
        neighbor_pos.x += 1;
        if reachable(map, grid_pos, neighbor_pos) {
            neighbors.push(neighbor_pos.y * width + neighbor_pos.x); // recreate node_id from x,y
                                                                     // pos
        }
        neighbor_pos.x -= 1;
    }
    
    neighbors
}

fn shortest_path(map: &Vec<Vec<char>>, start: Node, end: Node) -> Option<usize> {    
    let height = map.len();
    let width = map.first().unwrap().len();
    let mut dist: Vec<usize> = vec![usize::MAX; height * width];//(0..height * width).map(
    let mut heap = BinaryHeap::new();

    // At start, with 0 cost
    dist[start.node_id as usize] = 0;
    heap.push(State { cost: 0, position: start.node_id });

    while let Some(State { cost, position }) = heap.pop() {
        if position == end.node_id {
            return Some(cost);
        }

        // no reason to explore a less efficient path
        if cost > dist[position] {
            continue;
        }

        for neighbor in get_neighbors(map, position) {
            // cost of traversing an edge is always 1 since we just care about distance
            let next = State { cost: cost + 1, position: neighbor };

            if next.cost < dist[next.position] {
                dist[next.position] = next.cost;
                heap.push(next);
            }
        }
    }

    None
}

// Apply Dijkstra's Algorithm, treating the map as a directed graph, all connections with weight
// one
fn main() {
    let (map, _, end) = load_map();

    let mut start: Node;
    let mut node_id: usize = 0;
    let mut min_cost: Option<usize> = None;

    for (i, row) in map.iter().enumerate() {
        for (j, point) in row.iter().enumerate() {
            if point == &'a' {
                start = Node {
                            node_id,
                            pos: XYPos { x: j, y: i },
                            height: *point
                };
                match shortest_path(&map, start, end) {
                    Some(cost) => {
                        match min_cost {
                            Some(least_cost) => {
                                min_cost = Some(min(least_cost, cost));
                            },
                            None => {
                                min_cost = Some(cost);
                            }
                        }
                    },
                    None => {}
                }
            }
            node_id += 1;
        }
    }



    match min_cost {
        Some(x) => {
            println!("Shortest path has cost: {x}");
        },
        None => {
            println!("No path exists!");
        }
    }
}

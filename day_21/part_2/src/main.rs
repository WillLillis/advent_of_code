use std::fs;
use std::collections::HashMap;
// Tree stuff adapted from: https://levelup.gitconnected.com/rust-binary-tree-30efdd355b60

#[derive(Debug, Clone, Copy)]
enum Op<T> {
    Add,
    Sub,
    Mul,
    Div,
    Val(Option<T>),
    Eq // special case for the root monkey
}

// convenience type alias
type ChildNode<T> = Option<Box<Node<T>>>;

#[derive(Debug, Clone)]
struct Node<T> {
    left: ChildNode<T>,
    right: ChildNode<T>,
    op: Op<T>
}

struct BinTree<T> {
    head: Option<Node<T>>
}

impl<T> Node<T> {
    fn new(op: Op<T>, left: Node<T>, right: Node<T>) -> Self {
        Node::<T> {
            op,
            left: Some(Box::new(left)),
            right: Some(Box::new(right))
        }
    }

    fn add_node(left: Node<T>, right: Node<T>) -> Node<T> {
        Node::new(Op::Add, left, right)
    }

    fn sub_node(left: Node<T>, right: Node<T>) -> Node<T> {
        Node::new(Op::Sub, left, right)
    }
    
    fn mul_node(left: Node<T>, right: Node<T>) -> Node<T> {
        Node::new(Op::Mul, left, right)
    }
    
    fn div_node(left: Node<T>, right: Node<T>) -> Node<T> {
        Node::new(Op::Div, left, right)
    }

    fn eq_node(left: Node<T>, right: Node<T>) -> Node<T> {
        Node::new(Op::Eq, left, right)
    }

    fn val_node(val: Option<T>) -> Node<T> {
        Node { op: Op::Val(val), left: None, right: None }
    }
}

impl<T> BinTree<T> 
where
    T: std::ops::Add<T, Output = T> + std::ops::Sub<T, Output = T>
    +  std::ops::Mul<T, Output = T> + std::ops::Div<T, Output = T>
    +  std::cmp::PartialEq<i64> + Clone + Copy + std::fmt::Debug
{
    pub fn new(head: Node<T>) -> Self {
        BinTree::<T> { head: Some(head) }
    }
    
    pub fn collapse(node: &Box<Node<T>>) -> Option<T> {
        let mut r: Option<T> = None;
        let mut l: Option<T> = None;

        if let Op::Val(Some(val)) = node.op {
            return Some(val);
        }
        
        if let Some(left) = &node.left {
            l = BinTree::collapse(left);
        }
        
        if let Some(right) = &node.right {
            r = BinTree::collapse(right);
        }
        
        let (l, r) = match (l, r) {
            (Some(x), Some(y)) => (x, y),
            _ => {
                return None;
            }
        };
       
        match &node.op {
            Op::Add => { Some(l + r) },
            Op::Sub => { Some(l - r) },
            Op::Mul => { Some(l * r) },
            Op::Div => {
                if r == 0 { 
                    panic!("attempted divide-by-zero operation.")  
                }
                Some(l / r)
            },
            _ => {
                panic!("This code shouldn't be reachable!");
            }
        }
    }
}

#[derive(Debug)]
struct Monkey<T> {
    left: Option<String>,
    right: Option<String>,
    op: Op<T>
}

impl<T> Monkey<T> {
    fn new(left: Option<String>, right: Option<String>, op: Op<T>) -> Self {
        Monkey {
            left,
            right,
            op
        }
    }
}

fn get_monkeys(file_name: &str) -> HashMap<String, Monkey<i64>> {
    let input = fs::read_to_string(file_name)
        .expect("Failed to read the input file");

    let mut monkeys: HashMap<String, Monkey<i64>> = HashMap::new();

    for line in input.lines() {
        let mut parts = line
            .trim()
            .split(&[' ', ':'][..])
            .filter(|s| s.len() > 0);

        let name = parts.next().unwrap();
        let next = parts.next().unwrap();

        if let Ok(num) = next.parse::<i64>() {
            monkeys.insert(String::from(name), Monkey::new(None, None, Op::Val(Some(num))));
        } else {
            let name_1 = Some(String::from(next));
            let mut op = match parts.next().unwrap() {
                "+" => Op::Add,
                "-" => Op::Sub,
                "*" => Op::Mul,
                "/" => Op::Div,
                _ => {
                    panic!("Parsing error!");
                }
            };
            // special case for part 2
            if name == "root" {
                op = Op::Eq;
            }
            let name_2 = Some(String::from(parts.next().unwrap()));
            
            
            let monkey = Monkey {
                left: name_1,
                right: name_2,
                op
            };

            monkeys.insert(String::from(name), monkey);
        }
    }

    return monkeys;
}

fn create_node(monkeys: &HashMap<String, Monkey<i64>>, name: &str) -> Node<i64> {
    let info = monkeys.get(name).unwrap();

    match info.op {
        Op::Add => {
            let left_name = info.left.clone().unwrap();
            let right_name = info.right.clone().unwrap();
            Node::add_node(create_node(monkeys, &left_name), create_node(monkeys, &right_name))
        },
        Op::Sub => {
            let left_name = info.left.clone().unwrap();
            let right_name = info.right.clone().unwrap();
            Node::sub_node(create_node(monkeys, &left_name), create_node(monkeys, &right_name))
        },
        Op::Mul => {
            let left_name = info.left.clone().unwrap();
            let right_name = info.right.clone().unwrap();
            Node::mul_node(create_node(monkeys, &left_name), create_node(monkeys, &right_name))
        },
        Op::Div => {
            let left_name = info.left.clone().unwrap();
            let right_name = info.right.clone().unwrap();
            Node::div_node(create_node(monkeys, &left_name), create_node(monkeys, &right_name))
        },
        Op::Val(x) => {
            // special case: ignore the "humn" value for part 2
            if name == "humn" {
                Node::val_node(None)
            } else {
                Node::val_node(x)
            }
        },
        Op::Eq => {
           if name != "root" {
                panic!("Only the root node should have operation of type Eq");
           }
           let left_name = info.left.clone().unwrap();
           let right_name = info.right.clone().unwrap();
           Node::eq_node(create_node(monkeys, &left_name), create_node(monkeys, &right_name))
        }
    }
}

// Inspiration for algo's track_val from: https://github.com/frhel/AdventOfCode/blob/master/2022/day_21/js/index.js#L121
fn solve(node: &Box<Node<i64>>, track_val: i64) -> i64 {
    // find which subtree side is collapsible
    // do the inverse op on the track_val with that collapsed value
    // and then recurse on the other subtree side
   
    match node.op {
        Op::Val(None) => {
            return track_val;
        },
        _ => {}
    }

    let left_side: bool;
    let side_val = match BinTree::collapse(&node.left.clone().unwrap()) {
        Some(val) => { 
            left_side = true;
            val 
        },
        None => {
            // Failed to evaluate the left subtree, checking the right...
            match BinTree::collapse(&node.right.clone().unwrap()) {
                Some(val) => { 
                    left_side = false;
                    val 
                },
                None => {
                    panic!("Failed to evaluate either subtree");
                }
            }
        }
    };

    return match node.op {
        Op::Add => {
            // Inverse of addition is subtraction...
            if left_side { 
                solve(&node.right.clone().unwrap(), track_val - side_val)
            } else { 
                solve(&node.left.clone().unwrap(), track_val - side_val) 
            }
        },
        Op::Sub => {
            // Careful! Subtraction isn't commutative
            if left_side {
                solve(&node.right.clone().unwrap(), -(track_val - side_val))
            } else {
                solve(&node.left.clone().unwrap(), track_val + side_val)
            }
        },
        Op::Mul => {
            // Inverse of multiplication is division
            if left_side {
                solve(&node.right.clone().unwrap(), track_val / side_val)
            } else {
                solve(&node.left.clone().unwrap(), track_val / side_val)
            }
        },
        Op::Div => {
            // Inverse of division is multiplication
            if left_side {
                solve(&node.right.clone().unwrap(), track_val * side_val)
            } else {
                solve(&node.left.clone().unwrap(), track_val * side_val)
            }
        },
        Op::Val(None) => {
            panic!("Reached the humn node in an unexpected way");
        },
        Op::Val(Some(_)) => {
            panic!("Expected expression, found value.");
        }
        Op::Eq => {
            panic!("Only the root monkey should have Op type Eq");
        }
    }
}

fn main() {
    let monkeys = get_monkeys("input.txt");
    let root = create_node(&monkeys, "root");
    
    // try to eval each side of the tree next
    let left_side: bool;
    let side_val = match BinTree::collapse(&root.left.clone().unwrap()) {
        Some(val) => { 
            left_side = true;
            val 
        },
        None => {
            println!("Failed to evaluate the left subtree, checking the right...");
            
            match BinTree::collapse(&root.right.clone().unwrap()) {
                Some(val) => { 
                    left_side = false;
                    val 
                },
                None => {
                    println!("Failed to evaluate the right subtree!");
                    panic!("Failed to evaluate either subtree");
                }
            }
        }
    };

    println!("Evaluated the {} side: {}", if left_side {"left"} else {"right"}, side_val);

    let human_val;
    if left_side {
        human_val = solve(&root.right.clone().unwrap(), side_val);
    } else {
        human_val = solve(&root.left.clone().unwrap(), side_val);
    }

    println!("The human value should be {}", human_val);
}

use std::fs;
use std::collections::HashMap;
// https://levelup.gitconnected.com/rust-binary-tree-30efdd355b60

#[derive(Debug, Clone, Copy)]
enum Op<T> {
    Add,
    Sub,
    Mul,
    Div,
    Val(Option<T>)
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
        
        // will this work if we don't fully collapse
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

    // solve func for part 2
    // traverse the human side of the tree, with a "Tracked number" equal to the other side's val
        // at each node, check if either side is collapsible to a value
        // if it is, do the inverse operation of the node between the tracked val and that
        // collapsed val, then go down to the child that wasn't collapsible
            // if it's the human node, then we return the tracked value
            // if it's another node, repeat the above procedure
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
            let op = match parts.next().unwrap() {
                "+" => Op::Add,
                "-" => Op::Sub,
                "*" => Op::Mul,
                "/" => Op::Div,
                _ => {
                    panic!("Parsing error!");
                }
            };
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
            Node::val_node(x)
        }
    }

}

fn main() {
    let monkeys = get_monkeys("input.txt");
    
    let root_info = monkeys.get("root").expect("No root monkey found!");

    let mut root = Node {
        left: None,
        right: None,
        op: root_info.op
    };

    root.left = Some(Box::new(create_node(&monkeys, &root_info.left.clone().unwrap())));
    root.right = Some(Box::new(create_node(&monkeys, &root_info.right.clone().unwrap())));

    for monkey in monkeys {
        println!("{:?}", monkey);
    }

    println!("\n\n\n\nThe moment of truth:\n\n\n");
    println!("{:#?}", root);

    match BinTree::collapse(&Box::new(root)) {
        Some(val) => {
            println!("The monkeys will shout {val}");
        },
        None => {
            println!("Failed to evaluate the tree");
        }
    }

}



/*fn create_node<T>(monkeys: &HashMap<String, Monkey<T>>, name: &str) -> Node<T>
where
T: Copy
{
    // special case for humn entry
   
    // if it's a value monkey create a leaf node and return
    // if it's an expression monkey create and expression node and recurse to the left and right
    let info = monkeys.get(name).unwrap();
    let op = info.op;
    let left_name = info.operand_1;
    let right_name = info.operand_2;

    Node {
        left: create_node(monkeys, left_name),
        right: create_node(monkeys, right_name),
        op
    }

}*/

/*fn main() {
    // (10 - (2 * 2)) + (8 + (10 / 2))
    let test_tree = BinTree::new(
        Node::add_node(
            Node::sub_node(
            let left_name = info.left.clone().unwrap();
            let right_name = info.right.clone().unwrap();
                Node::val_node(Some(10)),
                Node::mul_node(
                    Node::val_node(Some(2)),
                    Node::val_node(Some(2))
                )
            ),
            Node::add_node(
                Node::val_node(Some(8)),
                Node::div_node(
                    Node::val_node(Some(10)),
                    Node::val_node(Some(2))
                )
            )
        )
    );

    //match BinTree::collapse(&Box::new(test_tree.head.unwrap())) {
    match BinTree::collapse(&test_tree.head.unwrap().left.unwrap()) {
        Some(val) => {
            println!("Evaluated tree to {val}.");
        },
        None => {
            println!("Failed to evaluate the tree.");
        }
    }
}*/

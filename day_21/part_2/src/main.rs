use std::fs;
// https://levelup.gitconnected.com/rust-binary-tree-30efdd355b60

#[derive(Debug)]
enum Op<T> {
    Add,
    Sub,
    Mul,
    Div,
    Val(T)
}

// convenience type alias
type ChildNode<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
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

    fn val_node(val: T) -> Node<T> {
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

        if let Op::Val(val) = node.op {
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
            Op::Val(x) => Some(x.clone()) // shouldn't ever reach here 
        }
    }
}

fn main() {
    // (10 - (2 * 2)) + (8 + (10 / 2))
    let test_tree = BinTree::new(
        Node::add_node(
            Node::sub_node(
                Node::val_node(10),
                Node::mul_node(
                    Node::val_node(2),
                    Node::val_node(2)
                )
            ),
            Node::add_node(
                Node::val_node(8),
                Node::div_node(
                    Node::val_node(10),
                    Node::val_node(2)
                )
            )
        )
    );

    match BinTree::collapse(&Box::new(test_tree.head.unwrap())) {
        Some(val) => {
            println!("Evaluated tree to {val}.");
        },
        None => {
            println!("Failed to evaluate the tree.");
        }
    }
}

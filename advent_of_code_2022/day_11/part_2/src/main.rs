use std::{fs, collections::VecDeque};

#[derive(Debug)]
pub enum MonkeyOp {
    AddByVal(u128),
    AddBySelf,
    MultByVal(u128),
    MultBySelf
}

impl MonkeyOp {
    fn new(op_char: char, op_num: Option<u128>) -> Self {
        match op_num {
            Some(num) => {
                match op_char {
                    '+' => MonkeyOp::AddByVal(num),
                    '*' => MonkeyOp::MultByVal(num),
                    _ => {
                        panic!("Invalid character passed to constructor!");
                    }
                }
            },
            None => {
                match op_char {
                    '+' => MonkeyOp::AddBySelf,
                    '*' => MonkeyOp::MultBySelf,
                    _ => {
                        panic!("Invalid character passed to constructor!");
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Monkey {
    pub op: MonkeyOp,
    pub items: VecDeque<u128>,
    pub test_num: u128,
    pub true_dest: usize,
    pub false_dest: usize,
    pub n_inspect: u128
}

impl Monkey {
    pub fn new(op: MonkeyOp, items: VecDeque<u128>, test_num: u128, true_dest: usize,
               false_dest: usize) -> Self {
        Monkey {
            op,
            items,
            test_num,
            true_dest,
            false_dest,
            n_inspect: 0
        }
    }
}


pub fn monkey_file_to_vec() -> Vec<Monkey> {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut input = input.lines();
    let mut monkeys: Vec<Monkey> = Vec::new();

    loop {
        let monkey_id;
        match input.next() {
            Some(x) => {
                if x.len() < 4 { // blank line separating entries
                    monkey_id = input.next().unwrap();
                } else { // actual monkey id entry
                    monkey_id = x;
                }
            },
            None => {
                break;
            }
        }
        let _monkey_id = monkey_id.split(&[' ', ':'][..]).skip(1)
            .next().unwrap().parse::<u128>().unwrap();
        let starting_items = input.next().unwrap();
        let starting_items: VecDeque<u128> = starting_items
            .split(&[':', ','][..])
            .into_iter()
            .skip(1) // skip "Starting items"
            .map(|x| x.trim().parse::<u128>().unwrap())
            .collect();
        let op = input.next().unwrap();
        let op: Vec<&str> = op
            .split_inclusive(&['+', '*'][..])
            .collect();
        let op_num: Option<u128> = match op.last().unwrap().trim().parse::<u128>() {
            Ok(x) => Some(x),
            _ => None,
        };
        let op = MonkeyOp::new(
                    op.first().unwrap().chars().last().unwrap(), 
                    op_num
                );
        let test_num = input.next().unwrap();
        let test_num: u128 = test_num
            .split_whitespace()
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .parse::<u128>()
            .unwrap();
        let true_dest = input.next().unwrap();
        let true_dest = true_dest
            .split_whitespace()
            .last()
            .unwrap()
            .trim()
            .parse::<usize>()
            .unwrap();
        let false_dest = input.next().unwrap();
        let false_dest = false_dest
            .split_whitespace()
            .last()
            .unwrap()
            .trim()
            .parse::<usize>()
            .unwrap();

        monkeys.push(
            Monkey::new(
                op,
                starting_items,
                test_num,
                true_dest,
                false_dest
            )
        );
    }
    
    monkeys
}

pub fn simulate_round(monkeys: &mut Vec<Monkey>) {
    for idx in 0..monkeys.len() {
        while !monkeys[idx].items.is_empty() {
            let mut item = monkeys[idx].items.pop_front().unwrap();
            monkeys[idx].n_inspect += 1;
            // update worry level according to monkey's action
            item = match monkeys[idx].op {
                MonkeyOp::AddByVal(x) => item + x,
                MonkeyOp::AddBySelf => item + item,
                MonkeyOp::MultByVal(x) => item * x,
                MonkeyOp::MultBySelf => {
                    // do this to prevent overflow...
                    let tmp = item * item;
                    if tmp > 9699690 { // LCM of all the divisibility rules
                        tmp % 9699690
                    } else {
                        tmp
                    }
                }
            };
            // move the item according to the monkey's rule
            let dest;
            if item % monkeys[idx].test_num == 0 {
                dest = monkeys[idx].true_dest;
            } else {
                dest = monkeys[idx].false_dest;
            }
            monkeys[dest].items.push_back(item);
        }
    }
}

fn main() {
    let mut monkeys = monkey_file_to_vec();
   
    for _ in 0..10000 {
        simulate_round(&mut monkeys);
    }

    let mut inspections: Vec<u128> = Vec::new();
    for (idx, monkey) in monkeys.iter().enumerate() {
        println!("Monkey {idx} inspected items {} times",
                 monkey.n_inspect);
        inspections.push(monkey.n_inspect);
    }

    inspections.sort_by(|a,b| b.cmp(a));
    let monkey_business = inspections.iter().take(2).product::<u128>();
    println!("Monkey business: {monkey_business}");
}

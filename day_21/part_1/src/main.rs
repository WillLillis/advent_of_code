use std::fs;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum BinaryOp {
    Add,
    Sub,
    Mult,
    Div
}

#[derive(Debug, Clone)]
enum MonkeyOperand {
    Num(i64),
    Ref(String)
}

impl MonkeyOperand {
    fn get_val(&self) -> Option<&i64> {
        match self {
            MonkeyOperand::Num(num) => Some(num),
            MonkeyOperand::Ref(_) => None
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    operand_1: MonkeyOperand,
    operand_2: MonkeyOperand,
    op: BinaryOp
}

fn get_monkeys(file_name: &str) -> (HashMap<String, Monkey>, HashMap<String, i64>) {
    let input = fs::read_to_string(file_name)
        .expect("Failed to read the input file");

    let mut monkeys: HashMap<String, Monkey> = HashMap::new();
    let mut resolved: HashMap<String, i64> = HashMap::new();

    for line in input.lines() {
        let mut parts = line
            .trim()
            .split(&[' ', ':'][..])
            .filter(|s| s.len() > 0);

        let name = parts.next().unwrap();
        let next = parts.next().unwrap();

        if let Ok(num) = next.parse::<i64>() {
            resolved.insert(String::from(name), num);
        } else {
            let name_1 = MonkeyOperand::Ref(String::from(next));
            let op = match parts.next().unwrap() {
                "+" => BinaryOp::Add,
                "-" => BinaryOp::Sub,
                "*" => BinaryOp::Mult,
                "/" => BinaryOp::Div,
                _ => {
                    panic!("Parsing error!");
                }
            };
            let name_2 = MonkeyOperand::Ref(String::from(parts.next().unwrap()));
            
            
            let monkey = Monkey {
                operand_1: name_1,
                operand_2: name_2,
                op
            };

            monkeys.insert(String::from(name), monkey);
        }
    }

    return (monkeys, resolved);
}

fn resolve_dependencies(monkeys: &mut HashMap<String, Monkey>, resolved: &mut HashMap<String, i64>) {
    // need to rethink this, can't do straightforward in place manipulation
    // because of the borrow checker
    let mut changed = true;
    let mut to_remove = None;
    while changed {
        changed = false;
        for (name, exp) in monkeys.iter_mut() {
            to_remove = None;
            let op_1 = exp.operand_1.clone();
            match op_1 {
                MonkeyOperand::Ref(ref_name) => {
                    match resolved.get(&ref_name) {
                        Some(val) => {
                            changed = true;
                            exp.operand_1 = MonkeyOperand::Num(*val);
                        },
                        None => {}
                    }
                },
                _ => {}
            }
            let op_2 = exp.operand_2.clone();
            match op_2 {
                MonkeyOperand::Ref(ref_name) => {
                    match resolved.get(&ref_name) {
                        Some(val) => {
                            changed = true;
                            exp.operand_2 = MonkeyOperand::Num(*val);
                        },
                        None => {}
                    }
                },
                _ => {}
            }

            let val_1 = exp.operand_1.get_val();
            let val_2 = exp.operand_2.get_val();

            let val = match (val_1, val_2) {
                (Some(val_1), Some(val_2)) => {    
                    match exp.op {
                        BinaryOp::Add => {
                            Some(val_1 + val_2)
                        },
                        BinaryOp::Sub => {
                            Some(val_1 - val_2)
                        },
                        BinaryOp::Mult => {
                            Some(val_1 * val_2)
                        },
                        BinaryOp::Div => {
                            Some(val_1 / val_2)
                        }
                    }
                },
                _ => { None }
            };

            match val {
                Some(x) => { 
                    resolved.insert(String::from(name), x); 
                    to_remove = Some(name);
                    //break;
                },
                None => {}
            }
        }
        // can't remove the resolved entry because it constitutes a double borrow, not sure how to
        // get around this
        //match to_remove {
        //    Some(name) => {
        //        monkeys.remove(name);
        //    },
        //    None => {}
        //}
    }
    
}

fn main() {
    let (mut monkeys, mut resolved) = get_monkeys("input.txt");

    resolve_dependencies(&mut monkeys, &mut resolved);

    match resolved.get("root") {
        Some(num) => {
            println!("The root monkey will shout {num}");
        },
        None => {
            println!("Failed to resolve the dependencies");
        }
    }
}

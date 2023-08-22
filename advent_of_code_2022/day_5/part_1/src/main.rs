use std::fs;

fn init_stacks(input: &String) -> Vec<Vec<char>> {
    let input: Vec<&str> = input.lines().take_while(
        |x| x.trim().len() > 2).collect();

    let n_stacks = (input.last().unwrap().len() + 1) / 4;
    //println!("number of stacks: {n_stacks}");

    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); n_stacks];

    for line in input.iter().rev().skip(1) {
        let iter = line.chars().skip(1).step_by(4);
        for x in iter.enumerate() {
            match x {
                (_, ' ') => {},
                (idx, x) => {
                    stacks[idx].push(x);
                }
            }
        }
    }

    stacks
}



fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read the file");

    let mut stacks = init_stacks(&input); // get the initial state of the stacks
    let input: Vec<&str> = input.lines().skip_while( // grab the input...
        |x| x.trim().len() > 2).skip(1).collect(); // skipping the state description

    let mut steps: Vec<&str>;
    let mut src: usize;
    let mut dest: usize;
    let mut quant: usize;

    // go through the move instructions
    for line in input {
        //println!("{line}");
        steps = line.trim().split(char::is_lowercase).filter(
            |x| x.len() > 0).collect();
        //println!("\t{:?}", steps);
        
        assert!(steps.len() == 3);
        quant = steps[0].trim().parse::<usize>().expect("Failed to parse the quantity");
        src   = steps[1].trim().parse::<usize>().expect("Failed to parse the source");
        dest  = steps[2].trim().parse::<usize>().expect("Failed to parse the destination");

        for _ in 0..quant {
            let item: char = stacks[src - 1].pop().expect("Unexpected empty stack!");
            stacks[dest - 1].push(item);
        }
    }

    //println!("Final state:");
    //for (i, stack) in stacks.iter().enumerate() {
    //    println!("{i}: {:?}", &stack);
    //}

    print!("Top crates: ");
    for stack in stacks.iter() {
        print!("{}", stack.last().expect("Empty stack!"));
    }
    println!("");
}

use std::fs;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read the file");

    let mut items: HashMap<char, u32> = HashMap::new();
    let mut total_priority: u32 = 0;
    let mut elf_1 = input.lines().step_by(3);
    let mut elf_2 = input.lines().skip(1).step_by(3);
    let mut elf_3 = input.lines().skip(2).step_by(3);
    let mut bag_1: &str;
    let mut bag_2: &str;
    let mut bag_3: &str;

    loop {
        bag_1 = match elf_1.next() {
            Some(x) => x,
            None => {break;}
        };
        bag_2 = match elf_2.next() {
            Some(x) => x,
            None => {break;}
        };
        bag_3 = match elf_3.next() {
            Some(x) => x,
            None => {break;}
        };

        // just insert 1
        for item in bag_1.chars() {
            items.insert(item, 1);
        }

        // if it's 1, make it 2, otherwise we don't care
        for item in bag_2.chars() {
            items.entry(item).and_modify(|count| 
                if *count == 1 {
                    *count = 2;
                }
            );
        }

        // find the entry with a value of 2
        for item in bag_3.chars() {
            match items.get(&item) {
                Some(2) => {
                    //println!("\tDuplicate item: {item}");
                    total_priority += if item.is_ascii_lowercase() {
                        1 + item as u32 - 'a' as u32
                    } else if item.is_ascii_uppercase() {
                        27 + item as u32 - 'A' as u32
                    } else {
                        panic!("Invalid character!");
                    };
                    break;
                },
                _ => {}
            }
        }

        items.clear();
    };

    println!("Total total priority: {}", total_priority);
}

use std::{fs, cmp};

fn get_notes(file_name: &str) -> (Vec<Vec<char>>, String) {
    let input = fs::read_to_string(file_name)
        .expect("Failed to read the input file");

    // need to get count and add padding
    let mut map:Vec<Vec<char>> = input
        .lines()
        .take_while(|s| !s.is_empty())
        .map(|s| s.chars().collect())
        .collect();

    // padding added, not sure if this will be helpful or not
    let max_len = map.iter().fold(0, |max, x| cmp::max(max, x.len()));
    
    for i in 0..map.len() {
        let len = map[i].len();
        if len < max_len {
            map[i].append(&mut vec![' '; max_len - len]);
        }
    
    }


    println!("Max len: {:?}", max_len);

    for thing in &map {
        println!("{:?}", thing);
    }

    let path = String::from(input.lines()
                            .into_iter()
                            .skip_while(|s| !s.is_empty())
                            .skip(1)
                            .collect::<String>());

    println!("\n\n\n{}", path);

    return (map, path);
}


fn main() {

    let (map, path) = get_notes("test_input.txt"); 
    
}

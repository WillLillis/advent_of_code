use std::{fs, process};

#[derive(Debug)]
struct XY {
    x: i32,
    y: i32,
}

// first get the basic sim step working
// then work out a way to efficiently search the possible v_0 vectors

fn get_target_area(file_name: &str) -> (XY, XY, XY, XY) {
    let input = fs::read_to_string(file_name).unwrap_or_else(|err| {
        eprintln!("Error occurred while opening file 1: {err}");
        process::exit(1);
    });

    let vals: Vec<i32> = input
        .trim()
        .split(&[' ', '.', ',', '='])
        .filter_map(|x| x.parse::<i32>().ok())
        .collect();
    assert!(vals.len() == 4);

    let bottom_left = XY {
        x: vals[0],
        y: vals[2],
    };
    let top_left = XY {
        x: vals[0],
        y: vals[3],
    };
    let bottom_right = XY {
        x: vals[1],
        y: vals[2],
    };
    let top_right = XY {
        x: vals[1],
        y: vals[3],
    };

    return (bottom_left, top_left, bottom_right, top_right);
}

fn main() {
    let corners = get_target_area("test_input.txt");

    println!("{:?}", corners);
}

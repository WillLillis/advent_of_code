use std::{fs, collections::VecDeque};

#[derive(Debug)]
struct BITSTransmission {
    first_packet: Packet,
}

#[derive(Debug)]
struct Packet {
    header: PacketHeader,
    body: PacketBody,
}

#[derive(Debug)]
struct PacketHeader {
    version: u8,
    type_id: u8,
}

#[derive(Debug)]
enum PacketBody {
    Literal(u128),
    Operator(Vec<Box<PacketBody>>),
}

fn hex_to_bin(hex_in: char) -> Option<[bool; 4]> {
    println!("{hex_in}->{}", hex_in.to_ascii_uppercase());
    match hex_in.to_ascii_uppercase() {
        '0' => Some([false, false, false, false]),
        '1' => Some([false, false, false, true]),
        '2' => Some([false, false, true, false]),
        '3' => Some([false, false, true, true]),
        '4' => Some([false, true, false, false]),
        '5' => Some([false, true, false, true]),
        '6' => Some([false, true, true, false]),
        '7' => Some([false, true, true, true]),
        '8' => Some([true, false, false, false]),
        '9' => Some([true, false, false, true]),
        'A' => Some([true, false, true, false]),
        'B' => Some([true, false, true, true]),
        'C' => Some([true, true, false, false]),
        'D' => Some([true, true, false, true]),
        'E' => Some([true, true, true, false]),
        'F' => Some([true, true, true, true]),
        _ => None
    }
}

fn get_transmission(file_name: &str) -> VecDeque<bool> {
    let input = fs::read_to_string(file_name).expect("Failed to read the input file.");

    let mut binary: VecDeque<bool> = VecDeque::new();

    for c in input.trim().chars() {
        match hex_to_bin(c) {
            Some(bin_vals) => {
                for val in bin_vals {
                    binary.push_back(val);
                }
            }, 
            None => {
                panic!("Error parsing the file! Invalid character encountered");
            }
        }
    }

    return binary;
}

fn parse_header(transmission: &mut VecDeque<bool>) -> PacketHeader {
    let ver_2 = transmission.pop_front().expect("Unexpected packet end!");
    let ver_1 = transmission.pop_front().expect("Unexpected packet end!");
    let ver_0 = transmission.pop_front().expect("Unexpected packet end!");

    let version: u8 = (if ver_2 {4} else {0}) + (if ver_1 {2} else {0}) + (if ver_0 {1} else {0});

    let id_2 = transmission.pop_front().expect("Unexpected packet end!");
    let id_1 = transmission.pop_front().expect("Unexpected packet end!");
    let id_0 = transmission.pop_front().expect("Unexpected packet end!");

    return match (id_2, id_1, id_0) {
        (true, false, false) => PacketHeader { version, type_id: 4u8 },
        _ => {
            let type_id: u8 = (if id_2 {4} else {0}) + (if id_1 {2} else {0}) + (if id_0 {1} else {0});
            PacketHeader { version, type_id }
        }
    };
}

fn parse_body_literal(transmission: &mut VecDeque<bool>) -> PacketBody {
    let mut literal_val: Vec<bool> = Vec::new();

    loop {
        let last_indic = transmission.pop_front().expect("Unexpected packet end!");
        for _ in 0..4 {
            literal_val.push(transmission.pop_front().expect("Unexpected packet end!"));
        }
        if !last_indic {
            break;
        }
    }

    let mut coeff = 1;
    let mut accum = 0;
    for &val in literal_val.iter().rev() {
        accum += if val {coeff} else {0};
        coeff *= 2;
    }

    return PacketBody::Literal(accum);
}

fn parse_body_operator(transmission: &mut VecDeque<bool>) -> PacketBody {
    todo!();
}

fn parse_packet(transmission: &mut VecDeque<bool>) -> Packet {
    let header = parse_header(transmission);
    
    let body = if header.type_id == 4 {
        parse_body_literal(transmission)
    } else {
        parse_body_operator(transmission)
    };

    return Packet { header, body };
}

fn parse_transmission(transmission: &mut VecDeque<bool>) -> BITSTransmission {
    return BITSTransmission { first_packet: parse_packet(transmission) };
}

fn main() {
    let mut transmission = get_transmission("test_input_0.txt");

    let parsed = parse_transmission(&mut transmission);

    println!("{:#?}", parsed);
}

use std::{collections::VecDeque, fs, mem::swap};

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
    Operator(Vec<Packet>),
}

fn hex_to_bin(hex_in: char) -> Option<[bool; 4]> {
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
        _ => None,
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
            }
            None => {
                panic!("Error parsing the file! Invalid character encountered");
            }
        }
    }

    return binary;
}

fn parse_header(transmission: &mut VecDeque<bool>) -> Option<PacketHeader> {
    let ver_2 = match transmission.pop_front() {
        Some(bit) => bit,
        None => {
            return None;
        }
    };
    let ver_1 = match transmission.pop_front() {
        Some(bit) => bit,
        None => {
            return None;
        }
    };
    let ver_0 = match transmission.pop_front() {
        Some(bit) => bit,
        None => {
            return None;
        }
    };

    let version: u8 =
        (if ver_2 { 4 } else { 0 }) + (if ver_1 { 2 } else { 0 }) + (if ver_0 { 1 } else { 0 });

    let id_2 = match transmission.pop_front() {
        Some(bit) => bit,
        None => {
            return None;
        }
    };
    let id_1 = match transmission.pop_front() {
        Some(bit) => bit,
        None => {
            return None;
        }
    };
    let id_0 = match transmission.pop_front() {
        Some(bit) => bit,
        None => {
            return None;
        }
    };

    
    let type_id: u8 = (if id_2 { 4 } else { 0 })
        + (if id_1 { 2 } else { 0 })
        + (if id_0 { 1 } else { 0 });
     return Some(PacketHeader { version, type_id });
}

fn parse_body_literal(transmission: &mut VecDeque<bool>) -> Option<PacketBody> {
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
        accum += if val { coeff } else { 0 };
        coeff *= 2;
    }

    return Some(PacketBody::Literal(accum));
}

fn parse_body_operator(transmission: &mut VecDeque<bool>) -> Option<PacketBody> {
    let len_type_id = transmission.pop_front().expect("Unexpected packet end!");

    if len_type_id {
        let mut literal_val: Vec<bool> = Vec::new();
        for _ in 0..11 {
            match transmission.pop_front() {
                Some(val) => {
                    literal_val.push(val);
                }
                None => {
                    return None;
                }
            }
        }

        let mut n_packets = 0;
        {
            let mut coeff = 1;
            for &val in literal_val.iter().rev() {
                n_packets += if val { coeff } else { 0 };
                coeff *= 2;
            }
        }

        // check for 0 necessary?
        if n_packets == 0 {
            return None;
        }
        let mut sub_packets: Vec<Packet> = Vec::new();
        sub_packets.reserve(n_packets);

        for _ in 0..n_packets {
            match parse_packet(transmission) {
                Some(packet) => {
                    sub_packets.push(packet);
                }
                None => {}
            }
        }

        return Some(PacketBody::Operator(sub_packets));
    } else {
        let mut literal_val: Vec<bool> = Vec::new();
        for _ in 0..15 {
            match transmission.pop_front() {
                Some(val) => {
                    literal_val.push(val);
                }
                None => {
                    return None;
                }
            }
        }

        let mut sub_packets_len = 0;
        {
            let mut coeff = 1;
            for &val in literal_val.iter().rev() {
                sub_packets_len += if val { coeff } else { 0 };
                coeff *= 2;
            }
        }

        if sub_packets_len == 0 {
            return None;
        }

        let mut sub_transmission = transmission.split_off(sub_packets_len);
        swap(&mut sub_transmission, transmission);

        let mut sub_packets: Vec<Packet> = Vec::new();
        while !sub_transmission.is_empty() {
            match parse_packet(&mut sub_transmission) {
                Some(packet) => {
                    sub_packets.push(packet);
                }
                None => {}
            }
        }

        return Some(PacketBody::Operator(sub_packets));
    }
}

fn parse_packet(transmission: &mut VecDeque<bool>) -> Option<Packet> {
    let header = match parse_header(transmission) {
        Some(contents) => contents,
        None => {
            return None;
        }
    };

    let body = if header.type_id == 4 {
        parse_body_literal(transmission)
    } else {
        parse_body_operator(transmission)
    };

    return match body {
        Some(contents) => Some(Packet {
            header,
            body: contents,
        }),
        None => None,
    };
}

fn parse_transmission(transmission: &mut VecDeque<bool>) -> Option<BITSTransmission> {
    return match parse_packet(transmission) {
        Some(first_packet) => Some(BITSTransmission { first_packet }),
        None => None,
    };
}

fn get_version_sum(transmission: &Packet) -> u32 {
    let mut version_accum = 0u32;
    version_accum += transmission.header.version as u32;
    match &transmission.body {
        PacketBody::Literal(_) => {}
        PacketBody::Operator(sub_packets) => {
            for packet in sub_packets.iter() {
                version_accum += get_version_sum(packet);
            }
        }
    }

    return version_accum;
}

fn main() {
    let mut transmission = get_transmission("input.txt");

    let parsed = parse_transmission(&mut transmission).unwrap();
    let version_accum = get_version_sum(&parsed.first_packet);

    //println!("{:#?}", parsed);
    println!("{version_accum}");
}

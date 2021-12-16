use std::{str::Chars, iter::Peekable};

fn main() {
    let data = hex_to_binary(&std::fs::read_to_string("./input.txt").unwrap());
    let mut packet = data.chars().peekable();
    println!("Part 1 {}", parse_packet(&mut packet, 1));
    let mut packet = data.chars().peekable();
    println!("Part 2 {}", part2(&mut packet));
}


fn parse_packet(packet: &mut Peekable<Chars>, mut packets: i32) -> i64{

    let mut version_total = 0;
    while packets > 0 {
        let version = i64::from_str_radix(&take_n(packet, 3), 2).unwrap();
        let type_id = i64::from_str_radix(&take_n(packet, 3), 2).unwrap();

        // println!("{} {}", version, type_id);
        if type_id == 4 { // literal packet
            let mut result: String = String::new();
            let mut first_byte = 1;
            while first_byte == 1{
                    let next_chunk = take_n(packet, 5);
                    result.push_str(&next_chunk[1..]);
                    first_byte = i32::from_str_radix(&next_chunk[0..1], 2).unwrap();
            }

            // return i64::from_str_radix(&result, 2).unwrap()
        } else { // operator packet

            if take_n(packet, 1) == "0" {
                // next 15 bits are total length in bits of sub-packet
                let packet_bit_count = usize::from_str_radix(&take_n(packet, 15), 2).unwrap();
                let next_chars = take_n(packet, packet_bit_count);
                let mut packet_bits = next_chars.chars().peekable();
                while packet_bits.peek().is_some() {
                    version_total += parse_packet(&mut packet_bits, 1);
                }

            } else {
                // next 11 bits are total number of sub-packets
                let sub_packet_count = i32::from_str_radix(&take_n(packet, 11), 2).unwrap();
                version_total += parse_packet(packet, sub_packet_count);
                
            }
        }
        // println!("{}", version);
        version_total += version;
        packets -= 1;

    }
    version_total
}

fn hex_to_binary(hex: &str) -> String {
    let mut result: String = String::new();
    for c in hex.chars() {
        let n = i32::from_str_radix(&c.to_string(), 16).unwrap();
        result.push_str(&format!("{:04b}", n));
    }

    result
}

fn take_n(chars: &mut Peekable<Chars>, mut n: usize) -> String{

    let mut result = String::new();
    while n > 0 {
        result.push(chars.next().unwrap());
        n -= 1
    }

    result

}



fn part2(packet: &mut Peekable<Chars>) -> i64 {
    let version = i64::from_str_radix(&take_n(packet, 3), 2).unwrap();
    let type_id = i64::from_str_radix(&take_n(packet, 3), 2).unwrap();

    // println!("{} {}", version, type_id);
    if type_id == 4 { // literal packet
        let mut result: String = String::new();
        let mut first_byte = 1;
        while first_byte == 1{
                let next_chunk = take_n(packet, 5);
                result.push_str(&next_chunk[1..]);
                first_byte = i32::from_str_radix(&next_chunk[0..1], 2).unwrap();
        }

        i64::from_str_radix(&result, 2).unwrap()
    } else { // operator packet
        let mut values:Vec<i64> = Vec::new();
        if take_n(packet, 1) == "0" {
            // next 15 bits are total length in bits of sub-packet
            let packet_bit_count = usize::from_str_radix(&take_n(packet, 15), 2).unwrap();
            let next_chars = take_n(packet, packet_bit_count);
            let mut packet_bits = next_chars.chars().peekable();
            while packet_bits.peek().is_some() {
                values.push(part2(&mut packet_bits));
            }

        } else {
            // next 11 bits are total number of sub-packets
            let mut sub_packet_count = i32::from_str_radix(&take_n(packet, 11), 2).unwrap();
            while sub_packet_count > 0 {
                values.push(part2(packet));
                sub_packet_count -=1
            }
            
        }
        // println!("{:?}", values);
        match type_id {
            0 => {
                values.iter().sum::<i64>()
            }
            1 => {
                values.iter().product::<i64>()
            }
            2 => {*values.iter().min().unwrap()}
            3 => {*values.iter().max().unwrap()}
            5 => if values[0] > values[1] {1} else {0}
            6 => if values[0] < values[1] {1} else {0}
            7 => if values[0] == values[1] {1} else {0}
            _ => unreachable!()
        }
        
    }


    
}

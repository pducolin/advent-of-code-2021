use std::cmp::{min, max};
use std::fs::File;
use std::io::{self, BufRead};

fn hex_to_binary_string(hex: char) -> String {
  match hex {
    '0' => "0000".to_string(),
    '1' => "0001".to_string(),
    '2' => "0010".to_string(),
    '3' => "0011".to_string(),
    '4' => "0100".to_string(),
    '5' => "0101".to_string(),
    '6' => "0110".to_string(),
    '7' => "0111".to_string(),
    '8' => "1000".to_string(),
    '9' => "1001".to_string(),
    'A' => "1010".to_string(),
    'B' => "1011".to_string(),
    'C' => "1100".to_string(),
    'D' => "1101".to_string(),
    'E' => "1110".to_string(),
    'F' => "1111".to_string(),
    _ => panic!("Unexpected char {}", hex)
  }
}

#[derive(Clone)]
struct Packet {
  version: u8,
  type_id: u8,
  value: Option<u64>,
  sub_packets: Option<Vec<Packet>>,
}

fn parse_data(data: &String) -> Vec<Packet> {
  let mut binary_string = "".to_owned();
  for c in data.chars() {
    binary_string.push_str(&hex_to_binary_string(c));
  }
  return parse_buffer(&binary_string, None).0;
}

fn parse_buffer(source_binary_string: &String, tot_packets: Option<usize>) -> (Vec<Packet>, usize) {
  let mut instructions: Vec<Packet> = Vec::new();
  let mut binary_string = source_binary_string.clone();

  loop {
    // min packet len is 11
    if binary_string.len() < 11 {
      break
    } 
    // parse instruction
    // VVVTTT...
    // VERSION VVV
    let version = parse_version(&mut binary_string);
    // TYPE TTT
    let instr_type = parse_type(&mut binary_string);
    match instr_type {
      4 => {
        // Payload
        let value = parse_literal_instruction(&mut binary_string);
        instructions.push(Packet{
          version,
          type_id: instr_type,
          value: Some(value),
          sub_packets: None,
        });
      },
      _ => {
        let sub_instructions = parse_operator_instruction(&mut binary_string);
        instructions.push(Packet{
          version,
          type_id: instr_type,
          value: None,
          sub_packets: Some(sub_instructions),
        });
      },
    }

    if tot_packets.is_some() && instructions.len() == tot_packets.unwrap() {
      // stop consuming
      break;
    }
  }
  return (instructions, source_binary_string.len() - binary_string.len());
}

fn parse_literal_instruction(binary_string: &mut String) -> u64 {
  let mut payload: String = String::new();
  loop {
    // parse literal packets
    // a literal packet are 5 bits long
    // last literal packet starts with 0
    let packet = binary_string[..5].to_string();
    for _ in 0..5 {
      binary_string.remove(0);
    }
    payload.push_str(&packet[1..]);
    if packet.starts_with("0") {
      break;
    }
  }
  return u64::from_str_radix(&payload, 2).unwrap();
}

fn parse_operator_instruction(binary_string: &mut String) -> Vec<Packet>  {
  let mut sub_packets: Vec<Packet> = Vec::new();
  // length type
  // I
  let length_type = binary_string.remove(0);
  match length_type {
      '0' => {
        let length_str = binary_string[..15].to_string();
        for _ in 0..15 {
          binary_string.remove(0);
        }
        let length = usize::from_str_radix(&length_str, 2).unwrap();
        let sub_packet_payload = binary_string[..length].to_string();
        for _ in 0..length {
          binary_string.remove(0);
        }
        sub_packets = parse_buffer(&sub_packet_payload, None).0;
      },
      '1' => {
        let num_packets_str = binary_string[..11].to_string();
        for _ in 0..11 {
          binary_string.remove(0);
        }
        let num_packets = usize::from_str_radix(&num_packets_str, 2).unwrap();
        let tuple = parse_buffer(binary_string, Some(num_packets));
        sub_packets = tuple.0;
        let parsed_len = tuple.1;
        for _ in 0..parsed_len {
          binary_string.remove(0);
        }
      },
      _ => panic!("Unexpected length type {}", length_type),
  }
  return sub_packets;
}


fn parse_version(binary_string: &mut String) -> u8 {
    let version_bin: String = binary_string[..3].to_string();
    let version: u8 = u8::from_str_radix(&version_bin, 2).unwrap();
    for _ in 0..3 {
      binary_string.remove(0);
    }
    return version;
}

fn parse_type(binary_string: &mut String) -> u8 {
  let type_bin: String = binary_string[..3].to_string();
  let instr_type: u8 = u8::from_str_radix(&type_bin, 2).unwrap();
  for _ in 0..3 {
    binary_string.remove(0);
  }
  return instr_type;
}

fn get_versions(packets: &Vec<Packet>) -> Vec<u8> {
  let mut versions: Vec<u8> = Vec::new();
  for p in packets {
    versions.push(p.version);
    if p.sub_packets.is_some() {
      let sub_packets = p.sub_packets.as_ref().unwrap();
      versions.extend(get_versions(sub_packets));
    }
  }
  return versions;
}

fn solution_1(data: &Vec<String>) -> u64 {
  let packets = parse_data(data.first().unwrap());
  println!("first level has {} packets", packets.len());
  let versions = get_versions(&packets);
  let mut counter: u64 = 0;
  for v in versions{
    counter += v as u64;
  }
  return counter;
}

fn evaluate_packet(packet: &Packet) -> u64 {
  match packet.type_id {
      0 => {
        // sum
        let mut sum: u64 = 0;
        for sub_packet in packet.sub_packets.as_ref().unwrap() {
           sum += evaluate_packet(sub_packet);
        }
        return sum;
      },
      1 => {
        // product
        let mut prod: u64 = 1;
        for sub_packet in packet.sub_packets.as_ref().unwrap() {
           prod *= evaluate_packet(sub_packet);
        }
        return prod;
      },
      2 => {
        // min
        let mut packets_min: u64 = u64::MAX;
        for sub_packet in packet.sub_packets.as_ref().unwrap() {
          packets_min = min(packets_min, evaluate_packet(sub_packet));
        }
        return packets_min;
      },
      3 => {
        // max
        let mut packets_max: u64 = u64::MIN;
        for sub_packet in packet.sub_packets.as_ref().unwrap() {
          packets_max = max(packets_max, evaluate_packet(sub_packet));
        }
        return packets_max;
      },
      4 => {
        // literal
        return packet.value.unwrap();
      }
      5 => {
        // greater than
        let mut sub_packets = packet.sub_packets.as_ref().unwrap().clone();
        
        let b = evaluate_packet(&sub_packets.pop().unwrap());
        let a = evaluate_packet(&sub_packets.pop().unwrap());
        if a > b {
          return 1;
        }
        return 0;
      },
      6 => {
        // less than
        let mut sub_packets = packet.sub_packets.as_ref().unwrap().clone();
        let b = evaluate_packet(&sub_packets.pop().unwrap());
        let a = evaluate_packet(&sub_packets.pop().unwrap());
        if a < b {
          return 1;
        }
        return 0;
      },
      7 => {
        // equal to
        let mut sub_packets = packet.sub_packets.as_ref().unwrap().clone();
        let b = evaluate_packet(&sub_packets.pop().unwrap());
        let a = evaluate_packet(&sub_packets.pop().unwrap());
        if a == b {
          return 1;
        }
        return 0;
      },
      _ => panic!("Unexpected type id {}", packet.type_id),
  }
  0
}

fn solution_2(data: &Vec<String>) -> u64 {
  let packets = parse_data(data.first().unwrap());
  return evaluate_packet(&packets.first().unwrap());
}

fn main() {
  let test = false;
  let mut file_path: String = "inputs/day16".to_string();
  let mut emoji: String = "🎉".to_string();
  if test {
    file_path += ".test";
    emoji = "🧪".to_string();
  }
  file_path += ".txt";

  let file = File::open(file_path).unwrap();
  let data: Vec<String> = io::BufReader::new(file)
              .lines()
              .map(|line|line.unwrap())
              .collect();

  println!(r#"{:?} Part 1 result is {:?}"#, emoji, solution_1(&data));

  println!(r#"{:?} Part 2 test result is {:?}"#, emoji, solution_2(&data));
}

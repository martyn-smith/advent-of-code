#[derive(Debug)]
enum PacketType {
    Literal,
    Operator,
}

#[derive(Debug)]
enum PacketMode {
    Length(usize),
    Subs(usize),
}

struct Cursor {
    version: usize,
    val: usize,
    len: usize,
}

fn parse_byte(input: &[bool]) -> usize {
    let mut val = 0;
    for i in input.iter() {
        val <<= 1;
        val += if *i { 1 } else { 0 };
    }
    val
}

fn parse_packet(input: &[bool]) -> Cursor {
    let mut version = parse_byte(&input[..3]);
    let p_type = match input[3..6] {
        [true, false, false] => PacketType::Literal,
        _ => PacketType::Operator,
    };
    match p_type {
        PacketType::Literal => {
            let mut val = 0usize;
            let mut len = 6; //starting length
            for i in input[6..].chunks(5) {
                len += 5;
                val <<= 4;
                val += parse_byte(&i[1..]);
                if !i[0] {
                    break;
                }
            }
            Cursor {
                version,
                val,
                len,
            }
        }
        PacketType::Operator => {
            let mode = match input[6] {
                false => PacketMode::Length(15),
                true => PacketMode::Subs(11),
            };
            match mode {
                PacketMode::Length(l) => {
                    let mut len = parse_byte(&input[6..6 + l]);
                    let c = parse_packet(&input[6 + l..6 + l + len]);
                    version += c.version;
                    let val = c.val;
                    len += c.len;
                    Cursor {
                        version,
                        val,
                        len,
                    }
                }
                PacketMode::Subs(n) => {
                    let num = parse_byte(&input[6..6 + n]);
                    let mut len = 6 + n;
                    for i in 0..num {
                        let c = parse_packet(&input[len..]);
                        version += c.version;
                        len += c.len;
                    }
                    Cursor {
                        version,
                        val: 0,
                        len,
                    }
                }
            }
            //part_1(input[6..])
        }
    }
}

pub fn get_input() -> Vec<bool> {
    let input = include_str!("../../data/16.txt").trim();
    let bytes = input
        .chars()
        .map(|c| c.to_digit(16).unwrap() as u8)
        .collect::<Vec<u8>>();
    bytes
        .iter()
        .map(|byte| {
            vec![
                byte >> 3 != 0,
                (byte >> 2) & 1 != 0,
                (byte >> 1) & 1 != 0,
                byte & 1 != 0,
            ]
        })
        .flatten()
        .collect::<Vec<bool>>()
}

pub fn part_1(input: &[bool]) -> usize {
    parse_packet(input).version
}

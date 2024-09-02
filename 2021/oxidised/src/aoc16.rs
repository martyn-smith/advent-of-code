fn parse<'a>(mut b: impl Iterator<Item = &'a bool>, n: u8) -> u16 {
    (0..n)
        .rev()
        .fold(0, |acc, i| acc | (*b.next().unwrap() as u16) << i)
}

#[derive(Debug)]
enum PacketType {
    Literal(usize),
    Operator(Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
    v: u8,
    id: u8,
    p: PacketType,
}

impl Packet {
    fn from(bits: &[bool]) -> (Self, u16) {
        //dbg!(&quads);
        let mut b = bits.iter();
        let v = parse(&mut b, 3) as u8;
        let id = parse(&mut b, 3) as u8;
        let mut size = 6u16;
        let p = match id {
            4 => {
                let mut value = 0usize;
                loop {
                    let last = !b.next().unwrap();
                    let v = parse(&mut b, 4) as u8;
                    value <<= 4;
                    value += v as usize;
                    size += 5;
                    if last {
                        break;
                    }
                }
                PacketType::Literal(value)
            }
            _ => match b.next().unwrap() {
                false => {
                    //length of subpackets
                    let mut length = parse(&mut b, 15);
                    size += 16;
                    let mut packets: Vec<Packet> = vec![];
                    while length > 0 {
                        let (packet, s) = Packet::from(&bits[size as usize..]);
                        packets.push(packet);
                        size += s;
                        length -= s;
                    }
                    PacketType::Operator(packets)
                }
                true => {
                    //number of subpackets
                    let mut num = parse(&mut b, 11);
                    size += 12;
                    let mut packets: Vec<Packet> = vec![];
                    while num > 0 {
                        let (packet, s) = Packet::from(&bits[size as usize..]);
                        packets.push(packet);
                        size += s;
                        num -= 1;
                    }
                    PacketType::Operator(packets)
                }
            },
        };
        (Self { v, id, p }, size)
    }

    fn score(&self) -> usize {
        match &self.p {
            PacketType::Literal(_) => self.v as usize,
            PacketType::Operator(pt) => self.v as usize + pt.iter().map(|pkt| pkt.score() as usize).sum::<usize>()
        }
    }
}

pub fn get_input() -> Vec<bool> {
    //let raw = "D2FE28";
    //let raw = "EE00D40C823060";
    let raw = include_str!("../../data/16.txt").trim();
    raw.chars()
        .map(|c| u8::from_str_radix(&c.to_string(), 16).expect("invalid digit"))
        .map(|quad| {
            vec![
                quad >> 3 & 1 != 0,
                quad >> 2 & 1 != 0,
                quad >> 1 & 1 != 0,
                quad & 1 != 0,
            ]
        })
        .flatten()
        .collect::<Vec<_>>()
}

pub fn part_1(input: &[bool]) -> usize {
    let (packet, _) = Packet::from(input);
    packet.score()
}

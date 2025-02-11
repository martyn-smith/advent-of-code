///
/// Advent of Code day 16: get_u16 the input
///
use anyhow::{anyhow, Context, Result};
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

#[macro_export]
macro_rules! bin {
    ($pkt:expr) => {{
        use std::fmt::Write;
        $pkt.iter()
            .fold(String::with_capacity($pkt.len()), |mut out, b| {
                let _ = write!(&mut out, "{}", if *b { 1 } else { 0 });
                out
            })
    }};
}
#[derive(Debug)]
enum PacketType {
    Literal(usize),
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Minimum(Vec<Packet>),
    Maximum(Vec<Packet>),
    GT(Box<[Packet; 2]>),
    LT(Box<[Packet; 2]>),
    EQ(Box<[Packet; 2]>),
}

#[derive(Debug)]
struct Packet {
    v: u8,
    id: u8,
    p: PacketType,
    size: u16,
}

//impl TryFrom<&mut Iter<'_, bool>> for Packet {
impl TryFrom<&[bool]> for Packet {
    type Error = anyhow::Error;
    fn try_from(bits: &[bool]) -> Result<Self> {
        //println!("{}", bin!(bits));
        let v = Self::get_u16(&bits[..3])? as u8;
        let id = Self::get_u16(&bits[3..6])? as u8;
        let mut size = 6u16;
        let p = match id {
            0 => {
                let (packets, s) = Self::get_packets(&bits[6..])?;
                size += s;
                PacketType::Sum(packets)
            }
            1 => {
                let (packets, s) = Self::get_packets(&bits[6..])?;
                size += s;
                PacketType::Product(packets)
            }
            2 => {
                let (packets, s) = Self::get_packets(&bits[6..])?;
                size += s;
                PacketType::Minimum(packets)
            }
            3 => {
                let (packets, s) = Self::get_packets(&bits[6..])?;
                size += s;
                PacketType::Maximum(packets)
            }
            4 => {
                let (value, s) = Self::get_value(&bits[6..])?;
                size += s;
                PacketType::Literal(value)
            }
            5 => {
                let (pairs, s) = Self::get_pair(&bits[6..])?;
                size += s;
                PacketType::GT(Box::new(pairs))
            }
            6 => {
                let (pairs, s) = Self::get_pair(&bits[6..])?;
                size += s;
                PacketType::LT(Box::new(pairs))
            }
            7 => {
                let (pairs, s) = Self::get_pair(&bits[6..])?;
                size += s;
                PacketType::EQ(Box::new(pairs))
            }
            _ => return Err(anyhow!("packet type must be between 0 and 7")),
        };
        //dbg!(&p);
        Ok(Self { v, id, p, size })
    }
}

impl Display for Packet {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        todo!()
    }
}

impl Packet {
    //fn get_u16<'a>(mut b: impl Iterator<Item = &'a bool>, n: u8) -> u16 {
    fn get_u16(bits: &[bool]) -> Result<u16> {
        let n = bits.len();
        let mut b = bits.iter();
        Ok((0..n).rev().fold(0, |acc, i| {
            acc | (*b.next().expect("run out of bits in get_u16") as u16) << i
        }))
    }

    //fn get_value<'a>(mut b: impl Iterator<Item = &'a bool>, size: &mut u16) -> usize {
    fn get_value(bits: &[bool]) -> Result<(usize, u16)> {
        let mut value = 0usize;
        let mut i = 0;
        loop {
            let last = !bits[i];
            i += 1;
            let v = Self::get_u16(&bits[i..i + 4])? as u8;
            value <<= 4;
            value += v as usize;
            i += 4;
            if last {
                break;
            }
        }
        Ok((value, i as u16))
    }

    fn get_pair(bits: &[bool]) -> Result<([Packet; 2], u16)> {
        let mut i = 0;
        match bits[i] {
            false => {
                //length of subpackets
                let length = Self::get_u16(&bits[1..16])?;
                i = 16;
                let p0 = Packet::try_from(&bits[i..])?;
                i += p0.size as usize;
                let p1 = Packet::try_from(&bits[i..])?;
                i += p1.size as usize;
                Ok(([p0, p1], i as u16))
            }
            true => {
                //number of subpackets
                //let mut _num = Self::get_u16(&bits[1..12])?;
                i = 12;
                let p0 = Packet::try_from(&bits[i..])?;
                i += p0.size as usize;
                let p1 = Packet::try_from(&bits[i..])?;
                i += p1.size as usize;
                Ok(([p0, p1], i as u16))
            }
        }
    }

    fn get_packets(bits: &[bool]) -> Result<(Vec<Packet>, u16)> {
        let mut packets: Vec<Packet> = vec![];
        let mut i;
        match bits[0] {
            false => {
                //length of subpackets
                let mut length = Self::get_u16(&bits[1..16])?;
                i = 16;
                while length > 0 {
                    let packet = Packet::try_from(&bits[i..])?;
                    i += packet.size as usize;
                    length -= packet.size;
                    packets.push(packet);
                }
            }
            true => {
                //number of subpackets
                let mut num = Self::get_u16(&bits[1..12])?;
                i = 12;
                while num > 0 {
                    //let packet = Packet::try_from(&bits[*size as usize..])?;
                    let packet = Packet::try_from(&bits[i..])?;
                    i += packet.size as usize;
                    packets.push(packet);
                    num -= 1;
                }
            }
        }
        //dbg!(&packets);
        Ok((packets, i as u16))
    }

    fn score(&self) -> usize {
        match &self.p {
            PacketType::Literal(_) => self.v as usize,
            PacketType::Sum(pt)
            | PacketType::Product(pt)
            | PacketType::Minimum(pt)
            | PacketType::Maximum(pt) => {
                self.v as usize + pt.iter().map(|pkt| pkt.score()).sum::<usize>()
            }
            PacketType::LT(pt) | PacketType::GT(pt) | PacketType::EQ(pt) => {
                self.v as usize + pt[0].score() + pt[1].score()
            }
        }
    }

    fn eval(&self) -> usize {
        match &self.p {
            PacketType::Literal(v) => *v,
            PacketType::Sum(pt) => pt.iter().map(|pkt| pkt.eval()).sum::<usize>(),
            PacketType::Product(pt) => pt.iter().map(|pkt| pkt.eval()).product::<usize>(),
            PacketType::Minimum(pt) => pt.iter().map(|pkt| pkt.eval()).min().unwrap(),
            PacketType::Maximum(pt) => pt.iter().map(|pkt| pkt.eval()).max().unwrap(),
            PacketType::GT(pt) => {
                if pt[0].eval() > pt[1].eval() {
                    1
                } else {
                    0
                }
            }
            PacketType::LT(pt) => {
                if pt[0].eval() < pt[1].eval() {
                    1
                } else {
                    0
                }
            }
            PacketType::EQ(pt) => {
                if pt[0].eval() == pt[1].eval() {
                    1
                } else {
                    0
                }
            }
        }
    }
}

pub fn get_input() -> Vec<bool> {
    //let raw = "D2FE28";
    //let raw = "EE00D40C823060";
    //let raw = "38006F45291200";
    let raw = include_str!("../../data/16.txt").trim();
    raw.chars()
        .map(|c| u8::from_str_radix(&c.to_string(), 16).expect("invalid digit"))
        .flat_map(|quad| {
            vec![
                quad >> 3 & 1 != 0,
                quad >> 2 & 1 != 0,
                quad >> 1 & 1 != 0,
                quad & 1 != 0,
            ]
        })
        .collect::<Vec<_>>()
}

pub fn part_1(input: &[bool]) -> usize {
    let packet = Packet::try_from(input).unwrap();
    packet.score()
}

pub fn part_2(input: &[bool]) -> usize {
    let packet = Packet::try_from(input).unwrap();
    packet.eval()
}

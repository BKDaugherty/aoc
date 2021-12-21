use anyhow::{anyhow, Result};
use env_logger::Env;
use std::io;
use std::iter::{FromIterator, IntoIterator};

#[derive(Clone, Debug, Eq, PartialEq)]
enum Packet {
    Literal(PacketLiteral),
    Operator(PacketOperator),
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PacketLiteral {
    packet_version: usize,
    value: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Operator {
    Sum,
    Product,
    Min,
    Max,
    Gt,
    Lt,
    Equal,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PacketOperator {
    packet_version: usize,
    packet_operator_type: Operator,
    subpackets: Vec<Packet>,
}

fn convert_to_numeric_value(values: Vec<char>) -> Result<usize> {
    Ok(usize::from_str_radix(&String::from_iter(values.iter()), 2)?)
}

fn parse_packet_literal(
    packet_version: usize,
    vals: &mut impl Iterator<Item = char>,
) -> Result<PacketLiteral> {
    let mut values = Vec::new();
    loop {
        let starting_bit = usize::from_str_radix(&String::from_iter(vals.take(1)), 2)?;
        let mut numerical_bits = vals.take(4).collect::<Vec<char>>();
        values.append(&mut numerical_bits);
        match starting_bit {
            0 => {
                break;
            }
            1 => {}
            _ => {
                return Err(anyhow!("Invalid binary"));
            }
        }
    }
    let padding_len = 4 - (values.len() + (values.len() / 4) + 6) % 4;
    vals.skip(padding_len);
    let value = convert_to_numeric_value(values)?;
    Ok(PacketLiteral {
        value,
        packet_version,
    })
}

fn parse_packet_operator(
    packet_version: usize,
    opcode: usize,
    vals: &mut impl Iterator<Item = char>,
) -> Result<PacketOperator> {
    let packet_length = usize::from_str_radix(&String::from_iter(vals.take(1)), 2)?;
    let mut packets = Vec::new();

    let operator = match opcode {
        0 => Operator::Sum,
        1 => Operator::Product,
        2 => Operator::Min,
        3 => Operator::Max,
        5 => Operator::Gt,
        6 => Operator::Lt,
        7 => Operator::Equal,
        _ => return Err(anyhow!("unknown opcode")),
    };

    match packet_length {
        0 => {
            let packet_bit_length = usize::from_str_radix(&String::from_iter(vals.take(15)), 2)?;
            let sub_packet_buffer = vals.take(packet_bit_length).collect::<Vec<char>>();
            let mut sub_packet_iter = sub_packet_buffer.into_iter().peekable();

            while sub_packet_iter.peek() != None {
                packets.push(parse_packet(sub_packet_iter.by_ref())?);
            }
        }
        1 => {
            // Parse the next 11 packets
            let num_packets = usize::from_str_radix(&String::from_iter(vals.take(11)), 2)?;
            for _ in 0..num_packets {
                packets.push(parse_packet(vals)?);
            }
        }
        _ => {
            return Err(anyhow!("Invalid binary"));
        }
    };

    Ok(PacketOperator {
        subpackets: packets,
        packet_operator_type: operator,
        packet_version,
    })
}

fn parse_packet(vals: &mut impl Iterator<Item = char>) -> Result<Packet> {
    let version = String::from_iter(vals.take(3));
    let packet_version = usize::from_str_radix(&version, 2)?;
    let packet_type = usize::from_str_radix(&String::from_iter(vals.take(3)), 2)?;
    let packet = match packet_type {
        4 => Packet::Literal(parse_packet_literal(packet_version, vals)?),
        opcode => Packet::Operator(parse_packet_operator(packet_version, opcode, vals)?),
    };
    Ok(packet)
}

fn from_literal(value: String) -> Result<Packet> {
    parse_packet(value.chars().by_ref())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn init() {
        let _ = env_logger::builder()
            .is_test(true)
            .filter(None, LevelFilter::Info)
            .try_init();
    }

    #[test]
    fn parse_literal() -> Result<()> {
        init();

        let literal = "110100101111111000101000".to_string();
        let packet = from_literal(literal)?;

        assert_eq!(
            packet,
            Packet::Literal(PacketLiteral {
                value: 2021,
                packet_version: 6
            })
        );
        Ok(())
    }

    #[test]
    fn parse_operator() -> Result<()> {
        init();
        info!("OP test");
        let operator = "00111000000000000110111101000101001010010001001000000000".to_string();
        let packet = from_literal(operator)?;
        assert_eq!(
            packet,
            Packet::Operator(PacketOperator {
                packet_version: 1,
                packet_operator_type: Operator::Gt,
                subpackets: vec!(
                    Packet::Literal(PacketLiteral {
                        packet_version: 6,
                        value: 10,
                    },),
                    Packet::Literal(PacketLiteral {
                        packet_version: 2,
                        value: 20
                    })
                )
            })
        );
        Ok(())
    }
}

fn add_versions(packet: &Packet) -> usize {
    match packet {
        Packet::Literal(literal) => literal.packet_version,
        Packet::Operator(op) => {
            let mut total = op.packet_version;
            for subpacket in &op.subpackets {
                total += add_versions(subpacket);
            }
            total
        }
    }
}

fn evaluate_packet(packet: &Packet) -> usize {
    match packet {
        Packet::Literal(literal) => literal.value,
        Packet::Operator(operator) => {
            let sub_exprs: Vec<usize> = operator
                .subpackets
                .iter()
                .map(|p| evaluate_packet(p))
                .collect();
            match operator.packet_operator_type {
                Operator::Sum => sub_exprs.iter().sum(),
                Operator::Product => sub_exprs.iter().product(),
                Operator::Min => *sub_exprs.iter().min().unwrap(),
                Operator::Max => *sub_exprs.iter().max().unwrap(),
                Operator::Gt => (sub_exprs[0] > sub_exprs[1]) as usize,
                Operator::Lt => (sub_exprs[0] < sub_exprs[1]) as usize,
                Operator::Equal => (sub_exprs[0] == sub_exprs[1]) as usize,
            }
        }
    }
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let mut binary_string = String::new();

    for c in buffer.chars() {
        let result = match c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => panic!("Not hex"),
        };
        binary_string.push_str(result);
    }

    let packet = from_literal(binary_string)?;
    println!("Versions: {}", add_versions(&packet));

    let result = evaluate_packet(&packet);
    println!("Result: {}", result);

    Ok(())
}

use binascii::hex2bin;
use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
struct Node {
    version: u8,
    typeid: u8,
    literal: Option<u128>,
    nodes: Option<Vec<Node>>,
}

#[cfg(test)]
mod test {
    use bitreader::BitReader;
    use test_case::test_case;

    use super::*;

    #[test]
    fn literal() {
        let mut out = [0u8; 1024];
        let res = hex2bin("D2FE28".as_bytes(), &mut out);

        let x = res.unwrap();

        let mut reader = BitReader::new(x);

        let nodes = parse(&mut reader, 1);

        assert_eq!(1, nodes.len());
        let node = nodes.first().unwrap();
        assert_eq!(6, node.version);
        assert_eq!(2021, node.literal.unwrap());
    }

    #[test]
    fn operator_lenght() {
        let mut out = [0u8; 1024];
        let res = hex2bin("38006F45291200".as_bytes(), &mut out);

        let x = res.unwrap();

        let mut reader = BitReader::new(x);

        let nodes = parse(&mut reader, 1);

        assert_eq!(1, nodes.len());
        let node = nodes.first().unwrap();
        assert_eq!(1, node.version);
        let mut literals = node.nodes.as_ref().unwrap().iter();
        let literal1 = literals.next().unwrap();
        assert_eq!(10, literal1.literal.unwrap());
        let literal2 = literals.next().unwrap();
        assert_eq!(20, literal2.literal.unwrap());
    }

    #[test]
    fn operator_count() {
        let mut out = [0u8; 1024];
        let res = hex2bin("EE00D40C823060".as_bytes(), &mut out);

        let x = res.unwrap();

        let mut reader = BitReader::new(x);

        let nodes = parse(&mut reader, 1);

        assert_eq!(1, nodes.len());
        let node = nodes.first().unwrap();
        assert_eq!(7, node.version);
        let mut literals = node.nodes.as_ref().unwrap().iter();
        let literal1 = literals.next().unwrap();
        assert_eq!(1, literal1.literal.unwrap());
        let literal2 = literals.next().unwrap();
        assert_eq!(2, literal2.literal.unwrap());
        let literal2 = literals.next().unwrap();
        assert_eq!(3, literal2.literal.unwrap());
    }

    #[test]
    fn bin() {
        let mut out = [0u8; 1024];
        let res = hex2bin("D2FE28".as_bytes(), &mut out);
        dbg!(&res);

        let x = res.unwrap();
        for b in x.iter() {
            print!("{}", format!("{:08b}", b));
        }
        println!();

        let mut reader = BitReader::new(x);

        let foo = parse(&mut reader, 1);

        dbg!(foo);

        let three_bits = reader.read_u64(3).unwrap();

        print!("{}", format!("{:08b}", three_bits));
    }

    fn parse(reader: &mut BitReader, limit: i32) -> Vec<Node> {
        let mut nodes = vec![];

        let start = reader.position() as i32;
        let mut i = 0;
        while i < limit || (reader.position() as i32) < (start - limit) {
            i += 1;
            let version = reader.read_u8(3).unwrap();
            let typeid = reader.read_u8(3).unwrap();

            nodes.push(match typeid {
                4 => parse_literal(reader, version, typeid),
                _ => parse_operator(reader, version, typeid),
            });
        }

        nodes
    }

    fn parse_operator(reader: &mut BitReader, version: u8, typeid: u8) -> Node {
        let mode = reader.read_u8(1).unwrap();
        match mode {
            0 => parse_operator_size(reader, version, typeid),
            _ => parse_operator_count(reader, version, typeid),
        }
    }

    fn parse_operator_size(reader: &mut BitReader, version: u8, typeid: u8) -> Node {
        let length = reader.read_u16(15).unwrap() as i32;
        Node {
            version,
            typeid,
            literal: None,
            nodes: Some(parse(reader, -length)),
        }
    }

    fn parse_operator_count(reader: &mut BitReader, version: u8, typeid: u8) -> Node {
        let count = reader.read_u16(11).unwrap() as i32;
        Node {
            version,
            typeid,
            literal: None,
            nodes: Some(parse(reader, count)),
        }
    }

    fn parse_literal(reader: &mut BitReader, version: u8, typeid: u8) -> Node {
        let mut literal: u128 = 0;
        loop {
            let group = reader.read_u8(5).unwrap();
            let val = group & 0b00001111;
            literal = literal << 4;
            literal += val as u128;
            if (group & 0b00010000) == 0 {
                break;
            }
        }
        Node {
            version,
            typeid,
            literal: Some(literal),
            nodes: None,
        }
    }

    #[test_case("sample1.txt" => is eq(16); "sample1")]
    #[test_case("sample2.txt" => is eq(12); "sample2")]
    #[test_case("sample3.txt" => is eq(23); "sample3")]
    #[test_case("sample4.txt" => is eq(31); "sample4")]
    #[test_case("input.txt" => is eq(936); "input")]
    fn part1(input: &str) -> u32 {
        let string = read_to_string(input).unwrap();
        let mut out = [0u8; 1024];
        let res = hex2bin(string.trim().as_bytes(), &mut out);
        let mut reader = BitReader::new(res.unwrap());

        let nodes = parse(&mut reader, 1);

        sum_versions(nodes)
    }

    fn sum_versions(nodes: Vec<Node>) -> u32 {
        let mut sum = 0;

        for n in nodes {
            sum += n.version as u32;
            if n.nodes.is_some() {
                sum += sum_versions(n.nodes.unwrap());
            }
        }

        sum
    }
}

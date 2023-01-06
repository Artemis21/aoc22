use crate::Day;

struct Parser {
    input: &'static [u8],
    offset: usize,
}

impl Parser {
    const fn new(input: &'static str) -> Self {
        Self {
            input: input.as_bytes(),
            offset: 0,
        }
    }

    fn packet(&mut self) -> Packet {
        if self.take('[') {
            let mut packets = vec![];
            while !self.take(']') {
                packets.push(self.packet());
                self.take(',');
            }
            Packet::Packet(packets)
        } else {
            Packet::Number(self.number())
        }
    }

    fn number(&mut self) -> usize {
        let mut num = 0;
        while self.input[self.offset].is_ascii_digit() {
            num = num * 10 + (self.input[self.offset] - b'0') as usize;
            self.offset += 1;
        }
        num
    }

    fn take(&mut self, s: char) -> bool {
        if self.input[self.offset] == s as u8 {
            self.offset += 1;
            true
        } else {
            false
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum Packet {
    Packet(Vec<Packet>),
    Number(usize),
}

#[derive(Clone)]
pub struct Day13(Vec<(Packet, Packet)>);

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Packet(p1), Self::Packet(p2)) => p1.cmp(p2),
            (Self::Packet(pac), Self::Number(num)) => pac.cmp(&vec![Self::Number(*num)]),
            (Self::Number(num), Self::Packet(pac)) => vec![Self::Number(*num)].cmp(pac),
            (Self::Number(n1), Self::Number(n2)) => n1.cmp(n2),
        }
    }
}

impl Day for Day13 {
    fn parse(input: &'static str) -> Self {
        let mut pairs = vec![];
        for chunk in input
            .lines()
            .filter(|l| !l.is_empty())
            .collect::<Vec<_>>()
            .chunks(2)
        {
            let packet1 = Parser::new(chunk[0]).packet();
            let packet2 = Parser::new(chunk[1]).packet();
            pairs.push((packet1, packet2));
        }
        Self(pairs)
    }

    fn part1(&self) -> String {
        self.0
            .iter()
            .enumerate()
            .map(|(i, (a, b))| if a < b { i + 1 } else { 0 })
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self) -> String {
        let pack2 = Packet::Packet(vec![Packet::Packet(vec![Packet::Number(2)])]);
        let pack6 = Packet::Packet(vec![Packet::Packet(vec![Packet::Number(6)])]);
        let mut lt2 = 1;
        let mut lt6 = 2;
        for (a, b) in &self.0 {
            if a < &pack2 {
                lt2 += 1;
            }
            if a < &pack6 {
                lt6 += 1;
            }
            if b < &pack2 {
                lt2 += 1;
            }
            if b < &pack6 {
                lt6 += 1;
            }
        }
        (lt2 * lt6).to_string()
    }
}

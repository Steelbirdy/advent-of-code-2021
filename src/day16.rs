pub type Input = Vec<u8>;

const BITS: [[u8; 4]; 16] = [
    [0, 0, 0, 0],
    [0, 0, 0, 1],
    [0, 0, 1, 0],
    [0, 0, 1, 1],
    [0, 1, 0, 0],
    [0, 1, 0, 1],
    [0, 1, 1, 0],
    [0, 1, 1, 1],
    [1, 0, 0, 0],
    [1, 0, 0, 1],
    [1, 0, 1, 0],
    [1, 0, 1, 1],
    [1, 1, 0, 0],
    [1, 1, 0, 1],
    [1, 1, 1, 0],
    [1, 1, 1, 1],
];

#[aoc_generator(day16)]
pub fn generator(input: &str) -> Input {
    input
        .bytes()
        .flat_map(|b| BITS[(if b <= b'9' { b - b'0' } else { b - b'A' + 10 }) as usize])
        .collect()
}

fn decode_bits(bits: &[u8]) -> u64 {
    let mut ret = 0;
    for &b in bits {
        ret = (ret << 1) + b as u64;
    }
    ret
}

fn parse_packet(bits: &[u8]) -> (usize, Packet) {
    let version = decode_bits(&bits[..3]) as u8;
    let type_id = decode_bits(&bits[3..6]) as u8;

    let mut idx = 6;

    let packet = if type_id == 4 {
        let mut value = 0_u64;
        loop {
            let leading = bits[idx];
            value = (value << 4) + decode_bits(&bits[idx + 1..idx + 5]);
            idx += 5;

            if leading == 0 {
                break;
            }
        }
        Packet::literal(version, value)
    } else if bits[6] == 0 {
        let num_bits = decode_bits(&bits[7..22]);
        let mut sub_packets = Vec::with_capacity(2);
        idx = 22;
        while idx < 22 + num_bits as usize {
            let (len, sp) = parse_packet(&bits[idx..]);
            idx += len;
            sub_packets.push(sp);
        }
        Packet::operator(version, type_id, sub_packets)
    } else {
        let sub_count = decode_bits(&bits[7..18]);
        let mut sub_packets = Vec::with_capacity(sub_count as usize);
        idx = 18;
        for _ in 0..sub_count {
            let (len, sp) = parse_packet(&bits[idx..]);
            idx += len;
            sub_packets.push(sp);
        }
        Packet::operator(version, type_id, sub_packets)
    };

    (idx, packet)
}

enum Packet {
    Literal {
        version: u8,
        value: u64,
    },
    Operator {
        version: u8,
        type_id: u8,
        sub_packets: Vec<Packet>,
    },
}

impl Packet {
    #[inline]
    fn literal(version: u8, value: u64) -> Self {
        Self::Literal { version, value }
    }

    #[inline]
    fn operator(version: u8, type_id: u8, sub_packets: Vec<Packet>) -> Self {
        Self::Operator {
            version,
            type_id,
            sub_packets,
        }
    }

    fn version_sum(&self) -> u16 {
        match self {
            Self::Literal { version, .. } => *version as u16,
            Self::Operator {
                version,
                sub_packets,
                ..
            } => *version as u16 + sub_packets.iter().map(|p| p.version_sum()).sum::<u16>(),
        }
    }

    fn value(&self) -> u64 {
        match self {
            Self::Literal { value, .. } => *value,
            Self::Operator {
                type_id,
                sub_packets,
                ..
            } => match type_id {
                0 => sub_packets.iter().map(|p| p.value()).sum(),
                1 => sub_packets.iter().map(|p| p.value()).product(),
                2 => sub_packets.iter().map(|p| p.value()).min().unwrap(),
                3 => sub_packets.iter().map(|p| p.value()).max().unwrap(),
                5 => (sub_packets[0].value() > sub_packets[1].value()) as u64,
                6 => (sub_packets[0].value() < sub_packets[1].value()) as u64,
                7 => (sub_packets[0].value() == sub_packets[1].value()) as u64,
                _ => unreachable!(),
            },
        }
    }
}

#[aoc(day16, part1)]
pub fn part1(input: &Input) -> u16 {
    let (_, packet) = parse_packet(input);
    packet.version_sum()
}

#[aoc(day16, part2)]
pub fn part2(input: &Input) -> u64 {
    let (_, packet) = parse_packet(input);
    packet.value()
}

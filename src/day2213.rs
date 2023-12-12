use std::cmp::Ordering;

use itertools::izip;
use nom::multi::separated_list1;

use crate::util::*;

aoc_test!(part1, 221301, 13);
aoc_test!(part1, 221300, 6415);

struct Signal(Vec<PacketPair>);

#[derive(From, Debug)]
struct PacketPair(Packet, Packet);

#[derive(Debug)]
enum Packet {
    Number(u32),
    List(Vec<Packet>),
}

impl FromStr for Signal {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use aoc_nom::*;

        fn packet(s: &str) -> IResult<&str, Packet> {
            alt((
                u32.map(Packet::Number),
                delimited(tag("["), separated_list0(tag(","), packet), tag("]")).map(Packet::List),
            ))
            .parse(s)
        }

        let packet_pair = separated_pair(packet, multispace1, packet).map(PacketPair::from);
        let signal = separated_list1(multispace1, packet_pair).map(Signal);
        signal.anyhow(s)
    }
}

fn part1(signal: Signal) -> usize {
    signal.score()
}

impl Signal {
    fn score(&self) -> usize {
        self.0
            .iter()
            .zip(1..)
            .flat_map(|(pp, i)| pp.is_sorted().then_some(i))
            .sum()
    }
}

impl PacketPair {
    fn is_sorted(&self) -> bool {
        self.0.order(&self.1) != Ordering::Greater
    }
}

impl Packet {
    fn order(&self, other: &Self) -> Ordering {
        fn cmp_n(pp: &[Packet], qq: &[Packet]) -> Ordering {
            use itertools::EitherOrBoth::*;
            use Ordering::*;
            for pq in pp.iter().zip_longest(qq.iter()) {
                match pq {
                    Both(p, q) => match cmp_1(p, q) {
                        Less => return Less,
                        Equal => {}
                        Greater => return Greater,
                    },
                    Left(_) => return Greater,
                    Right(_) => return Less,
                }
            }
            Equal
        }
        fn cmp_1(p: &Packet, q: &Packet) -> Ordering {
            use Packet::*;
            match (p, q) {
                (Number(p), Number(q)) => p.cmp(q),
                (Number(p), List(qq)) => cmp_n(&[Number(*p)], qq),
                (List(pp), Number(q)) => cmp_n(pp, &[Number(*q)]),
                (List(pp), List(qq)) => cmp_n(pp, qq),
            }
        }
        cmp_1(self, other)
    }
}

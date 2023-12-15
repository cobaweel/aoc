use std::borrow::Borrow;

use crate::util::*;
use test_case::test_case;

struct Instructions(Vec<Instruction>);

#[derive(Debug)]
struct Instruction(Op, String);

#[derive(Debug, Clone)]
enum Op {
    Val(Value),
    And(Value, Value),
    Or(Value, Value),
    Left(Value, u8),
    Right(Value, u8),
    Not(Value),
}

#[derive(Debug, Clone)]
enum Value {
    Constant(u16),
    Wire(String),
}

#[test_case("d", 72)]
#[test_case("e", 507)]
#[test_case("f", 492)]
#[test_case("g", 114)]
#[test_case("h", 65412)]
#[test_case("i", 65079)]
#[test_case("x", 123)]
#[test_case("y", 456)]
fn test(wire: &str, word: u16) {
    let mut machine = Machine::from_instructions(parse_test_file(150701));
    assert_eq!(machine.run(wire), word);
}

aoc_test!(part1, 150700, 956);
aoc_test!(part2, 150700, 40149);

fn part1(instructions: Instructions) -> u16 {
    Machine::from_instructions(instructions).run("a")
}

fn part2(instructions: Instructions) -> u16 {
    let machine = &mut Machine::from_instructions(instructions);
    machine.hardwire("b", 956);
    machine.run("a")
}

struct Machine {
    program: HashMap<String, Op>,
    cache: HashMap<String, u16>,
}

impl Machine {
    fn from_instructions(instructions: Instructions) -> Self {
        let cache = HashMap::new();
        let program = instructions
            .0
            .into_iter()
            .map(|Instruction(op, wire)| (wire, op))
            .collect();
        Machine { program, cache }
    }

    fn hardwire(&mut self, wire: &str, word: u16) {
        self.program
            .insert(wire.to_string(), Op::Val(Value::Constant(word)));
    }

    fn run(&mut self, wire: &str) -> u16 {
        if self.cache.contains_key(wire) {
            *self.cache.get(wire).unwrap()
        } else {
            let word = self.exe(wire);
            self.cache.insert(wire.to_string(), word);
            word
        }
    }

    fn exe(&mut self, wire: &str) -> u16 {
        use Op::*;
        let op = self.program.remove(wire).unwrap();
        match op {
            Val(x) => self.val(&x),
            And(x, y) => self.val(&x) & self.val(&y),
            Or(x, y) => self.val(&x) | self.val(&y),
            Left(x, i) => self.val(&x) << i,
            Right(x, i) => self.val(&x) >> i,
            Not(x) => !self.val(&x),
        }
    }

    fn val(&mut self, value: &Value) -> u16 {
        use Value::*;
        match value {
            Constant(word) => *word,
            Wire(wire) => self.run(wire),
        }
    }
}

impl FromStr for Instructions {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Instructions, Self::Err> {
        use aoc_nom::*;
        use Op::*;
        use Value::*;
        let tok = |s| delimited(space0, tag(s), space0);
        let wire = || alpha1.map(|s: &str| s.to_string());
        let val = || fail.or(wire().map(Wire)).or(u16.map(Constant));
        let op = {
            fail.or(separated_pair(val(), tok("AND"), val()).map(|(p, q)| And(p, q)))
                .or(separated_pair(val(), tok("OR"), val()).map(|(p, q)| Or(p, q)))
                .or(separated_pair(val(), tok("RSHIFT"), u8).map(|(p, q)| Right(p, q)))
                .or(separated_pair(val(), tok("LSHIFT"), u8).map(|(p, q)| Left(p, q)))
                .or(preceded(tok("NOT"), val()).map(Not))
                .or(val().map(Val))
        };
        let instruction = separated_pair(op, tok("->"), wire()).map(|(p, q)| Instruction(p, q));
        let instructions = separated_list1(multispace1, instruction).map(Instructions);
        let instructions = terminated(instructions, tuple((multispace0, eof)));
        instructions.anyhow(s)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Constant(x) => write!(f, "{x}"),
            Value::Wire(x) => write!(f, "{x}"),
        }
    }
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Val(x) => write!(f, "{x}"),
            Op::Not(x) => write!(f, "NOT {x}"),
            Op::And(x, y) => write!(f, "{x} AND {y}"),
            Op::Or(x, y) => write!(f, "{x} AND {y}"),
            Op::Left(x, y) => write!(f, "{x} AND {y}"),
            Op::Right(x, y) => write!(f, "{x} AND {y}"),
        }
    }
}

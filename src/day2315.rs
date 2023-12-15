use crate::util::*;

aoc_test!(part1, 231501, 1320);
aoc_test!(part1, 231500, 494980);
aoc_test!(part2, 231501, 145);
aoc_test!(part2, 231500, 247933);

fn part1(sequence: String) -> usize {
    sequence.trim().split(',').map(hash).sum()
}

#[test]
fn test() {
    assert_eq!(hash("HASH"), 52);
}

fn hash(s: &str) -> usize {
    s.as_bytes()
        .iter()
        .fold(0, |h: u8, &c| h.wrapping_add(c).wrapping_mul(17)) as usize
}

#[derive(Debug)]
enum Instruction {
    Remove(String),
    Insert(String, usize),
}

struct Instructions(Vec<Instruction>);

struct State {
    cases: Vec<Vec<(String, usize)>>,
}

impl State {
    fn new() -> State {
        State {
            cases: vec![vec![]; 256],
        }
    }

    fn frob(&mut self, s: String, mut n: Option<usize>) {
        let case = &mut self.cases[hash(&s)];
        let mut evict = None;
        for (i,(s_, n_)) in case.iter_mut().enumerate() {
            if &s == s_ {
                if let Some(n) = n.take() {
                    *n_ = n;
                } else {
                    let _ = evict.insert(i);
                }
            }
        }
        if let Some(i) = evict {
            case.remove(i);
        }
        if let Some(n) = n {
            case.push((s, n));
        }
    }

    fn remove(&mut self, s: String) {
        self.frob(s, None);
    }

    fn insert(&mut self, s: String, i: usize) {
        self.frob(s, Some(i));
    }

    fn power(&self) -> usize {
        let mut power = 0;
        for (case_i, case) in self.cases.iter().enumerate() {
            for (slot_i, (_, focal_length)) in case.iter().enumerate() {
                power += (case_i + 1) * (slot_i + 1) * focal_length;
            }
        }
        power
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Remove(s) => self.remove(s),
            Instruction::Insert(s, i) => self.insert(s, i),
        }
    }
}

fn part2(instructions: Instructions) -> usize {
    let mut state = State::new();
    for instruction in instructions.0 {
        state.execute(instruction);
    }
    state.power()
}

impl FromStr for Instructions {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use aoc_nom::*;

        let variable =
            terminated(alpha1, tag("-")).map(|s: &str| Instruction::Remove(s.to_string()));
        let insert = separated_pair(alpha1, tag("="), u32)
            .map(|(s, i): (&str, u32)| Instruction::Insert(s.to_string(), i as usize));
        let instruction = alt((variable, insert));
        let instructions = separated_list1(tag(","), instruction).map(Instructions);
        let instructions = terminated(instructions, eof);

        instructions.anyhow(s)
    }
}

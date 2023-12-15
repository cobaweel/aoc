use crate::util::*;

aoc_test!(part1, 231501, 1320);
aoc_test!(part1, 231500, 494980);
aoc_test!(part2, 231501, 145);
aoc_test!(part2, 231500, 247933);

fn part1(sequence: String) -> usize {
    sequence.trim().split(',').map(hash).sum()
}

fn hash(s: &str) -> usize {
    s.as_bytes()
        .iter()
        .fold(0, |h: u8, &c| h.wrapping_add(c).wrapping_mul(17)) as usize
}

fn part2(instructions: Instructions) -> usize {
    let mut state = State::new();
    state.run_all(instructions);
    state.power()
}

struct Instructions(Vec<Instruction>);

struct Instruction(Key, Operation);

enum Operation {
    Remove,
    Insert(Val),
}

type Key = String;
type Val = usize;

impl FromStr for Instructions {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use aoc_nom::*;
        let remove = tag("-").into_str_parser().map(|_| Operation::Remove);
        let insert = preceded(tag("="), u32).map(|i| Operation::Insert(i as Val));
        let instruction = tuple((alpha1, alt((remove, insert))));
        let instruction = instruction.map(|(s, op)| Instruction(s.to_string(), op));
        let instructions = separated_list1(tag(","), instruction).map(Instructions);
        let instructions = terminated(instructions, eof);
        instructions.anyhow(s)
    }
}

struct State {
    bins: Vec<Vec<(String, usize)>>,
}

impl State {
    fn new() -> State {
        State {
            bins: vec![vec![]; 256],
        }
    }

    fn run(&mut self, Instruction(key, operation): Instruction) {
        let bin = &mut self.bins[hash(&key)];
        let mut evict = None;
        for (i, (bin_key, bin_val)) in bin.iter_mut().enumerate() {
            if &key == bin_key {
                match operation {
                    Operation::Remove => {
                        let _ = evict.insert(i);
                        break;
                    }
                    Operation::Insert(val) => {
                        *bin_val = val;
                        return;
                    }
                }
            }
        }
        if let Some(i) = evict {
            bin.remove(i);
        }
        if let Operation::Insert(val) = operation {
            bin.push((key, val));
        }
    }

    fn run_all(&mut self, instructions: Instructions) {
        for instruction in instructions.0 {
            self.run(instruction);
        }
    }

    fn power(&self) -> usize {
        let mut power = 0;
        for (case_i, case) in self.bins.iter().enumerate() {
            for (slot_i, (_, focal_length)) in case.iter().enumerate() {
                power += (case_i + 1) * (slot_i + 1) * focal_length;
            }
        }
        power
    }
}

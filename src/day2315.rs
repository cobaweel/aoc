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
    let mut bins = Bins::new();
    bins.run_all(instructions);
    bins.power()
}

#[derive(Deref, DerefMut)]
struct Instructions(Vec<Instruction>);

struct Instruction(Key, Operation);

enum Operation {
    Remove,
    Insert(Val),
}

#[derive(Clone, PartialEq)]
struct Key(String);

#[derive(Clone)]
struct Val(usize);

#[derive(Deref, DerefMut)]
struct Bins(Vec<Vec<(Key, Val)>>);

impl Key {
    fn hash(&self) -> usize {
        hash(self.0.as_str())
    }
}

impl Bins {
    fn new() -> Bins {
        Bins(vec![vec![]; 256])
    }

    fn run(&mut self, Instruction(key, operation): Instruction) {
        let bin = &mut self[key.hash()];

        // Rust, very sensibly, doesn't like us modifying a collection while
        // we're iterating over it, so instead we make a note of which entry in
        // a bin to remove (if any) once the loop is over.
        let mut remove_after_loop: Option<usize> = None;

        for (i, (bin_key, bin_val)) in bin.iter_mut().enumerate() {
            if &key == bin_key {
                match operation {
                    Operation::Remove => {
                        let _ = remove_after_loop.insert(i);
                        break;
                    }
                    Operation::Insert(val) => {
                        *bin_val = val;
                        return;
                    }
                }
            }
        }
        if let Some(i) = remove_after_loop {
            bin.remove(i);
        }
        if let Operation::Insert(val) = operation {
            bin.push((key, val));
        }
    }

    fn run_all(&mut self, instructions: Instructions) {
        for instruction in instructions.0.into_iter() {
            self.run(instruction);
        }
    }

    fn power(self) -> usize {
        let mut power = 0;
        for (case_i, case) in self.0.into_iter().enumerate() {
            for (slot_i, (_, focal_length)) in case.into_iter().enumerate() {
                power += (case_i + 1) * (slot_i + 1) * focal_length.0;
            }
        }
        power
    }
}

impl FromStr for Instructions {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use aoc_nom::*;
        let remove = tag("-").into_str_parser().map(|_| Operation::Remove);
        let insert = preceded(tag("="), u32).map(|i| Operation::Insert(Val(i as usize)));
        let instruction = tuple((alpha1, alt((remove, insert))));
        let instruction = instruction.map(|(s, op)| Instruction(Key(String::from(s)), op));
        let instructions = separated_list1(tag(","), instruction).map(Instructions);
        let instructions = terminated(instructions, eof);
        instructions.anyhow(s)
    }
}

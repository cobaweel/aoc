use crate::util::*;

// These tests are commented out because AdventCoin mining makes my
// computer into a space heater, and takes too long.

aoc_test!(part1, 150400, 346386);
aoc_test!(part2, 150400, 9958218);

fn part1(input: String) -> usize {
    // mine_advent_coint("00000", input.as_str())
    Mine::new("00000".to_string(), input).mine()
}

fn part2(input: String) -> usize {
    Mine::new("000000".to_string(), input).mine()
}

struct Mine {
    prefix: String,
    input: String,
}

impl Mine {
    fn new(prefix: String, input: String) -> Self {
        Self { prefix, input }
    }

    fn blast(&self, i: usize) -> bool {
        let hashed = format!("{}{}", self.input, i);
        let hash = format!("{:x}", md5::compute(hashed));
        hash.starts_with(self.prefix.as_str())
    }

    fn mine(&self) -> usize {
        use rayon::iter::IntoParallelIterator as _;
        use rayon::prelude::*;
        let block_size = 10000;
        (0..)
            .flat_map(|block_idx| {
                let block = block_idx * block_size..(block_idx + 1) * block_size;
                block.into_par_iter().find_first(|i| self.blast(*i))
            })
            .next()
            .unwrap()
    }
}

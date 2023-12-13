use crate::util::*;

aoc_test!(part1, 150400, 346386);
aoc_test!(part2, 150400, 9958218);

fn part1(input: String) -> usize {
    Mine::new(5, input.into_bytes()).mine()
}

fn part2(input: String) -> usize {
    Mine::new(6, input.into_bytes()).mine()
}

struct Mine {
    prefix: usize,
    context: md5::Context,
}

impl Mine {
    fn new(prefix: usize, input: Vec<u8>) -> Self {
        let mut context = md5::Context::new();
        context.consume(input.as_slice());
        Self { prefix, context }
    }

    fn blast(&self, i: usize) -> bool {
        let mut context = self.context.clone();
        context.consume(i.to_string().as_bytes());
        let md5::Digest(hash) = context.compute();
        for nybble in 0..self.prefix {
            match (nybble % 2, hash[nybble / 2] & 0xf0, hash[nybble / 2] & 0x0f) {
                (0, 0, _) | (1, _, 0) => {}
                _ => return false,
            }
        }
        true
    }

    fn mine(&self) -> usize {
        use rayon::iter::IntoParallelIterator as _;
        use rayon::prelude::*;
        let block_size = 1000000;
        (0..)
            .flat_map(|block_idx| {
                let block = block_idx * block_size..(block_idx + 1) * block_size;
                block.into_par_iter().find_first(|i| self.blast(*i))
            })
            .next()
            .unwrap()
    }
}

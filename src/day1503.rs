use crate::util::*;

aoc_test!(part1, 150300, 2572);
aoc_test!(part2, 150300, 2631);

fn part1(turns: String) -> usize {
    let mut book = Book::default();
    book.process_turns(turns.chars());
    book.count()
}

fn part2(turns: String) -> usize {
    let mut book = Book::default();
    let santa_turns = turns.chars().step_by(2);
    let robo_santa_turns = turns.chars().skip(1).step_by(2);
    book.process_turns(santa_turns);
    book.process_turns(robo_santa_turns);
    book.count()
}

#[derive(Default)]
struct Book {
    visited: HashSet<(isize, isize)>,
}

impl Book {
    fn process_turns(&mut self, turns: impl Iterator<Item = char>) {
        let (mut x, mut y) = (0, 0);
        self.visited.insert((x, y));
        for turn in turns {
            match turn {
                '>' => x += 1,
                '<' => x -= 1,
                'v' => y += 1,
                '^' => y -= 1,
                _ => {}
            }
            self.visited.insert((x, y));
        }
    }

    fn count(&mut self) -> usize {
        self.visited.len()
    }
}

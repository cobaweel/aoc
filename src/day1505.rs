use crate::util::*;

aoc_test!(part1, 150501, 2);
aoc_test!(part1, 150500, 236);
aoc_test!(part2, 150502, 2);
aoc_test!(part2, 150500, 51);

struct Text {
    lines: Vec<Line>,
}

struct Line(String);

impl FromStr for Text {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().map(Line::from).collect();
        Ok(Text { lines })
    }
}

impl From<&str> for Line {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

fn part1(text: Text) -> usize {
    text.lines.into_iter().filter(Line::nice_1).count()
}

fn part2(text: Text) -> usize {
    text.lines.into_iter().filter(Line::nice_2).count()
}

impl Line {
    fn nice_1(&self) -> bool {
        self.nice_1a() && self.nice_1b() && self.nice_1c()
    }

    fn nice_1a(&self) -> bool {
        let vowels: HashSet<char> = "aoeui".chars().collect();
        self.0.chars().filter(|c| vowels.contains(c)).count() >= 3
    }

    fn nice_1b(&self) -> bool {
        self.0.chars().tuple_windows().any(|(a, b)| a == b)
    }

    fn nice_1c(&self) -> bool {
        self.0
            .chars()
            .tuple_windows()
            .all(|x| !matches!(x, ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y')))
    }

    fn nice_2(&self) -> bool {
        self.nice_2a() && self.nice_2b()
    }

    fn nice_2a(&self) -> bool {
        let pairs = self.0.chars().tuple_windows();
        let positions_by_pair = pairs.zip(0..).into_grouping_map::<(char, char), usize>();
        let mut position_sets = positions_by_pair.collect::<BTreeSet<usize>>().into_values();
        position_sets.any(|ps| {
            let first = ps.first().unwrap_or(&0);
            let last = ps.last().unwrap_or(&0);
            first.abs_diff(*last) > 1
        })
    }

    fn nice_2b(&self) -> bool {
        self.0.chars().tuple_windows().any(|(a, _, b)| a == b)
    }
}

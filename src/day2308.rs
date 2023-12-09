use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

#[test]
fn test1() {
    crate::util::parse_and_test(part1, 230801, 2);
}

#[test]
fn test2() {
    crate::util::parse_and_test(part1, 230800, 16897);
}

#[test]
fn test3() {
    crate::util::parse_and_test(part2, 230802, 6);
}

#[test]
fn test4() {
    crate::util::parse_and_test(part2, 230800, 16563603485021);
}

struct Instructions {
    turns: Vec<Turn>,
    nodes: Vec<Node>,
}

#[derive(Debug)]
enum Turn {
    R,
    L,
}

struct Node {
    this: String,
    left: String,
    right: String,
}

impl FromStr for Instructions {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use crate::util::parse_with_nom::*;
        let left = tag("L").map(|_| Turn::L).id();
        let right = tag("R").map(|_| Turn::R).id();
        let turn = alt((left, right));
        let turns = many1(turn);
        let node = count(terminated(alphanumeric1, many1(one_of(" =(),"))), 3).id();
        let node = node.map(|ns| ns.into_iter().map(str::to_string));
        let node = map_opt(node, Itertools::collect_tuple);
        let node = node.map(|(this, left, right)| Node { this, left, right });
        let nodes = separated_list1(multispace1, node);
        let instructions = separated_pair(turns, multispace1, nodes);
        let instructions = instructions.map(|(turns, nodes)| Instructions { turns, nodes });
        instructions.anyhow(s)
    }
}

fn part1(instructions: Instructions) -> i64 {
    let (n, _) = path(&instructions, &"AAA")
        .find_position(|&n| n == "ZZZ")
        .expect("camel got lost");
    n as i64
}

// Iterate through the path starting at a certain node, producing nodes
fn path<'a>(instructions: &'a Instructions, mut name: &'a str) -> impl Iterator<Item = &'a str> {
    let nodes = instructions.nodes.iter();
    let tree: HashMap<&str, (&str, &str)> = nodes
        .map(|Node { this, left, right }| (this.as_str(), (left.as_str(), right.as_str())))
        .collect();
    instructions.turns.iter().cycle().map(move |turn| {
        let out = name;
        if let Some((left, right)) = tree.get(name) {
            name = match turn {
                Turn::R => right,
                Turn::L => left,
            };
        }
        out
    })
}

/// Higher level representation of a path, resulting from an analysis of its
/// periodicity. The path consists of a "lead" followed by infinite iterations of
/// a "loop". The first occurence of the loop occupies path indices `start..end`.
/// The `end_points` field contains the path indices smaller than `end` where the
/// ghost visits a node that ends in `Z`. This is all the information we need to
/// solve the problem.
#[derive(Debug)]
struct PathInfo {
    start: usize,
    end: usize,
    end_points: HashSet<usize>,
}


impl PathInfo {
    /// Compute the `PathInfo` for a given starting node
    fn compute<'a>(instructions: &'a Instructions, name: &'a str) -> Option<Self> {
        let path = path(instructions, name);
        let n_turns = instructions.turns.len();
        let mut seen: HashMap<&str, usize> = HashMap::new();
        let mut end_points: HashSet<usize> = HashSet::new();
        for (i, name) in path.enumerate() {
            if name.ends_with('Z') {
                end_points.insert(i);
            }
            if let Some(j) = seen.get(name) {
                if (i - j) % n_turns == 0 {
                    return Some(PathInfo {
                        start: *j,
                        end: i,
                        end_points,
                    });
                }
            } else {
                seen.insert(name, i);
            }
        }
        None
    }

    

    /// Figure out whether the `i`-th node is an end node
    fn is_end_point(&self, i: usize) -> bool {
        let i = if i < self.end {
            i
        } else {
            (i - self.start) % (self.end - self.start) + self.start
        };
        self.end_points.contains(&i)
    }

    /// Returns pairs (x,n) such that for i=x,x+n,x+2n,..., the i-th node in the
    /// path is an end node.
    fn sequences(&self) -> Vec<(usize, usize)> {
        (self.start..self.end)
            .filter(|i| self.end_points.contains(i))
            .map(|i| (i, self.end - self.start))
            .collect_vec()
    }
}

#[derive(Debug, Clone)]
struct Sequence {
    off: usize,
    period: usize,
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    (a / gcd(a, b)) * b
}

fn solve((off1, period1): (usize, usize), (off2, period2): (usize, usize)) -> (usize, usize) {
    let i = (0..)
        .find(|i| (off1 + i * period1) % period2 == off2 % period2)
        .unwrap();
    (off1 + i * period1, lcm(period1, period2))
}

fn part2(instructions: Instructions) -> i64 {
    let n_turns = instructions.turns.len();
    let nodes = instructions.nodes.iter();
    let path_infos = nodes
        .map(|node| node.this.as_str())
        .filter(|name| name.ends_with('A'))
        .flat_map(|name| PathInfo::compute(&instructions, name))
        .collect_vec();

    // Before using these fancy path infos to do fancy number theory, we first
    // just do a brute force iteration up to the point where we're sure the
    // fancy number theory actually applies.
    for i in 0..path_infos.iter().map(|pi| pi.end).max().unwrap_or(0) {
        if path_infos.iter().all(|path_info| path_info.is_end_point(i)) {
            return i as i64;
        }
    }

    // Now the real fanciness begins. We retrieve a bunch of arithmetic
    // progressions and intersect them.
    let map = path_infos
        .iter()
        .map(PathInfo::sequences)
        .multi_cartesian_product()
        .flat_map(|sequences| sequences.into_iter().reduce(solve))
        .map(|(x, _)| x)
        .min()
        .unwrap();
    map as i64
}

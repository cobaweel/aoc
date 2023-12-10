use crate::util::*;

#[test]
fn test1() {
    crate::util::parse_and_test(part1, 230701, 6440);
}

#[test]
fn test2() {
    crate::util::parse_and_test(part1, 230700, 250347426);
}

#[test]
fn test3() {
    crate::util::parse_and_test(part2, 230701, 5905);
}

#[test]
fn test4() {
    crate::util::parse_and_test(part2, 230700, 251224870);
}

fn part1(mut hands: Hands) -> i64 {
    hands.winnings(Hand::key1)
}

fn part2(mut hands: Hands) -> i64 {
    hands.winnings(Hand::key2)
}

#[derive(Debug)]
struct Hands {
    hands: Vec<Hand>,
}

impl Hands {
    fn new(hands: Vec<Hand>) -> Self {
        Self { hands }
    }

    fn winnings(&mut self, key: fn(&Hand) -> (HandType, Vec<i64>)) -> i64 {
        let hands = self.hands.iter();
        let hands = hands.map(|hand| (key(hand), hand.bid)).sorted();
        let hands = hands.zip(1..);
        hands.map(|((_, bid), rank)| bid * rank).sum()
    }
}

impl FromStr for Hands {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use crate::util::parse_with_nom::*;
        let card = map_opt(none_of(" "), Card::parse);
        let hand = separated_pair(many1(card), space1, i64).map(Hand::from);
        let hands = separated_list1(newline, hand).map(Hands::new);
        hands.anyhow(s)
    }
}

#[derive(Eq, Ord, PartialEq, PartialOrd, Debug, derive_more::From)]
struct Hand {
    cards: Vec<Card>,
    bid: i64,
}

#[derive(Eq, Ord, PartialEq, PartialOrd, Debug, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    // Key to sort hands by for part 1 of the problem.
    fn key1(&self) -> (HandType, Vec<i64>) {
        let hand_type = self.hand_type(&Card::CJ);
        let values = self.values(Card::value1);
        (hand_type, values)
    }

    // Key to sort hands by for part 2 of the problem.
    fn key2(&self) -> (HandType, Vec<i64>) {
        let hand_type = Card::all()
            .into_iter()
            .map(|joker| self.hand_type(&joker))
            .max()
            .unwrap_or(HandType::HighCard);
        let values = self.values(Card::value2);
        (hand_type, values)
    }

    fn values(&self, f: fn(&Card) -> i64) -> Vec<i64> {
        self.cards.iter().map(f).collect_vec()
    }

    fn hand_type(&self, joker: &Card) -> HandType {
        let cards = self
            .cards
            .iter()
            .map(|card| if card == &Card::CJ { joker } else { card })
            .sorted()
            .group_by(|c| *c)
            .into_iter()
            .map(|(_, g)| g.count())
            .sorted()
            .rev()
            .collect_vec();
        use HandType::*;
        match cards.as_slice() {
            [5] => FiveOfAKind,
            [4, 1] => FourOfAKind,
            [3, 2] => FullHouse,
            [3, 1, 1] => ThreeOfAKind,
            [2, 2, 1] => TwoPair,
            [2, 1, 1, 1] => OnePair,
            _ => HighCard,
        }
    }
}

#[derive(Eq, Ord, PartialEq, PartialOrd, Debug, Clone, Copy)]
enum Card {
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    CT,
    CJ,
    CQ,
    CK,
    CA,
}

impl Card {
    fn all() -> Vec<Self> {
        use Card::*;
        vec![C2, C3, C4, C5, C6, C7, C8, C9, CT, CJ, CQ, CK, CA]
    }

    fn parse(c: char) -> Option<Self> {
        use Card::*;
        match c {
            '2' => Some(C2),
            '3' => Some(C3),
            '4' => Some(C4),
            '5' => Some(C5),
            '6' => Some(C6),
            '7' => Some(C7),
            '8' => Some(C8),
            '9' => Some(C9),
            'T' => Some(CT),
            'J' => Some(CJ),
            'Q' => Some(CQ),
            'K' => Some(CK),
            'A' => Some(CA),
            _ => None,
        }
    }

    fn value1(&self) -> i64 {
        use Card::*;
        match self {
            C2 => 0,
            C3 => 1,
            C4 => 2,
            C5 => 3,
            C6 => 4,
            C7 => 5,
            C8 => 6,
            C9 => 7,
            CT => 8,
            CJ => 9,
            CQ => 10,
            CK => 11,
            CA => 12,
        }
    }

    fn value2(&self) -> i64 {
        use Card::*;
        match self {
            CJ => 0,
            C2 => 1,
            C3 => 2,
            C4 => 3,
            C5 => 4,
            C6 => 5,
            C7 => 6,
            C8 => 7,
            C9 => 8,
            CT => 9,
            CQ => 10,
            CK => 11,
            CA => 12,
        }
    }
}

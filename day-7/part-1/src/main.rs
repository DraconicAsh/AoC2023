use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    let f = File::open(&args[1]).unwrap();
    let input = BufReader::new(f);

    let mut hands: Vec<Hand> = Vec::new();
    for line in input.lines() {
        hands.push(Hand::from_line(&line.unwrap()));
    }

    hands.sort();
    let mut total = 0;
    for (idx, hand) in hands.iter().enumerate() {
        println!("{hand}");
        total += hand.bid * (idx as u32 + 1);
    }

    println!("Answer: {total}");
}

#[derive(Debug, Eq)]
struct Hand {
    cards: CardVals,
    strength: HandStrength,
    bid: u32,
}

impl Hand {
    fn from_line(line: &'_ str) -> Self {
        let mut line = line.split_whitespace();
        let cards = line.next().unwrap();
        let bid = line.next().unwrap().parse::<u32>().unwrap();
        Self::new(cards, bid)
    }

    fn new(cards: &'_ str, bid: u32) -> Self {
        let cards = Self::map_card_values(cards);
        let strength = HandStrength::calc_strength(cards);
        Self {
            cards,
            strength,
            bid,
        }
    }

    fn map_card_values(cards: &'_ str) -> CardVals {
        let mut res: CardVals = [0; 5];
        for (i, c) in cards.char_indices() {
            res[i] = match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                _ => c.to_digit(10).unwrap() as u8,
            }
        }
        res
    }
}

impl std::fmt::Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} : {:?} : {}", self.cards, self.strength, self.bid)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let comp = self.strength.cmp(&other.strength);
        if comp == std::cmp::Ordering::Equal {
            Some(self.cards.cmp(&other.cards))
        } else {
            Some(comp)
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandStrength {
    Junk,
    Pair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

impl HandStrength {
    fn calc_strength(cards: CardVals) -> Self {
        let mut tracker = DupTracker::new();
        for c in cards {
            tracker.add(c);
        }
        let tallies = tracker.tallies();
        match tallies[0] {
            2 => {
                if tallies[1] == 2 {
                    Self::TwoPair
                } else {
                    Self::Pair
                }
            }
            3 => {
                if tallies[1] == 2 {
                    Self::FullHouse
                } else {
                    Self::ThreeKind
                }
            }
            4 => Self::FourKind,
            5 => Self::FiveKind,
            _ => Self::Junk,
        }
    }
}

#[derive(Debug)]
struct DupTracker {
    tracker: Vec<(u8, u8)>,
}

impl DupTracker {
    fn new() -> Self {
        Self {
            tracker: Vec::new(),
        }
    }

    fn add(&mut self, val: u8) {
        for set in self.tracker.iter_mut() {
            if set.0 == val {
                set.1 += 1;
                return;
            }
        }
        self.tracker.push((val, 1));
    }

    fn tallies(&self) -> Vec<u8> {
        let mut res = Vec::new();
        for set in self.tracker.iter() {
            res.push(set.1);
        }
        res.sort_by_key(|v| std::cmp::Reverse(*v));
        res
    }
}

type CardVals = [u8; 5];

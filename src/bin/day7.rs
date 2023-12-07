use std::time::Instant;

use aoc_2023::{load, print_res};
use bstr::{BString, ByteSlice};
use color_eyre::eyre::{self, eyre};
use enum_map::{Enum, EnumMap};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Enum)]
pub enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Enum)]
pub enum JokerCard {
    Ace,
    King,
    Queen,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Hand {
    Five,
    Four,
    FullHouse,
    Three,
    TwoPair,
    OnePair,
    HighCard,
}

impl From<Card> for JokerCard {
    fn from(value: Card) -> Self {
        match value {
            Card::Ace => JokerCard::Ace,
            Card::King => JokerCard::King,
            Card::Queen => JokerCard::Queen,
            Card::Jack => JokerCard::Joker,
            Card::Ten => JokerCard::Ten,
            Card::Nine => JokerCard::Nine,
            Card::Eight => JokerCard::Eight,
            Card::Seven => JokerCard::Seven,
            Card::Six => JokerCard::Six,
            Card::Five => JokerCard::Five,
            Card::Four => JokerCard::Four,
            Card::Three => JokerCard::Three,
            Card::Two => JokerCard::Two,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Draw([Card; 5]);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JokerDraw([JokerCard; 5]);

impl From<Draw> for JokerDraw {
    fn from(value: Draw) -> Self {
        let [a, b, c, d, e] = value.0;
        Self([a.into(), b.into(), c.into(), d.into(), e.into()])
    }
}

type Parsed = Vec<(Draw, usize)>;

pub fn parsing(input: &BString) -> color_eyre::Result<Parsed> {
    input
        .lines()
        .map(|l| {
            let (cards, bid) = l
                .split_once_str(" ")
                .ok_or_else(|| eyre!("Malformed line: {}", l.as_bstr()))?;

            eyre::ensure!(cards.len() == 5, "Must be exactly five cards");

            let mut parsed_cards = [Card::Ace; 5];
            for (&card, parsed) in cards.iter().zip(parsed_cards.iter_mut()) {
                *parsed = match card {
                    b'A' => Card::Ace,
                    b'K' => Card::King,
                    b'Q' => Card::Queen,
                    b'J' => Card::Jack,
                    b'T' => Card::Ten,
                    b'9' => Card::Nine,
                    b'8' => Card::Eight,
                    b'7' => Card::Seven,
                    b'6' => Card::Six,
                    b'5' => Card::Five,
                    b'4' => Card::Four,
                    b'3' => Card::Three,
                    b'2' => Card::Two,
                    _ => eyre::bail!("Invalid card: {}", card as char),
                };
            }

            let bid = std::str::from_utf8(bid)?.parse()?;

            Ok((Draw(parsed_cards), bid))
        })
        .collect()
}

impl Draw {
    fn hand(&self) -> Hand {
        let mut map: EnumMap<Card, usize> = EnumMap::default();
        for card in self.0 {
            map[card] += 1;
        }

        let largest_similar = map.values().max().unwrap();
        match largest_similar {
            5 => Hand::Five,
            4 => Hand::Four,
            3 => {
                if map.values().any(|&v| v == 2) {
                    Hand::FullHouse
                } else {
                    Hand::Three
                }
            }
            2 => {
                if map.values().filter(|&&v| v == 2).count() == 2 {
                    Hand::TwoPair
                } else {
                    Hand::OnePair
                }
            }
            1 => Hand::HighCard,
            _ => unreachable!(),
        }
    }
}

impl PartialOrd for Draw {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Draw {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand().cmp(&other.hand()).then(self.0.cmp(&other.0))
    }
}

pub fn part1(input: Parsed) {
    let total: usize = input
        .iter()
        .sorted_by_key(|(a, _)| a)
        .rev()
        .enumerate()
        .map(|(i, (_, b))| (i + 1) * b)
        .sum();
    print_res!("Total winnings: {total}")
}

impl JokerDraw {
    fn hand(&self) -> Hand {
        let mut map: EnumMap<JokerCard, usize> = EnumMap::default();

        for card in self.0 {
            map[card] += 1;
        }

        let largest_similar = map
            .iter()
            .filter_map(|(c, v)| match c {
                JokerCard::Joker => None,
                _ => Some(v),
            })
            .max()
            .unwrap();
        let joker_count = map[JokerCard::Joker];

        match largest_similar {
            5 | 0 => Hand::Five,
            4 => match joker_count {
                1 => Hand::Five,
                0 => Hand::Four,
                _ => unreachable!(),
            },
            3 => match joker_count {
                2 => Hand::Five,
                1 => Hand::Four,
                0 => {
                    if map.values().any(|&v| v == 2) {
                        Hand::FullHouse
                    } else {
                        Hand::Three
                    }
                }
                _ => unreachable!(),
            },
            2 => match joker_count {
                3 => Hand::Five,
                2 => Hand::Four,
                1 => {
                    if map.values().filter(|&&v| v == 2).count() == 2 {
                        Hand::FullHouse
                    } else {
                        Hand::Three
                    }
                }
                0 => {
                    if map.values().filter(|&&v| v == 2).count() == 2 {
                        Hand::TwoPair
                    } else {
                        Hand::OnePair
                    }
                }
                _ => unreachable!(),
            },
            1 => match joker_count {
                4 => Hand::Five,
                3 => Hand::Four,
                2 => Hand::Three,
                1 => Hand::OnePair,
                0 => Hand::HighCard,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}

impl PartialOrd for JokerDraw {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for JokerDraw {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand().cmp(&other.hand()).then(self.0.cmp(&other.0))
    }
}

pub fn part2(input: Parsed) {
    let total: usize = input
        .iter()
        .map(|&(d, b)| (JokerDraw::from(d), b))
        .sorted_by_key(|&(a, _)| a)
        .rev()
        .enumerate()
        .map(|(i, (_, b))| (i + 1) * b)
        .sum();
    print_res!("Total winnings: {total}")
}

pub fn main() -> color_eyre::Result<()> {
    let context = load()?;

    let start = Instant::now();
    let parsed = parsing(&context.input)?;
    let elapsed = humantime::format_duration(start.elapsed());

    let start = Instant::now();
    if context.part == 1 {
        part1(parsed);
    } else {
        part2(parsed);
    }
    let elapsed_part = humantime::format_duration(start.elapsed());

    println!("  Parsing: {elapsed}");
    println!("  Solving: {elapsed_part}");

    Ok(())
}

use std::{collections::HashSet, time::Instant};

use aoc_2023::{load, parse_u64_bytes, print_res};
use bstr::{BString, ByteSlice};
use color_eyre::eyre::{ensure, eyre};

#[derive(Debug)]
pub struct Card {
    winning: HashSet<u64>,
    drawn: HashSet<u64>,
}

type Parsed = Vec<Card>;

pub fn parsing(input: &BString) -> color_eyre::Result<Parsed> {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let line = line.as_bstr();
            let malformed = || eyre!("Malformed card: {line}");
            let (prefix, numbers) = line.split_once_str(":").ok_or_else(malformed)?;
            ensure!(
                parse_u64_bytes(prefix.strip_prefix(b"Card").ok_or_else(malformed)?.trim())
                    as usize
                    == i + 1,
                "Bad card index"
            );

            let parse_set = |set: &[u8]| {
                set.trim()
                    .split(|&c| c == b' ')
                    .filter(|c| !c.is_empty())
                    .map(parse_u64_bytes)
                    .collect()
            };
            let (winning, drawn) = numbers.split_once_str("|").ok_or_else(malformed)?;

            Ok(Card {
                winning: parse_set(winning),
                drawn: drawn
                    .trim()
                    .split(|&c| c == b' ')
                    .map(parse_u64_bytes)
                    .collect(),
            })
        })
        .collect()
}

impl Card {
    fn points(&self) -> usize {
        let winning_drawn = self.winning.intersection(&self.drawn).count();
        
        if winning_drawn == 0 {
            0
        } else {
            1 << (winning_drawn - 1)
        }
    }
}

pub fn part1(input: Parsed) {
    let total_points: usize = input.iter().map(Card::points).sum();
    print_res!("Total points are: {total_points}");
}

pub fn part2(input: Parsed) {
    todo!("todo part2")
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

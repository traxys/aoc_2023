use std::time::Instant;

use aoc_2023::{load, print_res};
use bstr::{BString, ByteSlice};
use color_eyre::eyre::eyre;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum State {
    Damaged,
    Operational,
    Unknown,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SpringField(Vec<State>);

impl std::fmt::Display for SpringField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &s in &self.0 {
            write!(
                f,
                "{}",
                match s {
                    State::Damaged => '#',
                    State::Operational => '.',
                    State::Unknown => '?',
                }
            )?;
        }

        Ok(())
    }
}

type Parsed<'a> = Vec<(SpringField, Vec<usize>)>;

pub fn parsing(input: &BString) -> color_eyre::Result<Parsed> {
    input
        .lines()
        .map(|line| {
            let line = line.as_bstr();
            let (springs, ranges) = line
                .split_once_str(" ")
                .ok_or_else(|| eyre!("Invalid line: {line}"))?;

            let ranges = std::str::from_utf8(ranges)?
                .split(',')
                .map(|n| n.parse())
                .try_collect()?;

            let springs = springs
                .iter()
                .map(|&c| match c {
                    b'#' => Ok(State::Damaged),
                    b'.' => Ok(State::Operational),
                    b'?' => Ok(State::Unknown),
                    _ => Err(eyre!("Invalid spring state: {}", c as char)),
                })
                .try_collect()?;

            Ok((SpringField(springs), ranges))
        })
        .collect()
}

fn possible_arrangements(springs: SpringField, ranges: &[usize]) -> usize {
    let mut stack = vec![(0, None, ranges)];
    let springs = &springs.0;

    let mut count = 0;
    while let Some((idx, prev, ranges)) = stack.pop() {
        let len = ranges[0];

        if idx + len > springs.len() {
            continue;
        }

        let pattern_start = &springs[idx..];

        let could_be = (prev.is_none() || prev == Some(State::Operational))
            && pattern_start
                .iter()
                .take(len)
                .all(|&c| c != State::Operational)
            && (pattern_start.len() == len || pattern_start[len] != State::Damaged);

        if could_be {
            if ranges.len() == 1 {
                if springs[idx + len..].iter().all(|&s| s != State::Damaged) {
                    count += 1;
                }
            } else if idx + len < springs.len() {
                assert_ne!(springs[idx + len], State::Damaged,);

                stack.push((idx + len + 1, Some(State::Operational), &ranges[1..]));
            }
        }

        if springs[idx] != State::Damaged {
            stack.push((idx + 1, Some(State::Operational), ranges));
        }
    }

    count
}

pub fn part1(input: Parsed) {
    let number_of_arrangements: usize = input
        .into_iter()
        .map(|(s, r)| possible_arrangements(s, &r))
        .sum();

    print_res!("Total number of arragengements: {number_of_arrangements}");
}

#[allow(unstable_name_collisions)]
pub fn part2(input: Parsed) {
    let number_of_arrangements: usize = input
        .into_iter()
        .map(|(s, r)| {
            let repeated = SpringField(
                std::iter::repeat_with(|| s.0.clone())
                    .take(5)
                    .intersperse_with(|| vec![State::Unknown])
                    .flatten()
                    .collect(),
            );
            (repeated, r.repeat(5))
        })
        .map(|(s, r)| possible_arrangements(s, &r))
        .sum();

    print_res!("Total number of arragengements: {number_of_arrangements}");
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

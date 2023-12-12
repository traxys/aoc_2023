use std::{
    collections::{hash_map::DefaultHasher, HashSet},
    hash::{Hash, Hasher},
    time::Instant,
};

use aoc_2023::{load, print_res};
use bstr::{BString, ByteSlice};
use color_eyre::eyre::eyre;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum State {
    Damaged,
    Operational,
    Unknown,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
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
    let mut stack = vec![(0, None, springs, ranges)];

    let mut ways = Vec::new();

    let mut seen = HashSet::new();

    fn calculate_hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }

    while let Some((idx, prev, springs, ranges)) = stack.pop() {
        let spring_hash = calculate_hash(&springs);

        if idx + ranges[0] > springs.0.len() || seen.contains(&(idx, spring_hash, ranges)) {
            continue;
        }

        seen.insert((idx, spring_hash, ranges));

        let len = ranges[0];
        let pattern_start = &springs.0[idx..];

        let could_be = (prev.is_none() || prev == Some(State::Operational))
            && pattern_start
                .iter()
                .take(len)
                .all(|&c| c != State::Operational)
            && (pattern_start.len() == len || pattern_start[len] != State::Damaged);

        if could_be {
            let mut springs = springs.clone();

            springs.0[idx..idx + len]
                .iter_mut()
                .for_each(|c| *c = State::Damaged);

            if ranges.len() == 1 {
                if springs.0[idx + len..].iter().all(|&s| s != State::Damaged) {
                    ways.push(springs);
                }
            } else if idx + len < springs.0.len() {
                assert_ne!(
                    springs.0[idx + len],
                    State::Damaged,
                    "At {idx} in field {springs} for range {len}",
                );

                springs.0[idx + len] = State::Operational;

                stack.push((idx + len, Some(State::Operational), springs, &ranges[1..]));
            }
        }

        if springs.0[idx] != State::Damaged {
            stack.push((idx + 1, Some(State::Operational), springs, ranges));
        }
    }

    ways.len()
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
            (
                SpringField(
                    std::iter::repeat_with(|| s.0.clone())
                        .intersperse_with(|| vec![State::Unknown])
                        .flatten()
                        .collect(),
                ),
                r.repeat(5),
            )
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

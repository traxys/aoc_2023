use std::{collections::HashMap, time::Instant};

use aoc_2023::{load, print_res};
use bstr::{BString, ByteSlice};
use color_eyre::eyre::eyre;
//use indicatif::ParallelProgressIterator;
use indicatif::ProgressIterator;
use itertools::Itertools;
//use rayon::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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

type Cache<'a> = HashMap<(&'a [State], &'a [usize]), usize>;

fn possible_arrangements_segment_cached<'a>(
    remain: &'a [State],
    ranges: &'a [usize],
    cache: &mut Cache<'a>,
) -> usize {
    match cache.get(&(remain, ranges)) {
        None => {
            let v = possible_arrangements_segment(remain, ranges, cache);
            cache.insert((remain, ranges), v);
            v
        }
        Some(&v) => v,
    }
}

fn possible_arrangements_segment<'a>(
    remain: &'a [State],
    ranges: &'a [usize],
    cache: &mut Cache<'a>,
) -> usize {
    let mut count = 0;
    let len = ranges[0];

    if len > remain.len() {
        return 0;
    }

    let could_be = remain.iter().take(len).all(|&c| c != State::Operational)
        && (remain.len() == len || remain[len] != State::Damaged);

    if could_be {
        if ranges.len() == 1 {
            if remain[len..].iter().all(|&s| s != State::Damaged) {
                count += 1;
            }
        } else if len < remain.len() {
            assert_ne!(remain[len], State::Damaged);

            count += possible_arrangements_segment_cached(&remain[len + 1..], &ranges[1..], cache);
        }
    }

    if remain[0] != State::Damaged {
        count += possible_arrangements_segment_cached(&remain[1..], ranges, cache);
    }

    count
}

fn possible_arrangements<'a>(
    springs: &'a [State],
    ranges: &'a [usize],
    cache: &mut Cache<'a>,
) -> usize {
    //println!("\n\nStarting: {springs} ranges: {ranges:?}");

    let runs = springs
        .split(|&s| s == State::Operational)
        .filter(|r| !r.is_empty())
        .collect_vec();

    let mut remaining_ranges = vec![(im::Vector::new(), ranges, im::Vector::new())];

    for &run in runs.iter() {
        let could_fit = (run.len() + 1) / 2;

        //println!(" run {} (could fit {could_fit})", SpringField(run.to_vec()));

        let mut new_remaining_ranges = Vec::new();

        for (possible, ranges, used) in &remaining_ranges {
            //println!("  with ranges {ranges:?}");

            if run.iter().all(|&s| s == State::Unknown) {
                new_remaining_ranges.push((possible.clone(), *ranges, used.clone()));
            }

            for r in 1..=could_fit {
                if r > ranges.len() {
                    continue;
                }

                let (try_range, next_range) = ranges.split_at(r);

                let ways = possible_arrangements_segment(run, try_range, cache);

                /*println!(
                    "   for (used:{used:?}) {try_range:?} (remain: {next_range:?}) --> {ways}"
                );*/

                if ways > 0 {
                    let mut new_possible = possible.clone();
                    new_possible.push_back(ways);
                    let mut used = used.clone();
                    used.push_back(try_range);

                    new_remaining_ranges.push((new_possible, next_range, used));
                }
            }
        }

        // for (ways, range, used) in &new_remaining_ranges {
        //     println!("  end: (used: {used:?}) {range:?} (ways: {ways:?})");
        // }

        remaining_ranges = new_remaining_ranges;
    }

    remaining_ranges.retain(|(_, r, _)| r.is_empty());

    // for (ways, _, used) in &remaining_ranges {
    //     println!(" final: {used:?} (ways: {ways:?})");
    // }

    remaining_ranges
        .iter()
        .map(|(possible, _, _)| possible.iter().product::<usize>())
        .sum()
}

pub fn part1(input: Parsed) {
    let mut cache = Cache::new();

    let number_of_arrangements: usize = input
        .iter()
        .map(|(s, r)| possible_arrangements(&s.0, r, &mut cache))
        .sum();

    print_res!("Total number of arragengements: {number_of_arrangements}");
}

#[allow(unstable_name_collisions)]
pub fn part2(input: Parsed) {
    let mut cache = Cache::new();

    let inputs = input
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
        .collect_vec();

    let line_count = inputs.len() as u64;
    let number_of_arrangements: usize = inputs
        .iter()
        //.into_par_iter()
        .map(|(s, r)| possible_arrangements(&s.0, r, &mut cache))
        .progress_count(line_count)
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

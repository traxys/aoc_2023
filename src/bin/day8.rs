use std::{collections::HashMap, time::Instant};

use aoc_2023::{load, print_res};
use bstr::{BStr, BString, ByteSlice};
use color_eyre::eyre::eyre;
use itertools::Itertools;

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
}

type Parsed<'a> = (Vec<Direction>, HashMap<&'a BStr, (&'a BStr, &'a BStr)>);

pub fn parsing(input: &BString) -> color_eyre::Result<Parsed> {
    let (directions, map) = input
        .split_once_str("\n\n")
        .ok_or_else(|| eyre!("Missing separator in input"))?;

    let directions = directions
        .iter()
        .map(|&b| match b {
            b'L' => Ok(Direction::Left),
            b'R' => Ok(Direction::Right),
            _ => Err(eyre!("Invalid direction: {}", b as char)),
        })
        .try_collect()?;

    let map = map
        .lines()
        .map(|line| -> color_eyre::Result<_> {
            let (from, to) = line
                .split_once_str(" = ")
                .ok_or_else(|| eyre!("Malformed line: {}", line.as_bstr()))?;

            let (left, right) = to
                .trim_with(|c| c == '(' || c == ')')
                .split_once_str(", ")
                .ok_or_else(|| eyre!("Malformed dest: {}", to.as_bstr()))?;

            Ok((from.as_bstr(), (left.as_bstr(), right.as_bstr())))
        })
        .try_collect()?;

    Ok((directions, map))
}

fn loop_len(directions: &[Direction], map: &HashMap<&BStr, (&BStr, &BStr)>, start: &BStr) -> usize {
    let mut current = start;
    let mut count = 0;

    while current.last() != Some(&b'Z') {
        let (left, right) = map[&current];
        current = match directions[count % directions.len()] {
            Direction::Left => left,
            Direction::Right => right,
        };
        count += 1;
    }

    count
}

pub fn part1((directions, map): Parsed) {
    let count = loop_len(&directions, &map, b"AAA".as_bstr());

    print_res!("Steps to go to ZZZ: {count}");
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        (a, b) = (b, a % b);
    }

    a
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

pub fn part2((directions, map): Parsed) {
    let period = map
        .keys()
        .filter(|l| l.ends_with(b"A"))
        .map(|l| loop_len(&directions, &map, l))
        .fold(1, lcm);

    print_res!("Steps to loop: {period}");
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

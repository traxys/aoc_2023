use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

use aoc_2023::{load, print_res};
use bstr::{BStr, BString, ByteSlice};
use regex::bytes::Regex;

type Parsed<'a> = Vec<&'a BStr>;

pub fn parsing(input: &BString) -> color_eyre::Result<Parsed> {
    Ok(input.lines().map(|l| l.as_bstr()).collect())
}

fn neighbours<'a>(
    input: &'a Parsed,
    x: usize,
    y: usize,
) -> impl Iterator<Item = (usize, usize, u8)> + 'a {
    let min_x = if x == 0 { 0 } else { -1 };
    let min_y = if y == 0 { 0 } else { -1 };
    let max_x = if x == input[0].len() - 1 { 0 } else { 1 };
    let max_y = if y == input.len() - 1 { 0 } else { 1 };

    (min_x..=max_x)
        .flat_map(move |x| (min_y..=max_y).map(move |y| (x, y)))
        .filter(|&c| c != (0, 0))
        .map(move |(dx, dy)| {
            let x = (x as i64 + dx) as usize;
            let y = (y as i64 + dy) as usize;
            (x, y, input[y][x])
        })
}

fn parse_bytes(b: &[u8]) -> u64 {
    b.iter()
        .map(|d| {
            if d.is_ascii_digit() {
                (d - b'0') as u64
            } else {
                panic!("Invalid digit in {}", b.as_bstr())
            }
        })
        .fold(0, |acc, d| acc * 10 + d)
}

pub fn part1(input: Parsed) {
    let regex = Regex::new(r#"\d+"#).unwrap();
    let input = &input;

    let part_number_sum: u64 = input
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            regex
                .find_iter(line)
                .filter(move |m| {
                    m.range()
                        .flat_map(|x| neighbours(input, x, y))
                        .any(|(_, _, c)| !c.is_ascii_digit() && c != b'.')
                })
                .map(|m| parse_bytes(m.as_bytes()))
        })
        .sum();
    print_res!("Sum of part numbers: {part_number_sum}");
}

pub fn part2(input: Parsed) {
    let regex = Regex::new(r#"\d+"#).unwrap();
    let input = &input;

    let stars = input
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            regex.find_iter(line).flat_map(move |m| {
                let num = parse_bytes(m.as_bytes());
                let loc = (m.start(), y);
                m.range()
                    .flat_map(move |x| neighbours(input, x, y))
                    .filter_map(|(sx, sy, c)| if c == b'*' { Some((sx, sy)) } else { None })
                    .map(move |star| (star, loc, num))
            })
        })
        .fold(
            HashMap::<_, HashSet<_>>::new(),
            |mut acc, (star, num_loc, num)| {
                acc.entry(star).or_default().insert((num_loc, num));
                acc
            },
        );

    let gear_ratio_sum: u64 = stars
        .values()
        .filter(|s| s.len() == 2)
        .map(|s| s.iter().map(|&(_, n)| n).product::<u64>())
        .sum();

    print_res!("The gear ration sum is: {gear_ratio_sum}")
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

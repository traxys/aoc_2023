use std::time::Instant;

use aoc_2023::{load, print_res};
use bstr::{BStr, BString, ByteSlice};
use regex::bytes::Regex;

type Parsed<'a> = Vec<&'a BStr>;

pub fn parsing(input: &BString) -> color_eyre::Result<Parsed> {
    Ok(input.lines().map(|l| l.as_bstr()).collect())
}

fn neighbours<'a>(input: &'a Parsed, x: usize, y: usize) -> impl Iterator<Item = u8> + 'a {
    let min_x = if x == 0 { 0 } else { -1 };
    let min_y = if y == 0 { 0 } else { -1 };
    let max_x = if x == input[0].len() - 1 { 0 } else { 1 };
    let max_y = if y == input.len() - 1 { 0 } else { 1 };

    (min_x..=max_x)
        .flat_map(move |x| (min_y..=max_y).map(move |y| (x, y)))
        .filter(|&c| c != (0, 0))
        .map(move |(dx, dy)| input[(y as i64 + dy) as usize][(x as i64 + dx) as usize])
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
                        .any(|c| !c.is_ascii_digit() && c != b'.')
                })
                .map(|m| parse_bytes(m.as_bytes()))
        })
        .sum();
    print_res!("Sum of part numbers: {part_number_sum}");
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

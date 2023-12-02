use std::time::Instant;

use aho_corasick::AhoCorasick;
use aoc_2023::{load, print_res};
use bstr::{BStr, BString, ByteSlice};
use color_eyre::eyre;

type Parsed<'a> = Vec<&'a BStr>;

pub fn parsing(input: &BString) -> color_eyre::Result<Parsed> {
    Ok(input.lines().map(BStr::new).collect())
}

pub fn part1(input: Parsed) -> color_eyre::Result<()> {
    let calibration = input
        .iter()
        .map(|line| {
            let Some(first) = line.iter().find(|p| p.is_ascii_digit()) else {
                eyre::bail!("No digit in string");
            };

            let Some(last) = line.iter().rfind(|p| p.is_ascii_digit()) else {
                eyre::bail!("No digit in string");
            };

            let first = first - b'0';
            let last = last - b'0';

            Ok(first as u64 * 10 + last as u64)
        })
        .sum::<Result<u64, _>>()?;

    print_res!("Calibration is: {calibration}");
    Ok(())
}

pub fn part2(input: Parsed) {
    let patterns: &[&[u8]] = &[
        b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine", b"1",
        b"2", b"3", b"4", b"5", b"6", b"7", b"8", b"9",
    ];

    let matcher = AhoCorasick::new(patterns).unwrap();

    let calibration = input
        .iter()
        .map(|line| {
            let mut matches = matcher.find_overlapping_iter(line);
            let first = matches.next().unwrap();
            let last = matches.last().unwrap_or(first);

            fn to_num(s: &[u8]) -> u64 {
                match s {
                    b"one" => 1,
                    b"two" => 2,
                    b"three" => 3,
                    b"four" => 4,
                    b"five" => 5,
                    b"six" => 6,
                    b"seven" => 7,
                    b"eight" => 8,
                    b"nine" => 9,
                    _ => (s[0] - b'0') as u64,
                }
            }

            let first = to_num(patterns[first.pattern()]);
            let last = to_num(patterns[last.pattern()]);

            10 * first + last
        })
        .sum::<u64>();

    print_res!("Calibration is: {calibration}");
}

pub fn main() -> color_eyre::Result<()> {
    let context = load()?;

    let start = Instant::now();
    let parsed = parsing(&context.input)?;
    let elapsed = humantime::format_duration(start.elapsed());

    let start = Instant::now();
    if context.part == 1 {
        part1(parsed)?;
    } else {
        part2(parsed);
    }
    let elapsed_part = humantime::format_duration(start.elapsed());

    println!("  Parsing: {elapsed}");
    println!("  Solving: {elapsed_part}");

    Ok(())
}

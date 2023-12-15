use std::time::Instant;

use aoc_2023::{load, print_res};
use bstr::{BStr, BString, ByteSlice};
use indexmap::IndexMap;

type Parsed<'a> = Vec<&'a BStr>;

pub fn parsing(input: &BString) -> color_eyre::Result<Parsed> {
    Ok(input
        .trim()
        .split(|&c| c == b',')
        .map(|h| h.as_bstr())
        .collect())
}

fn ascii_hash(s: &BStr) -> u8 {
    let mut digest: u32 = 0;

    for &c in s.iter() {
        digest += c as u32;
        digest *= 17;
        digest %= 256;
    }

    digest as u8
}

pub fn part1(input: Parsed) {
    let hash_sum = input
        .iter()
        .copied()
        .map(ascii_hash)
        .map(u64::from)
        .sum::<u64>();
    print_res!("Hash sum: {hash_sum}");
}

pub fn part2(input: Parsed) {
    let mut boxes = vec![IndexMap::<&BStr, u8>::new(); 256];

    for lens in &input {
        if let Some(label) = lens.strip_suffix(b"-") {
            let hash = ascii_hash(label.into());
            boxes[hash as usize].shift_remove(&label.as_bstr());
        } else if let Some((label, focal)) = lens.split_once_str("=") {
            let hash = ascii_hash(label.into());
            let focal = std::str::from_utf8(focal).unwrap().parse().unwrap();
            *boxes[hash as usize].entry(label.into()).or_default() = focal;
        } else {
            panic!("Malformed lens: {lens}");
        }
    }

    let total_power = boxes
        .iter()
        .enumerate()
        .flat_map(|(i, b)| {
            b.iter()
                .enumerate()
                .map(move |(l, (_, &f))| (i + 1) * (l + 1) * (f as usize))
        })
        .sum::<usize>();

    print_res!("Total focussing power: {total_power}");
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

use std::{collections::VecDeque, time::Instant};

use aoc_2023::{load, print_res};
use bstr::BString;
use color_eyre::eyre::{eyre, Context};
use itertools::Itertools;

type Parsed = Vec<Vec<i64>>;

pub fn parsing(input: &BString) -> color_eyre::Result<Parsed> {
    std::str::from_utf8(input)?
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.split_whitespace()
                .map(|n| n.parse().wrap_err(eyre!("In line {i}")))
                .try_collect()
        })
        .collect()
}

fn difference(a: &[i64]) -> impl Iterator<Item = i64> + '_ {
    a.iter().tuple_windows().map(|(&a, &b)| b - a)
}

fn seqs_of_diffs(input: Vec<i64>) -> Vec<Vec<i64>> {
    let mut sequences = vec![input];

    loop {
        let mut all_zero = true;
        let diff_seq = difference(sequences.last().unwrap())
            .inspect(|&n| all_zero &= n == 0)
            .collect();

        if all_zero {
            break sequences;
        } else {
            sequences.push(diff_seq);
        }
    }
}

pub fn part1(input: Parsed) {
    let mut last_sum = 0;

    for sequence in input {
        let mut sequences = seqs_of_diffs(sequence);

        let last = sequences.last_mut().unwrap();
        last.push(last[0]);

        for i in 0..(sequences.len() - 1) {
            let idx = sequences.len() - 1 - i;
            let &diff = sequences[idx].last().unwrap();
            let &orig = sequences[idx - 1].last().unwrap();
            sequences[idx - 1].push(diff + orig)
        }

        last_sum += sequences[0].last().unwrap();
    }

    print_res!("Sum of continuations is: {last_sum}");
}

pub fn part2(input: Parsed) {
    let mut first_sum = 0;

    for sequence in input {
        let sequences = seqs_of_diffs(sequence);
        let mut sequences = sequences.into_iter().map(VecDeque::from).collect_vec();

        let last = sequences.last_mut().unwrap();
        last.push_front(last[0]);

        for i in 0..(sequences.len() - 1) {
            let idx = sequences.len() - 1 - i;
            let diff = sequences[idx][0];
            let orig = sequences[idx - 1][0];
            sequences[idx - 1].push_front(orig - diff)
        }

        first_sum += sequences[0][0];
    }

    print_res!("Sum of previsions is: {first_sum}");
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

use std::{cmp::Ordering, time::Instant};

use aoc_2023::{load, print_res};
use bstr::{BString, ByteSlice};
use itertools::Itertools;

type Parsed<'a> = Vec<Pattern>;

pub struct Pattern(Vec<Vec<bool>>);

impl std::fmt::Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.0 {
            for &c in line {
                write!(f, "{}", if c { '#' } else { '.' })?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

pub fn parsing(input: &BString) -> color_eyre::Result<Parsed> {
    input
        .split_str(b"\n\n")
        .map(|l| {
            Ok(Pattern(
                l.lines()
                    .map(|l| {
                        l.iter()
                            .map(|&c| match c {
                                b'.' => Ok(false),
                                b'#' => Ok(true),
                                _ => color_eyre::eyre::bail!("Invalid character: {}", c as char),
                            })
                            .try_collect()
                    })
                    .try_collect()?,
            ))
        })
        .try_collect()
}

fn position_iterator(start: usize, len: usize) -> impl Iterator<Item = usize> {
    (0..start).rev().interleave(start..(len - 1))
}

impl Pattern {
    fn vertical_reflection_at(&self, col: usize) -> bool {
        let mut offset = 0;

        let get_col = |idx: usize| self.0.iter().map(move |row| row[idx]);

        while col >= offset && col + 1 + offset < self.0[0].len() {
            //println!("col {col} offset {offset}:");

            let left = get_col(col - offset);
            let right = get_col(col + 1 + offset);

            if !left
                .zip_eq(right)
                // .inspect(|(l, r)| {
                //     let to_c = |&b| if b { '#' } else { '.' };
                //     println!("{} {}", to_c(l), to_c(r));
                // })
                .all(|(a, b)| a == b)
            {
                return false;
            }

            offset += 1;
        }

        true
    }

    fn vertical_reflection(&self) -> Option<usize> {
        //println!("Pattern:\n{self}");
        let p = &self.0;
        let len = p[0].len();
        let start = (len / 2) + (len % 2);

        position_iterator(start, len)
            .find(|&i| self.vertical_reflection_at(i))
            .map(|i| i + 1)
    }

    fn horizontal_reflection_at(&self, col: usize) -> bool {
        let mut offset = 0;

        while col >= offset && col + 1 + offset < self.0.len() {
            //println!("col {col} offset {offset}:");

            let left = self.0[col - offset].iter();
            let right = self.0[col + 1 + offset].iter();

            if !left
                .zip_eq(right)
                // .inspect(|(l, r)| {
                //     let to_c = |&b| if b { '#' } else { '.' };
                //     println!("{} {}", to_c(l), to_c(r));
                // })
                .all(|(a, b)| a == b)
            {
                return false;
            }

            offset += 1;
        }

        true
    }

    fn horizontal_reflection(&self) -> Option<usize> {
        //println!("Pattern:\n{self}");
        let p = &self.0;
        let len = p.len();
        let start = (len / 2) + (len % 2);

        position_iterator(start, len)
            .find(|&i| self.horizontal_reflection_at(i))
            .map(|i| i + 1)
    }

    fn reflection_score(&self) -> usize {
        //println!("Reflections for:\n{self}");

        let vertical = self.vertical_reflection();
        let horizontal = self.horizontal_reflection();

        match (vertical, horizontal) {
            (None, None) => {
                panic!("No reflection found in:\n{self}");
            }
            (Some(s), None) => s,
            (None, Some(s)) => s * 100,
            (Some(a), Some(b)) => panic!("Ambigous reflection vertical {a}, horizontal {b}"),
        }
    }
}

pub fn part1(input: Parsed) {
    let score = input.iter().map(Pattern::reflection_score).sum::<usize>();
    print_res!("Score: {score}");
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

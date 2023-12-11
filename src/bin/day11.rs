use std::time::Instant;

use aoc_2023::{load, print_res};
use bitvec::vec::BitVec;
use bstr::{BString, ByteSlice};
use itertools::Itertools;

#[derive(Debug)]
pub struct NebulaGrid(Vec<BitVec>);

impl std::fmt::Display for NebulaGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.0 {
            for b in line {
                match *b {
                    true => write!(f, "#")?,
                    false => write!(f, ".")?,
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

type Parsed = NebulaGrid;

pub fn parsing(input: &BString) -> color_eyre::Result<Parsed> {
    input
        .lines()
        .map(|line| {
            line.iter()
                .map(|&c| match c {
                    b'#' => Ok(true),
                    b'.' => Ok(false),
                    _ => color_eyre::eyre::bail!("Invalid character: {}", c as char),
                })
                .collect()
        })
        .try_collect()
        .map(NebulaGrid)
}

impl NebulaGrid {
    fn duplicate_columns(&self) -> Vec<usize> {
        let row_len = self.0[0].len();

        (0..row_len)
            .filter(|&col| self.0.iter().map(|r| r[col]).all(|a| !a))
            .collect_vec()
    }

    fn duplicate_rows(&self) -> Vec<usize> {
        self.0
            .iter()
            .enumerate()
            .filter(|(_, l)| l.not_any())
            .map(|(idx, _)| idx)
            .collect_vec()
    }

    fn total_distance(&self, empty_space: usize) -> usize {
        let dup_rows = self.duplicate_rows();
        let dup_cols = self.duplicate_columns();

        fn match_count(a: usize, b: usize, dups: &[usize]) -> usize {
            let (a, b) = (a.min(b), a.max(b));

            let start = dups.binary_search(&a).unwrap_err();
            let end = dups.binary_search(&b).unwrap_err();

            end - start
        }

        let distance = |a: usize, b: usize, dups: &[usize]| {
            let dup_count = match_count(a, b, dups);

            let abs_diff = a.abs_diff(b);

            (abs_diff - dup_count) + (dup_count * empty_space)
        };

        self.galaxies()
            .cartesian_product(self.galaxies())
            .filter(|(a, b)| a != b)
            .map(|((ax, ay), (bx, by))| distance(ax, bx, &dup_cols) + distance(ay, by, &dup_rows))
            .sum::<usize>()
            / 2
    }

    fn galaxies(&self) -> impl Iterator<Item = (usize, usize)> + '_ + Clone {
        self.0.iter().enumerate().flat_map(|(y, line)| {
            line.iter().enumerate().filter_map(move |(x, c)| match *c {
                true => Some((x, y)),
                false => None,
            })
        })
    }
}

pub fn part1(input: Parsed) {
    let total_distance = input.total_distance(2);

    print_res!("Total distance: {total_distance}");
}

pub fn part2(input: Parsed) {
    let total_distance = input.total_distance(1_000_000);

    print_res!("Total distance: {total_distance}");
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

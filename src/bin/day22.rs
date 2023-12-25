use std::{collections::HashSet, str::FromStr, time::Instant};

use ahash::HashMap;
use aoc_2023::{load, print_res};
use bstr::BString;
use fxhash::FxHashSet;
use itertools::Itertools;

#[derive(Clone, Copy, Debug)]
struct Vec3 {
    x: u64,
    y: u64,
    z: u64,
}

impl FromStr for Vec3 {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y, z) = s
            .split(',')
            .map(str::parse)
            .collect_tuple()
            .ok_or_else(|| color_eyre::eyre::eyre!("Vec3 must have exactly three fields: {s}"))?;

        Ok(Self {
            x: x?,
            y: y?,
            z: z?,
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Block {
    a: Vec3,
    b: Vec3,
}

impl Block {
    fn height(&self) -> u64 {
        self.a.z.abs_diff(self.b.z) + 1
    }
}

impl FromStr for Block {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s
            .split_once('~')
            .ok_or_else(|| color_eyre::eyre::eyre!("Block must have two ends: {s}"))?;

        Ok(Self {
            a: a.parse()?,
            b: b.parse()?,
        })
    }
}

type Parsed = Vec<Block>;

pub fn parsing(input: &BString) -> color_eyre::Result<Parsed> {
    std::str::from_utf8(input)?
        .lines()
        .map(str::parse)
        .collect()
}

pub fn part1(mut input: Parsed) {
    input.sort_by_key(|bl| std::cmp::min(bl.a.z, bl.b.z));

    let mut height_map = HashMap::default();
    let mut multi_support = HashSet::new();
    let mut single_support = HashSet::new();
    let mut uncovered = HashSet::new();

    for (p, piece) in input.iter().enumerate() {
        let x = std::cmp::min(piece.a.x, piece.b.x);
        let y = std::cmp::min(piece.a.y, piece.b.y);

        uncovered.insert(p);

        let span_x = piece.a.x.abs_diff(piece.b.x) + 1;
        let span_y = piece.a.y.abs_diff(piece.b.y) + 1;

        let mut height = 0;
        for px in x..x + span_x {
            for py in y..y + span_y {
                if let Some(&(_, h)) = height_map.get(&(px, py)) {
                    height = std::cmp::max(h, height);
                }
            }
        }

        let after_height = height + piece.height();

        let mut under = FxHashSet::default();
        for px in x..x + span_x {
            for py in y..y + span_y {
                if let Some(&(u, h)) = height_map.get(&(px, py)) {
                    if h == height {
                        under.insert(u);
                        uncovered.remove(&u);
                    }
                }

                height_map.insert((px, py), (p, after_height));
            }
        }

        if under.len() > 1 {
            multi_support.extend(under.into_iter());
        } else {
            single_support.extend(under.into_iter());
        }
    }

    let multi_supporting = multi_support.difference(&single_support).count();

    print_res!("Zappable count: {}", multi_supporting + uncovered.len());
}

fn heights(input: &Parsed, skip: Option<usize>) -> Vec<Option<u64>> {
    let mut height_map = HashMap::default();
    let mut pieces_height = vec![None; input.len()];

    for (p, piece) in input.iter().enumerate() {
        if Some(p) == skip {
            continue;
        }

        let x = std::cmp::min(piece.a.x, piece.b.x);
        let y = std::cmp::min(piece.a.y, piece.b.y);

        let span_x = piece.a.x.abs_diff(piece.b.x) + 1;
        let span_y = piece.a.y.abs_diff(piece.b.y) + 1;

        let mut height = 0;
        for px in x..x + span_x {
            for py in y..y + span_y {
                if let Some(&(_, h)) = height_map.get(&(px, py)) {
                    height = std::cmp::max(h, height);
                }
            }
        }

        pieces_height[p] = Some(height);
        let after_height = height + piece.height();

        let mut under = FxHashSet::default();
        for px in x..x + span_x {
            for py in y..y + span_y {
                if let Some(&(u, h)) = height_map.get(&(px, py)) {
                    if h == height {
                        under.insert(u);
                    }
                }

                height_map.insert((px, py), (p, after_height));
            }
        }
    }

    pieces_height
}

pub fn part2(mut input: Parsed) {
    input.sort_by_key(|bl| std::cmp::min(bl.a.z, bl.b.z));

    let normal_heights = heights(&input, None);

    let sum_of_fall: usize = (0..input.len())
        .map(|zapp| {
            let zapped_heights = heights(&input, Some(zapp));

            normal_heights
                .iter()
                .zip(zapped_heights)
                .filter(|&(&normal, zapped)| zapped.is_some() && zapped != normal)
                .count()
        })
        .sum();

    print_res!("Sum of falls: {sum_of_fall}");
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

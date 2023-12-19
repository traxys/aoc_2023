use std::{collections::HashSet, time::Instant};

use aoc_2023::{load, print_res};
use bstr::BString;
use color_eyre::eyre::{self, eyre};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Step<'a> {
    direction: Direction,
    amount: i64,
    color: &'a str,
}

type Parsed<'a> = Vec<Step<'a>>;

pub fn parsing(input: &BString) -> color_eyre::Result<Parsed> {
    std::str::from_utf8(input)?
        .lines()
        .map(|step| {
            let (dir, rest) = step
                .split_once(' ')
                .ok_or_else(|| eyre!("Missing ' ' in {step}"))?;

            let (amount, color) = rest
                .split_once(' ')
                .ok_or_else(|| eyre!("Missing 2nd ' ' in {step}"))?;

            Ok(Step {
                direction: match dir {
                    "R" => Direction::Right,
                    "U" => Direction::Up,
                    "D" => Direction::Down,
                    "L" => Direction::Left,
                    _ => eyre::bail!("Invalid direction: {dir}"),
                },
                amount: amount.parse()?,
                color: color
                    .trim_matches(&['(', ')'] as &[_])
                    .strip_prefix('#')
                    .ok_or_else(|| eyre!("Malformed color: {color}"))?,
            })
        })
        .collect()
}

#[allow(unused)]
fn display_trench(trenched: &HashSet<(i32, i32)>, marked: Option<(i32, i32)>) {
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;

    for &(x, y) in trenched {
        min_x = std::cmp::min(x, min_x);
        min_y = std::cmp::min(y, min_y);

        max_x = std::cmp::max(x, max_x);
        max_y = std::cmp::max(y, max_y);
    }

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if Some((x, y)) == marked {
                print!("X");
            } else if trenched.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn trenched_count<I>(iter: I) -> usize
where
    I: Iterator<Item = (i64, Direction)>,
{
    let mut pos = (0, 0);

    let mut edges = Vec::new();

    for (amount, dir) in iter {
        let next = match dir {
            Direction::Up => (pos.0, pos.1 - amount),
            Direction::Down => (pos.0, pos.1 + amount),
            Direction::Left => (pos.0 - amount, pos.1),
            Direction::Right => (pos.0 + amount, pos.1),
        };

        edges.push((pos, next));
        pos = next;
    }

    edges.sort_by(|&edge_a, &edge_b| {
        let x_min = |((ax, _), (bx, _))| std::cmp::min(ax, bx);
        let x_spread = |((ax, _), (bx, _)): ((i64, _), (i64, _))| ax.abs_diff(bx);

        x_min(edge_a)
            .cmp(&x_min(edge_b))
            .then(x_spread(edge_a).cmp(&x_spread(edge_b)))
    });

    let mut min_x = i64::MAX;
    let mut min_y = i64::MAX;
    let mut max_x = i64::MIN;
    let mut max_y = i64::MIN;

    for &(x, y) in edges.iter().map(|(s, _)| s) {
        min_x = std::cmp::min(x, min_x);
        min_y = std::cmp::min(y, min_y);

        max_x = std::cmp::max(x, max_x);
        max_y = std::cmp::max(y, max_y);
    }

    let mut total = 0;
    for y in min_y..=max_y {
        let mut x = min_x - 1;
        let mut inside = false;

        // println!("y={y}");

        for &(a, b) in edges.iter().filter(|&&((_, ay), (_, by))| {
            (std::cmp::min(ay, by)..=std::cmp::max(ay, by)).contains(&y)
        }) {
            // println!(" Edge {a:?}->{b:?}");
            let edge_start = std::cmp::min(a.0, b.0);
            let edge_end = std::cmp::max(a.0, b.0);

            if inside && x <= edge_start {
                total += edge_start - x;
                // println!("  Adding inside {x}..{edge_start} ({})", edge_start - x);
            }

            match a.1 == b.1 {
                true => {
                    total += edge_end - edge_start - 1;
                    x = edge_end + 1;
                    // println!(
                    //     "  Adding {}..{edge_end} ({})",
                    //     edge_start + 1,
                    //     edge_end - edge_start - 1
                    // );
                }
                false => {
                    // println!("  Adding {}", a.0);
                    total += 1;
                    x = a.0 + 1;

                    if (a.1 == y && b.1 > y) || (b.1 == y && a.1 > y) || (a.1 != y && b.1 != y) {
                        inside ^= true;
                    }
                }
            }
        }

        // println!("Total: {total}");
    }

    total as usize
}

fn assert_perpendicular(a: Direction, b: Direction) {
    match (a, b) {
        (Direction::Left | Direction::Right, Direction::Left | Direction::Right)
        | (Direction::Up | Direction::Down, Direction::Up | Direction::Down) => {
            panic!("Not perpendicular: {a:?} & {b:?}")
        }
        _ => (),
    }
}

pub fn part1(input: Parsed) {
    assert_perpendicular(input[0].direction, input.last().unwrap().direction);

    let count = trenched_count(input.iter().map(|s| (s.amount, s.direction)));

    print_res!("Size of pool: {count}");
}

pub fn part2(input: Parsed) {
    let steps = input
        .iter()
        .map(|s| {
            let (dist, dir) = (&s.color[0..5], &s.color[5..]);
            (
                i64::from_str_radix(dist, 16).unwrap(),
                match dir {
                    "0" => Direction::Right,
                    "1" => Direction::Down,
                    "2" => Direction::Left,
                    "3" => Direction::Up,
                    _ => unreachable!(),
                },
            )
        })
        .collect_vec();

    assert_perpendicular(steps[0].1, steps.last().unwrap().1);

    let count = trenched_count(steps.iter().copied());

    print_res!("Size of pool: {count}");
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

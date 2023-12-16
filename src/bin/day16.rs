use std::{collections::HashSet, time::Instant};

use aoc_2023::{load, print_res};
use bstr::{BStr, BString, ByteSlice};
use itertools::Itertools;

type Parsed<'a> = Vec<&'a BStr>;

pub fn parsing(input: &BString) -> color_eyre::Result<Parsed> {
    Ok(input.lines().map(ByteSlice::as_bstr).collect_vec())
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Beam {
    x: isize,
    y: isize,
    direction: Direction,
}

impl Beam {
    fn step_forwards(&mut self) {
        match self.direction {
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
        }
    }

    fn passes_through(&self, grid: &[&BStr]) -> HashSet<(isize, isize)> {
        let mut stack = vec![*self];

        let mut seen = HashSet::new();

        while let Some(mut beam) = stack.pop() {
            if !(0..(grid[0].len() as isize)).contains(&beam.x)
                || !(0..(grid.len() as isize)).contains(&beam.y)
                || seen.contains(&beam)
            {
                continue;
            }

            seen.insert(beam);
            match grid[beam.y as usize][beam.x as usize] {
                b'.' => (),
                b'-' => match beam.direction {
                    Direction::Up | Direction::Down => {
                        let mut left = Beam {
                            direction: Direction::Left,
                            ..beam
                        };
                        left.step_forwards();
                        stack.push(left);

                        let mut right = Beam {
                            direction: Direction::Right,
                            ..beam
                        };
                        right.step_forwards();
                        stack.push(right);

                        continue;
                    }
                    _ => (),
                },
                b'|' => match beam.direction {
                    Direction::Right | Direction::Left => {
                        let mut up = Beam {
                            direction: Direction::Up,
                            ..beam
                        };
                        up.step_forwards();
                        stack.push(up);

                        let mut down = Beam {
                            direction: Direction::Down,
                            ..beam
                        };
                        down.step_forwards();
                        stack.push(down);

                        continue;
                    }
                    _ => (),
                },
                b'/' => {
                    beam.direction = match beam.direction {
                        Direction::Left => Direction::Down,
                        Direction::Right => Direction::Up,
                        Direction::Up => Direction::Right,
                        Direction::Down => Direction::Left,
                    };
                }
                b'\\' => {
                    beam.direction = match beam.direction {
                        Direction::Left => Direction::Up,
                        Direction::Right => Direction::Down,
                        Direction::Up => Direction::Left,
                        Direction::Down => Direction::Right,
                    };
                }
                _ => unreachable!("Invalid input"),
            }

            beam.step_forwards();

            stack.push(beam);
        }

        seen.iter().map(|b| (b.x, b.y)).collect()
    }
}

pub fn part1(input: Parsed) {
    let coords = Beam {
        x: 0,
        y: 0,
        direction: Direction::Right,
    }
    .passes_through(&input);

    print_res!("Energisted count: {}", coords.len());
}

pub fn part2(input: Parsed) {
    let max = (0..input[0].len())
        .flat_map(|x| {
            [
                Beam {
                    x: x as isize,
                    y: 0,
                    direction: Direction::Down,
                },
                Beam {
                    x: x as isize,
                    y: (input.len() - 1) as isize,
                    direction: Direction::Up,
                },
            ]
        })
        .chain((0..input.len()).flat_map(|y| {
            [
                Beam {
                    x: 0,
                    y: y as isize,
                    direction: Direction::Right,
                },
                Beam {
                    x: (input[0].len() - 1) as isize,
                    y: y as isize,
                    direction: Direction::Left,
                },
            ]
        }))
        .map(|b| b.passes_through(&input).len())
        .max()
        .unwrap();

    print_res!("Max energised: {}", max);
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

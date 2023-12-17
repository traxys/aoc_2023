use std::{
    collections::{BinaryHeap, HashMap},
    time::Instant,
};

use aoc_2023::{load, print_res};
use bstr::{BString, ByteSlice};

type Parsed = Vec<Vec<u8>>;

pub fn parsing(input: &BString) -> color_eyre::Result<Parsed> {
    input
        .lines()
        .map(|line| {
            line.iter()
                .map(|&d| match d {
                    b'0'..=b'9' => Ok(d - b'0'),
                    _ => Err(color_eyre::eyre::eyre!("Invalid digit: {}", d as char)),
                })
                .collect()
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn perpendicular(&self) -> [Self; 2] {
        match self {
            Direction::Up | Direction::Down => [Direction::Left, Direction::Right],
            Direction::Left | Direction::Right => [Direction::Up, Direction::Down],
        }
    }

    fn step(&self, x: usize, y: usize, max_x: usize, max_y: usize) -> Option<(usize, usize)> {
        match self {
            Direction::Up if y != 0 => Some((x, y - 1)),
            Direction::Down if y != max_y => Some((x, y + 1)),
            Direction::Left if x != 0 => Some((x - 1, y)),
            Direction::Right if x != max_x => Some((x + 1, y)),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Cauldron {
    direction: Direction,
    speed: u8,
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct Path {
    cauldron: Cauldron,
    heat_loss: u64,
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.heat_loss.cmp(&self.heat_loss)
    }
}

fn min_path(input: &[Vec<u8>], turn_speed: u8, max_speed: u8) -> u64 {
    let mut paths = BinaryHeap::new();
    paths.push(Path {
        heat_loss: 0,
        cauldron: Cauldron {
            direction: Direction::Right,
            speed: 1,
            x: 1,
            y: 0,
        },
    });
    paths.push(Path {
        heat_loss: 0,
        cauldron: Cauldron {
            direction: Direction::Down,
            speed: 1,
            x: 0,
            y: 1,
        },
    });

    let mut visited = HashMap::new();

    loop {
        let Path {
            cauldron,
            mut heat_loss,
        } = paths.pop().unwrap();

        heat_loss += input[cauldron.y][cauldron.x] as u64;

        match visited.get_mut(&cauldron) {
            None => {
                visited.insert(cauldron, heat_loss);
            }
            Some(&mut v) if v <= heat_loss => continue,
            Some(v) => *v = heat_loss,
        }

        if cauldron.y == input.len() - 1 && cauldron.x == input[0].len() - 1 {
            break heat_loss;
        }

        let mut push = |cauldron| {
            paths.push(Path {
                heat_loss,
                cauldron,
            })
        };

        let max_x = input[0].len() - 1;
        let max_y = input.len() - 1;

        if cauldron.speed >= turn_speed {
            for direction in cauldron.direction.perpendicular() {
                if let Some((x, y)) = direction.step(cauldron.x, cauldron.y, max_x, max_y) {
                    push(Cauldron {
                        direction,
                        speed: 1,
                        x,
                        y,
                    });
                }
            }
        }

        if cauldron.speed < max_speed {
            if let Some((x, y)) = cauldron
                .direction
                .step(cauldron.x, cauldron.y, max_x, max_y)
            {
                push(Cauldron {
                    direction: cauldron.direction,
                    x,
                    y,
                    speed: cauldron.speed + 1,
                });
            }
        }
    }
}

pub fn part1(input: Parsed) {
    let min_heat_loss = min_path(&input, 1, 3);

    print_res!("Min heat loss: {min_heat_loss}")
}

pub fn part2(input: Parsed) {
    let min_heat_loss = min_path(&input, 4, 10);

    print_res!("Min ultra heat loss: {min_heat_loss}")
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

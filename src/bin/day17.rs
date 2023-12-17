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

        match cauldron.direction {
            dir @ (Direction::Up | Direction::Down) => {
                if cauldron.x != 0 && cauldron.speed >= turn_speed {
                    push(Cauldron {
                        direction: Direction::Left,
                        speed: 1,
                        x: cauldron.x - 1,
                        y: cauldron.y,
                    });
                }

                if cauldron.x != input[0].len() - 1 && cauldron.speed >= turn_speed {
                    push(Cauldron {
                        direction: Direction::Right,
                        speed: 1,
                        x: cauldron.x + 1,
                        y: cauldron.y,
                    })
                }

                if dir == Direction::Up && cauldron.y != 0 && cauldron.speed < max_speed {
                    push(Cauldron {
                        direction: Direction::Up,
                        speed: cauldron.speed + 1,
                        x: cauldron.x,
                        y: cauldron.y - 1,
                    });
                }

                if dir == Direction::Down
                    && cauldron.y != input.len() - 1
                    && cauldron.speed < max_speed
                {
                    push(Cauldron {
                        direction: Direction::Down,
                        speed: cauldron.speed + 1,
                        x: cauldron.x,
                        y: cauldron.y + 1,
                    });
                }
            }
            dir @ (Direction::Left | Direction::Right) => {
                if cauldron.y != 0 && cauldron.speed >= turn_speed {
                    push(Cauldron {
                        direction: Direction::Up,
                        speed: 1,
                        x: cauldron.x,
                        y: cauldron.y - 1,
                    });
                }

                if cauldron.y != input.len() - 1 && cauldron.speed >= turn_speed {
                    push(Cauldron {
                        direction: Direction::Down,
                        speed: 1,
                        x: cauldron.x,
                        y: cauldron.y + 1,
                    })
                }

                if dir == Direction::Left && cauldron.x != 0 && cauldron.speed < max_speed {
                    push(Cauldron {
                        direction: Direction::Left,
                        speed: cauldron.speed + 1,
                        x: cauldron.x - 1,
                        y: cauldron.y,
                    });
                }

                if dir == Direction::Right
                    && cauldron.x != input[0].len() - 1
                    && cauldron.speed < max_speed
                {
                    push(Cauldron {
                        direction: Direction::Right,
                        speed: cauldron.speed + 1,
                        x: cauldron.x + 1,
                        y: cauldron.y,
                    });
                }
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

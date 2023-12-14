use std::{collections::HashMap, time::Instant};

use aoc_2023::{load, print_res};
use bstr::{BString, ByteSlice};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Slot {
    Empty,
    Rock,
    Stop,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Board(Vec<Vec<Slot>>);

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.0 {
            for e in line {
                write!(
                    f,
                    "{}",
                    match e {
                        Slot::Empty => '.',
                        Slot::Rock => 'O',
                        Slot::Stop => '#',
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

type Parsed = Board;

pub fn parsing(input: &BString) -> color_eyre::Result<Parsed> {
    input
        .lines()
        .map(|l| {
            l.iter()
                .map(|&c| {
                    Ok(match c {
                        b'.' => Slot::Empty,
                        b'O' => Slot::Rock,
                        b'#' => Slot::Stop,
                        _ => color_eyre::eyre::bail!("Invalid slot: {}", c as char),
                    })
                })
                .collect()
        })
        .try_collect()
        .map(Board)
}

fn empty(Board(shape): &Board) -> Board {
    Board(vec![vec![Slot::Empty; shape[0].len()]; shape.len()])
}

impl Board {
    fn load(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            .map(|(i, r)| (self.0.len() - i) * r.iter().filter(|&&s| s == Slot::Rock).count())
            .sum()
    }

    // Board `to` should start empty
    fn fold_column_north_into(&self, idx: usize, Board(to): &mut Board) {
        let from = &self.0;

        let mut stop_row = 0;
        let mut rock_count = 0;

        let mut set_range = |start: usize, len: usize, elem| {
            to[start..start + len]
                .iter_mut()
                .for_each(|row| row[idx] = elem)
        };

        for (i, row) in from.iter().enumerate() {
            match row[idx] {
                Slot::Empty => (),
                Slot::Rock => rock_count += 1,
                Slot::Stop => {
                    // Total len = i - stop_row = rock_count + empty_count
                    //  empty_count = i - stop_row - rock_count
                    //
                    //  Example: i = 8, stop_row = 0, rock_count = 4
                    //  set_range(0, 4)
                    set_range(stop_row, rock_count, Slot::Rock);
                    set_range(i, 1, Slot::Stop);

                    stop_row = i + 1;
                    rock_count = 0;
                }
            }
        }

        if rock_count > 0 {
            set_range(stop_row, rock_count, Slot::Rock);
        }
    }

    fn fold_column_south_into(&self, idx: usize, Board(to): &mut Board) {
        let from = &self.0;

        let mut stop_row = from.len() - 1;
        let mut rock_count = 0;

        let mut set_range = |start: usize, len: usize, elem| {
            to[start + 1 - len..start + 1]
                .iter_mut()
                .for_each(|row| row[idx] = elem)
        };

        for (i, row) in from.iter().enumerate().rev() {
            match row[idx] {
                Slot::Empty => (),
                Slot::Rock => rock_count += 1,
                Slot::Stop => {
                    set_range(i, 1, Slot::Stop);
                    set_range(stop_row, rock_count, Slot::Rock);

                    if i != 0 {
                        stop_row = i - 1;
                    }
                    rock_count = 0;
                }
            }
        }

        if rock_count > 0 {
            set_range(stop_row, rock_count, Slot::Rock);
        }
    }

    fn fold_column_west_into(&self, idx: usize, Board(to): &mut Board) {
        let from = &self.0;

        let mut stop_col = 0;
        let mut rock_count = 0;

        let mut set_range = |start: usize, len: usize, elem| {
            to[idx][start..start + len].fill(elem);
        };

        for (i, col) in from[idx].iter().enumerate() {
            match col {
                Slot::Empty => (),
                Slot::Rock => rock_count += 1,
                Slot::Stop => {
                    set_range(stop_col, rock_count, Slot::Rock);
                    set_range(i, 1, Slot::Stop);

                    stop_col = i + 1;
                    rock_count = 0;
                }
            }
        }

        if rock_count > 0 {
            set_range(stop_col, rock_count, Slot::Rock);
        }
    }

    fn fold_column_east_into(&self, idx: usize, Board(to): &mut Board) {
        let from = &self.0;

        let mut stop_col = from[0].len() - 1;
        let mut rock_count = 0;

        let mut set_range =
            |start: usize, len: usize, elem| to[idx][start + 1 - len..start + 1].fill(elem);

        for (i, col) in from[idx].iter().enumerate().rev() {
            match col {
                Slot::Empty => (),
                Slot::Rock => rock_count += 1,
                Slot::Stop => {
                    set_range(stop_col, rock_count, Slot::Rock);
                    set_range(i, 1, Slot::Stop);

                    if i != 0 {
                        stop_col = i - 1;
                    }
                    rock_count = 0;
                }
            }
        }

        if rock_count > 0 {
            set_range(stop_col, rock_count, Slot::Rock);
        }
    }

    fn cycle(&mut self) {
        let mut next = empty(self);
        for idx in 0..self.0[0].len() {
            self.fold_column_north_into(idx, &mut next);
        }
        *self = next;

        let mut next = empty(self);
        for idx in 0..self.0.len() {
            self.fold_column_west_into(idx, &mut next);
        }
        *self = next;

        let mut next = empty(self);
        for idx in 0..self.0[0].len() {
            self.fold_column_south_into(idx, &mut next);
        }
        *self = next;

        let mut next = empty(self);
        for idx in 0..self.0.len() {
            self.fold_column_east_into(idx, &mut next);
        }
        *self = next;
    }
}

pub fn part1(input: Parsed) {
    let mut rolled = empty(&input);

    for idx in 0..input.0[0].len() {
        input.fold_column_north_into(idx, &mut rolled);
    }

    print_res!("Load is {}", rolled.load());
}

pub fn part2(mut input: Parsed) {
    let mut cycles = HashMap::new();
    let mut count = 0;

    while !cycles.contains_key(&input) {
        cycles.insert(input.clone(), count);
        input.cycle();
        count += 1;
    }

    let last = cycles.get(&input).unwrap();
    let cycle_len = count - last;

    let todo = (1000000000 - count) % cycle_len;

    for _ in 0..todo {
        input.cycle();
    }

    print_res!(
        "Load after 1000000000 cycles (period: {cycle_len}): {}",
        input.load()
    );
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

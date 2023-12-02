use std::{str::FromStr, time::Instant};

use aoc_2023::{load, print_res};
use bstr::{BString, ByteSlice};
use color_eyre::eyre::{self, eyre};
use enum_map::{Enum, EnumMap};

#[derive(Debug, Enum)]
pub enum Color {
    Blue,
    Red,
    Green,
}

impl FromStr for Color {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "red" => Self::Red,
            "blue" => Self::Blue,
            "green" => Self::Green,
            _ => eyre::bail!("Invalid color: {s}"),
        })
    }
}

type Parsed = Vec<Vec<EnumMap<Color, usize>>>;

pub fn parsing(input: &BString) -> color_eyre::Result<Parsed> {
    input
        .lines()
        .enumerate()
        .map(|(i, game)| {
            let game = std::str::from_utf8(game)?;
            let Some((prefix, game)) = game.split_once(':') else {
                eyre::bail!("No ':' in {game}")
            };

            let prefix: usize = prefix
                .strip_prefix("Game ")
                .ok_or_else(|| eyre!("Game {i} not starting with 'Game'"))?
                .trim()
                .parse()?;
            eyre::ensure!(prefix == i + 1, "Game {i} is misnumbered");

            game.split(';')
                .map(|draw| {
                    draw.split(',')
                        .map(|cube| -> color_eyre::Result<_> {
                            let (amount, color) = cube
                                .trim()
                                .split_once(' ')
                                .ok_or_else(|| eyre!("Malformed cube {cube} in game {i}"))?;

                            Ok((amount.parse::<usize>()?, color.parse()?))
                        })
                        .try_fold(EnumMap::default(), |mut map, res| {
                            let (amount, color) = res?;
                            map[color] += amount;
                            Ok(map)
                        })
                })
                .collect()
        })
        .collect()
}

pub fn part1(input: Parsed) {
    let possible_games: usize = input
        .iter()
        .enumerate()
        .filter(|(_, draws)| {
            draws.iter().all(|draw| {
                draw[Color::Red] <= 12 && draw[Color::Green] <= 13 && draw[Color::Blue] <= 14
            })
        })
        .map(|(id, _)| id + 1)
        .sum();
    print_res!("Sum of possible games: {possible_games}")
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

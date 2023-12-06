use std::{ops::RangeInclusive, time::Instant};

use aoc_2023::{load, print_res};
use bstr::BString;
use color_eyre::eyre::eyre;

#[derive(Clone, Copy, Debug)]
pub struct Race {
    duration: u64,
    record: u64,
}

type Parsed = Vec<Race>;

pub fn parsing(input: &BString) -> color_eyre::Result<Parsed> {
    let input = std::str::from_utf8(input)?;
    let (time, distance) = input
        .split_once('\n')
        .ok_or_else(|| eyre!("Malformed input {input}"))?;
    let time = time
        .strip_prefix("Time:")
        .ok_or_else(|| eyre!("Malformed time: {time}"))?
        .trim();
    let distance = distance
        .strip_prefix("Distance:")
        .ok_or_else(|| eyre!("Malformed distance: {distance}"))?
        .trim();

    time.split_whitespace()
        .zip(distance.split_whitespace())
        .map(|(duration, record)| {
            Ok(Race {
                duration: duration.parse()?,
                record: record.parse()?,
            })
        })
        .collect()
}

impl Race {
    fn winning_charges(&self) -> RangeInclusive<u64> {
        let d_r = self.record as f64;
        let t_r = self.duration as f64;

        let delta = t_r * t_r - 4. * d_r;

        if delta < 0. {
            panic!("Race {self:?} can't be won")
        }

        let max = ((t_r + delta.sqrt()) / 2.).floor() as u64;
        let min = ((t_r - delta.sqrt()) / 2.).max(0.).ceil() as u64;

        let distance = |charge: u64| (self.duration - charge) * charge;

        let max = if distance(max) <= self.record {
            max - 1
        } else {
            max
        };

        let min = if distance(min) <= self.record {
            min + 1
        } else {
            min
        };

        min..=max
    }
}

pub fn part1(input: Parsed) {
    let possible_winning_charges_product: u64 = input
        .iter()
        .map(Race::winning_charges)
        .map(|r| r.end() - r.start() + 1)
        .product();
    print_res!("Product of wining charges: {possible_winning_charges_product}");
}

pub fn part2(input: Parsed) {
    let mut total_time = 0;
    let mut total_distance = 0;
    for race in input {
        total_time = total_time * 10u64.pow(race.duration.ilog10() + 1) + race.duration;
        total_distance = total_distance * 10u64.pow(race.record.ilog10() + 1) + race.record;
    }

    let single_race = Race {
        record: total_distance,
        duration: total_time,
    };

    let winning_range = single_race.winning_charges();
    let ways_to_win = winning_range.end() - winning_range.start() + 1;

    print_res!("Number of ways the race can be won: {ways_to_win}");
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

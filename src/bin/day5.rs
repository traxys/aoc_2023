use std::{collections::BTreeMap, time::Instant};

use aoc_2023::{load, print_res};
use bstr::BString;
use color_eyre::eyre::{self, eyre};

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub start: u64,
    pub len: u64,
}

impl std::cmp::PartialEq for Interval {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start
    }
}

impl std::cmp::Eq for Interval {}

impl std::cmp::PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Interval {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start.cmp(&other.start)
    }
}

type Mapping = BTreeMap<Interval, u64>;

#[derive(Debug)]
pub struct Mappings {
    pub seed_to_soil: Mapping,
    pub soil_to_fertilizer: Mapping,
    pub fertilizer_to_water: Mapping,
    pub water_to_light: Mapping,
    pub light_to_temperature: Mapping,
    pub temperature_to_humidity: Mapping,
    pub humidity_to_location: Mapping,
}

type Parsed = (Vec<u64>, Mappings);

pub fn parsing(input: &BString) -> color_eyre::Result<Parsed> {
    let input = std::str::from_utf8(input)?;

    let mut sections = input.split("\n\n");
    let seeds = sections
        .next()
        .ok_or_else(|| eyre!("Missing seed section"))?
        .strip_prefix("seeds:")
        .ok_or_else(|| eyre!("Invalid section name"))?
        .split_whitespace()
        .map(str::parse)
        .collect::<Result<Vec<u64>, _>>()?;

    fn parse_mapping<'a>(
        mut mapping: impl Iterator<Item = &'a str>,
        name: &str,
    ) -> color_eyre::Result<Mapping> {
        let section = mapping
            .next()
            .ok_or_else(|| eyre!("Missing section {name}"))?;

        let values = section
            .strip_prefix(&format!("{name} map:\n"))
            .ok_or_else(|| eyre!("Invalid section name '{name}': {section}"))?;

        let mut mapping = BTreeMap::new();

        for map in values.lines() {
            let &[to, from, len] = map
                .split_whitespace()
                .map(str::parse::<u64>)
                .collect::<Result<Vec<_>, _>>()?
                .as_slice()
            else {
                eyre::bail!("Invalid mapping: {map}")
            };
            mapping.insert(Interval { start: from, len }, to);
        }

        Ok(mapping)
    }

    Ok((
        seeds,
        Mappings {
            seed_to_soil: parse_mapping(sections.by_ref(), "seed-to-soil")?,
            soil_to_fertilizer: parse_mapping(sections.by_ref(), "soil-to-fertilizer")?,
            fertilizer_to_water: parse_mapping(sections.by_ref(), "fertilizer-to-water")?,
            water_to_light: parse_mapping(sections.by_ref(), "water-to-light")?,
            light_to_temperature: parse_mapping(sections.by_ref(), "light-to-temperature")?,
            temperature_to_humidity: parse_mapping(sections.by_ref(), "temperature-to-humidity")?,
            humidity_to_location: parse_mapping(sections.by_ref(), "humidity-to-location")?,
        },
    ))
}

impl Mappings {
    fn location(&self, seed: u64) -> u64 {
        fn translate(mapping: &Mapping, element: u64) -> u64 {
            match mapping
                .iter()
                .find(|(i, _)| i.start <= element && element < i.start + i.len)
            {
                Some((i, to)) => {
                    let offset = element - i.start;
                    to + offset
                }
                None => element,
            }
        }

        let soil = translate(&self.seed_to_soil, seed);
        let fertilizer = translate(&self.soil_to_fertilizer, soil);
        let water = translate(&self.fertilizer_to_water, fertilizer);
        let light = translate(&self.water_to_light, water);
        let temperature = translate(&self.light_to_temperature, light);
        let humidity = translate(&self.temperature_to_humidity, temperature);

        translate(&self.humidity_to_location, humidity)
    }
}

pub fn part1((seeds, mappings): Parsed) {
    let min_location = seeds
        .iter()
        .map(|&s| mappings.location(s))
        .min()
        .unwrap();
    print_res!("Min location: {min_location}")
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

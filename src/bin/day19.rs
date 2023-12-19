use std::{collections::HashMap, time::Instant};

use aoc_2023::{load, print_res};
use bstr::BString;
use enum_map::{Enum, EnumMap};
use itertools::Itertools;

#[derive(Enum, Debug, Clone, Copy)]
pub enum Spec {
    Cool,
    Musical,
    Aerodynamic,
    Shiny,
}

#[derive(Debug, Clone, Copy)]
pub enum Condition<'a> {
    Less(Spec, u16, &'a str),
    Greater(Spec, u16, &'a str),
    Jump(&'a str),
}

type Part = EnumMap<Spec, u16>;
type Workflow<'a> = HashMap<&'a str, Vec<Condition<'a>>>;
type Parsed<'a> = (Workflow<'a>, Vec<Part>);

pub fn parsing(input: &BString) -> color_eyre::Result<Parsed> {
    let input = std::str::from_utf8(input)?;

    let (workflows, parts) = input
        .split_once("\n\n")
        .ok_or_else(|| color_eyre::eyre::eyre!("Missing section separator"))?;

    let parse_spec = |s| {
        Ok(match s {
            "x" => Spec::Cool,
            "m" => Spec::Musical,
            "a" => Spec::Aerodynamic,
            "s" => Spec::Shiny,
            _ => color_eyre::eyre::bail!("Invalid spec: {s}"),
        })
    };

    let workflows = workflows
        .lines()
        .map(|l| {
            let (label, conditions) = l
                .split_once('{')
                .ok_or_else(|| color_eyre::eyre::eyre!("Missing workflow separator in {l}"))?;

            let conditions = conditions
                .strip_suffix('}')
                .ok_or_else(|| {
                    color_eyre::eyre::eyre!("Conditions do not end with '}}' in {conditions}",)
                })?
                .split(',')
                .map(|cond| match cond.split_once(':') {
                    Some((cond, to)) => {
                        let (spec, value) = cond.split_once(['<', '>']).ok_or_else(|| {
                            color_eyre::eyre::eyre!("Missing comparaison in {cond}")
                        })?;

                        let side = if cond.contains('<') {
                            Condition::Less
                        } else {
                            Condition::Greater
                        };

                        Ok::<_, color_eyre::Report>(side(parse_spec(spec)?, value.parse()?, to))
                    }
                    None => Ok(Condition::Jump(cond)),
                })
                .try_collect()?;

            Ok::<_, color_eyre::Report>((label, conditions))
        })
        .try_collect()?;

    let parts = parts
        .lines()
        .map(|p| {
            p.strip_suffix('}')
                .ok_or_else(|| color_eyre::eyre::eyre!("Missing '}}' in {p}"))?
                .strip_prefix('{')
                .ok_or_else(|| color_eyre::eyre::eyre!("Missing '{{' in {p}"))?
                .split(',')
                .map(|spec| {
                    let (name, value) = spec
                        .split_once('=')
                        .ok_or_else(|| color_eyre::eyre::eyre!("Missing '=' in {spec}"))?;

                    Ok::<_, color_eyre::Report>((parse_spec(name)?, value.parse()?))
                })
                .try_collect()
        })
        .try_collect()?;

    Ok((workflows, parts))
}

fn run_part(workflows: &Workflow, part: &Part) -> bool {
    let mut current = "in";

    loop {
        if current == "R" {
            return false;
        } else if current == "A" {
            return true;
        }

        let workflow = &workflows[&current];

        for &cond in workflow {
            match cond {
                Condition::Less(spec, v, to) => {
                    if part[spec] < v {
                        current = to;
                        break;
                    }
                }
                Condition::Greater(spec, v, to) => {
                    if part[spec] > v {
                        current = to;
                        break;
                    }
                }
                Condition::Jump(c) => {
                    current = c;
                    break;
                }
            }
        }
    }
}

pub fn part1((workflows, parts): Parsed) {
    let total_value = parts
        .iter()
        .filter(|&p| run_part(&workflows, p))
        .map(|p| p.values().map(|&v| v as u64).sum::<u64>())
        .sum::<u64>();
    print_res!("Total value is: {total_value}");
}

#[derive(Debug, Clone, Copy)]
struct SpecRange {
    min: u16,
    max: u16,
}

impl SpecRange {
    fn empty(&self) -> bool {
        self.min > self.max
    }
}

impl std::default::Default for SpecRange {
    fn default() -> Self {
        Self { min: 1, max: 4000 }
    }
}

pub fn part2((workflows, _): Parsed) {
    let mut possibilities = 0;

    let complete_range: EnumMap<Spec, SpecRange> = Default::default();

    let mut ranges = vec![("in", complete_range)];

    while let Some((workflow, mut r)) = ranges.pop() {
        if workflow == "R" {
            continue;
        } else if workflow == "A" {
            possibilities += r
                .values()
                .map(|r| (r.max - r.min + 1) as u64)
                .product::<u64>();
            continue;
        }

        let workflow = &workflows[&workflow];

        for &condition in workflow {
            match condition {
                Condition::Less(spec, amount, to) => {
                    let mut ok = r;

                    ok[spec].max = amount - 1;

                    if !ok[spec].empty() {
                        ranges.push((to, ok));
                    }

                    r[spec].min = amount;
                    if r[spec].empty() {
                        break;
                    }
                }
                Condition::Greater(spec, amount, to) => {
                    let mut ok = r;

                    ok[spec].min = amount + 1;

                    if !ok[spec].empty() {
                        ranges.push((to, ok));
                    }

                    r[spec].max = amount;
                    if r[spec].empty() {
                        break;
                    }
                }
                Condition::Jump(to) => {
                    ranges.push((to, r));
                    break;
                }
            }
        }
    }

    print_res!("Total number of possibilites: {possibilities}");
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

use std::{collections::HashMap, time::Instant};

use aoc_2023::{load, print_res};
use bstr::{BString, ByteSlice};
use color_eyre::eyre::ensure;
use enum_map::{Enum, EnumMap};
use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct GateName(u16);

impl std::fmt::Debug for GateName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("GateName")
            .field(&format_args!("{self}",))
            .finish()
    }
}

impl std::fmt::Display for GateName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            (self.0 >> 8) as u8 as char,
            (self.0 & 255) as u8 as char
        )
    }
}

impl GateName {
    fn from_bytes(a: &[u8]) -> color_eyre::Result<Self> {
        color_eyre::eyre::ensure!(a.len() <= 2, "Invalid name: {}", a.as_bstr());
        Ok(GateName((a[0] as u16) << 8 | a[1] as u16))
    }
}

#[derive(Debug)]
pub struct Broadcaster(Vec<GateName>);

#[derive(Debug)]
enum GateKind {
    FlipFlop,
    Conjunction,
}

#[derive(Debug)]
pub struct GateDesc {
    kind: GateKind,
    to: Vec<GateName>,
}

type Parsed<'a> = (Broadcaster, HashMap<GateName, GateDesc>);

pub fn parsing(input: &BString) -> color_eyre::Result<Parsed> {
    let mut brodcaster = None;
    let mut gates = HashMap::new();

    for line in input.lines() {
        let (name, outputs) = line
            .split_once_str(" -> ")
            .ok_or_else(|| color_eyre::eyre::eyre!("Missing separator in {}", line.as_bstr()))?;

        let outputs = outputs
            .split_str(", ")
            .map(GateName::from_bytes)
            .try_collect()?;

        if name == b"broadcaster" {
            ensure!(brodcaster.is_none(), "Two broadcasters");
            brodcaster = Some(Broadcaster(outputs));
        } else if name[0] == b'%' {
            let name = GateName::from_bytes(&name[1..])?;
            gates.insert(
                name,
                GateDesc {
                    kind: GateKind::FlipFlop,
                    to: outputs,
                },
            );
        } else if name[0] == b'&' {
            let name = GateName::from_bytes(&name[1..])?;
            gates.insert(
                name,
                GateDesc {
                    kind: GateKind::Conjunction,
                    to: outputs,
                },
            );
        } else {
            color_eyre::eyre::bail!("Invalid gate name: {}", name.as_bstr())
        }
    }

    Ok((
        brodcaster.ok_or_else(|| color_eyre::eyre::eyre!("No broadcaster found"))?,
        gates,
    ))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Enum)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug)]
enum GateState {
    FlipFlop { state: bool },
    Conjunction { from: HashMap<GateName, Pulse> },
}

struct Network<'a> {
    gates: HashMap<GateName, GateState>,
    desc: &'a HashMap<GateName, GateDesc>,
}

enum RunResult {
    Presses(enum_map::EnumMap<Pulse, u64>),
    Got,
}

impl<'a> Network<'a> {
    fn new(gate_desc: &'a HashMap<GateName, GateDesc>, broadcasters: &[GateName]) -> Self {
        Self {
            desc: gate_desc,
            gates: gate_desc
                .iter()
                .map(|(name, desc)| {
                    let state = match desc.kind {
                        GateKind::FlipFlop => GateState::FlipFlop { state: false },
                        GateKind::Conjunction => {
                            assert!(
                                !broadcasters.contains(name),
                                "Conjunction gates can't have broadcaster as a source"
                            );

                            GateState::Conjunction {
                                from: gate_desc
                                    .iter()
                                    .filter(|(_, src_desc)| src_desc.to.contains(name))
                                    .map(|(src, _)| (*src, Pulse::Low))
                                    .collect(),
                            }
                        }
                    };
                    (*name, state)
                })
                .collect(),
        }
    }

    fn run(&mut self, values: &[GateName], wait_for: Option<GateName>) -> RunResult {
        let mut pulse_count = enum_map::EnumMap::default();

        // println!("button -Low-> broadcaster");

        pulse_count[Pulse::Low] += 1;

        let mut pulses = Vec::new();

        for &name in values {
            pulses.push((None::<GateName>, name, Pulse::Low));
        }

        while !pulses.is_empty() {
            let mut new_pulses = Vec::new();

            for (src, to, len) in &pulses {
                if Some(*to) == wait_for && *len == Pulse::Low {
                    return RunResult::Got;
                }

                // println!(
                //     "{} -{len:?}-> {to}",
                //     src.as_ref()
                //         .map(|s| s.to_string())
                //         .as_deref()
                //         .unwrap_or("broadcaster")
                // );

                pulse_count[*len] += 1;

                let Some(state) = self.gates.get_mut(to) else {
                    continue;
                };

                let pulse = match state {
                    GateState::FlipFlop { state } => match len {
                        Pulse::Low => {
                            let s = *state;
                            *state ^= true;
                            Some(match s {
                                true => Pulse::Low,
                                false => Pulse::High,
                            })
                        }
                        Pulse::High => None,
                    },
                    GateState::Conjunction { from } => {
                        let src = src.unwrap();
                        from.insert(src, *len);

                        Some(match from.values().all(|&l| l == Pulse::High) {
                            true => Pulse::Low,
                            false => Pulse::High,
                        })
                    }
                };

                if let Some(p) = pulse {
                    for &next in &self.desc[to].to {
                        new_pulses.push((Some(*to), next, p))
                    }
                }
            }

            pulses = new_pulses;
        }

        RunResult::Presses(pulse_count)
    }
}

pub fn part1((broadcaster, gate_desc): Parsed) {
    let mut network = Network::new(&gate_desc, &broadcaster.0);

    let mut total_count = EnumMap::<_, u64>::default();
    for _ in 0..1000 {
        let RunResult::Presses(p) = network.run(&broadcaster.0, None) else {
            panic!("Got RX early");
        };
        p.iter().for_each(|(p, l)| total_count[p] += l);
    }

    print_res!(
        "Total pulses give: {}",
        total_count.values().product::<u64>()
    );
}

pub fn part2((broadcaster, gate_desc): Parsed) {
    let mut network = Network::new(&gate_desc, &broadcaster.0);
    let mut i = 0;
    let rx = GateName::from_bytes(b"rx").unwrap();

    let presses = loop {
        i += 1;
        if let RunResult::Got = network.run(&broadcaster.0, Some(rx)) {
            break i;
        }

        if i % 50000 == 0 {
            println!("{i}");
        }
    };

    print_res!("Button count required: {presses}");
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

use std::time::Instant;

use aoc_2023::{load, print_res};
use bstr::{BString, ByteSlice};
use color_eyre::eyre::ensure;
use enum_map::{Enum, EnumMap};
use fxhash::FxHashMap;
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

#[derive(Debug, PartialEq, Eq)]
enum GateKind {
    FlipFlop,
    Conjunction,
}

#[derive(Debug)]
pub struct GateDesc {
    kind: GateKind,
    to: Vec<GateName>,
}

type Parsed<'a> = (Broadcaster, FxHashMap<GateName, GateDesc>);

pub fn parsing(input: &BString) -> color_eyre::Result<Parsed> {
    let mut brodcaster = None;
    let mut gates = FxHashMap::default();

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Enum, Hash)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug)]
enum GateState {
    FlipFlop {
        state: bool,
    },
    Conjunction {
        from: FxHashMap<GateName, Pulse>,
        waiting: usize,
    },
}

struct Network<'a> {
    gates: FxHashMap<GateName, GateState>,
    desc: &'a FxHashMap<GateName, GateDesc>,
}

fn predecessors(
    gate_desc: &FxHashMap<GateName, GateDesc>,
    gate: GateName,
) -> impl Iterator<Item = GateName> + '_ {
    gate_desc
        .iter()
        .filter_map(move |(src, src_desc)| match src_desc.to.contains(&gate) {
            true => Some(*src),
            false => None,
        })
}

impl<'a> Network<'a> {
    fn new(gate_desc: &'a FxHashMap<GateName, GateDesc>, broadcasters: &[GateName]) -> Self {
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

                            let from: FxHashMap<_, _> = predecessors(gate_desc, *name)
                                .map(|src| (src, Pulse::Low))
                                .collect();
                            let waiting = from.len();

                            GateState::Conjunction { from, waiting }
                        }
                    };
                    (*name, state)
                })
                .collect(),
        }
    }

    fn run(
        &mut self,
        values: &[GateName],
        mut record: Option<&mut FxHashMap<(GateName, Pulse), usize>>,
    ) -> enum_map::EnumMap<Pulse, u64> {
        let mut pulse_count = enum_map::EnumMap::default();

        // println!("button -Low-> broadcaster");

        pulse_count[Pulse::Low] += 1;

        let mut pulses = Vec::new();

        for &name in values {
            pulses.push((None::<GateName>, name, Pulse::Low));
        }

        while !pulses.is_empty() {
            let mut new_pulses = Vec::new();

            for &(src, to, len) in &pulses {
                if let Some(r) = record.as_deref_mut() {
                    if let Some(src) = src {
                        if let Some(v) = r.get_mut(&(src, len)) {
                            *v += 1;
                        }
                    }
                }
                // println!(
                //     "{} -{len:?}-> {to}",
                //     src.as_ref()
                //         .map(|s| s.to_string())
                //         .as_deref()
                //         .unwrap_or("broadcaster")
                // );

                pulse_count[len] += 1;

                let Some(state) = self.gates.get_mut(&to) else {
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
                    GateState::Conjunction { from, waiting } => {
                        let src = src.unwrap();
                        let value = from.get_mut(&src).unwrap();

                        if *value != len {
                            match value {
                                Pulse::Low => *waiting -= 1,
                                Pulse::High => *waiting += 1,
                            }
                            *value = len;
                        }

                        Some(match *waiting == 0 {
                            true => Pulse::Low,
                            false => Pulse::High,
                        })
                    }
                };

                if let Some(p) = pulse {
                    for &next in &self.desc[&to].to {
                        new_pulses.push((Some(to), next, p))
                    }
                }
            }

            pulses = new_pulses;
        }

        pulse_count
    }
}

pub fn part1((broadcaster, gate_desc): Parsed) {
    let mut network = Network::new(&gate_desc, &broadcaster.0);

    let mut total_count = EnumMap::<_, u64>::default();
    for _ in 0..1000 {
        network
            .run(&broadcaster.0, None)
            .iter()
            .for_each(|(p, l)| total_count[p] += l);
    }

    print_res!(
        "Total pulses give: {}",
        total_count.values().product::<u64>()
    );
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }

    a
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

pub fn part2((broadcaster, gate_desc): Parsed) {
    let mut network = Network::new(&gate_desc, &broadcaster.0);
    let rx = GateName::from_bytes(b"rx").unwrap();

    let (rx_trigger,) = predecessors(&gate_desc, rx).collect_tuple().unwrap();
    assert!(gate_desc[&rx_trigger].kind == GateKind::Conjunction);

    let trigger_inputs = predecessors(&gate_desc, rx_trigger).collect_vec();

    let mut trigger_iterations: FxHashMap<_, _> =
        trigger_inputs.iter().map(|&g| (g, None)).collect();

    let mut i = 0;
    while trigger_iterations.values().any(|v| v.is_none()) {
        let mut record = trigger_inputs
            .iter()
            .map(|&g| ((g, Pulse::High), 0))
            .collect();

        i += 1;

        network.run(&broadcaster.0, Some(&mut record));

        for ((v, _), c) in record {
            assert!(c < 2, "{v} triggered twice");
            if c == 1 {
                let period = trigger_iterations.get_mut(&v).unwrap();
                if period.is_none() {
                    *period = Some(i);
                }
            }
        }
    }

    let all_period = trigger_iterations
        .values()
        .copied()
        .map(Option::unwrap)
        .fold(1, lcm);

    print_res!("Button count required: {all_period}");
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

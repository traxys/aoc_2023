use std::time::Instant;

use aoc_2023::{load, print_res};
use bstr::{BString, ByteSlice};
use itertools::Itertools;
use petgraph::{graphmap::UnGraphMap, visit::Bfs};

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Component(u32);

impl std::fmt::Display for Component {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let b = self.0.to_be_bytes();

        for i in b {
            if i != 0 {
                write!(f, "{}", i as char)?;
            }
        }

        Ok(())
    }
}

impl std::fmt::Debug for Component {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Component")
            .field(&format_args!("{self}"))
            .finish()
    }
}

impl Component {
    fn from_bytes(a: &[u8]) -> color_eyre::Result<Self> {
        color_eyre::eyre::ensure!(a.len() <= 4, "Component must be less than 4");
        let mut bytes = [0; 4];
        bytes[0..a.len()].copy_from_slice(a);

        Ok(Self(u32::from_be_bytes(bytes)))
    }
}

type Parsed = Vec<(Component, Vec<Component>)>;

pub fn parsing(input: &BString) -> color_eyre::Result<Parsed> {
    input
        .lines()
        .map(|l| {
            let (from, to) = l
                .split_once_str(": ")
                .ok_or_else(|| color_eyre::eyre::eyre!("Malformed line: {}", l.as_bstr()))?;

            Ok((
                Component::from_bytes(from)?,
                to.split(|&c| c == b' ')
                    .map(Component::from_bytes)
                    .try_collect()?,
            ))
        })
        .collect()
}

// crg -> krf
// jct -> rgv
// fmr -> zhg
pub fn part1(input: Parsed) {
    let crg = Component::from_bytes(b"crg").unwrap();
    let jct = Component::from_bytes(b"jct").unwrap();
    let fmr = Component::from_bytes(b"fmr").unwrap();
    let krf = Component::from_bytes(b"krf").unwrap();
    let rgv = Component::from_bytes(b"rgv").unwrap();
    let zhg = Component::from_bytes(b"zhg").unwrap();

    let mut graph = UnGraphMap::new();

    for (from, to) in &input {
        for &to in to {
            graph.add_edge(*from, to, ());
        }
    }

    graph.remove_edge(crg, krf).unwrap();
    graph.remove_edge(jct, rgv).unwrap();
    graph.remove_edge(fmr, zhg).unwrap();

    let mut bfs = Bfs::new(&graph, crg);
    let mut size_a = 0;
    while bfs.next(&graph).is_some() {
        size_a += 1;
    }

    let mut bfs = Bfs::new(&graph, krf);
    let mut size_b = 0;
    while bfs.next(&graph).is_some() {
        size_b += 1;
    }

    print_res!("Product of component sizes: {}", size_a * size_b);
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

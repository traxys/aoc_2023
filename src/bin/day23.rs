use std::{borrow::Cow, collections::HashMap, time::Instant};

use aoc_2023::{load, print_res};
use bitvec::{bitvec, vec::BitVec};
use bstr::{BString, ByteSlice};
use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Cell {
    Forest,
    Path,
    UpSlope,
    DownSlope,
    LeftSlope,
    RightSlope,
}

type Parsed = Vec<Vec<Cell>>;

pub fn parsing(input: &BString) -> color_eyre::Result<Parsed> {
    input
        .lines()
        .map(|l| {
            let l = l.as_bstr();
            l.iter()
                .map(|&c| {
                    Ok(match c {
                        b'#' => Cell::Forest,
                        b'.' => Cell::Path,
                        b'>' => Cell::RightSlope,
                        b'<' => Cell::LeftSlope,
                        b'^' => Cell::UpSlope,
                        b'v' => Cell::DownSlope,
                        _ => color_eyre::eyre::bail!("Invalid cell: {}", c as char),
                    })
                })
                .collect()
        })
        .collect()
}

fn ends(input: &[Vec<Cell>]) -> (usize, usize) {
    let ((start, _),) = input[0]
        .iter()
        .enumerate()
        .filter(|&(_, &c)| c == Cell::Path)
        .collect_tuple()
        .unwrap();
    let ((end, _),) = input
        .last()
        .unwrap()
        .iter()
        .enumerate()
        .filter(|&(_, &c)| c == Cell::Path)
        .collect_tuple()
        .unwrap();

    (start, end)
}

fn neighbours(x: usize, y: usize) -> [(usize, usize); 4] {
    [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
}

fn longest_path(
    to_x: usize,
    to_y: usize,
    visited: &mut im::HashSet<(usize, usize)>,
    start: usize,
    input: &[Vec<Cell>],
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if (to_x, to_y) == (start, 0) {
        return 0;
    }

    let mut longest = None;

    for (nx, ny) in neighbours(to_x, to_y) {
        let c = input[ny][nx];
        if c != Cell::Forest
            && !visited.contains(&(nx, ny))
            && (c == Cell::Path
                || (c == Cell::DownSlope && ny < to_y)
                || (c == Cell::RightSlope && nx < to_x)
                || (c == Cell::LeftSlope && nx > to_x)
                || (c == Cell::UpSlope && ny > to_y))
        {
            let mut visited = visited.clone();
            visited.insert((to_x, to_y));
            let len = match cache.get(&(nx, ny)) {
                Some(&l) => l,
                None => {
                    let v = longest_path(nx, ny, &mut visited, start, input, cache);
                    cache.insert((to_x, to_y), v);
                    v
                }
            };
            longest = match longest {
                Some(old) => Some(std::cmp::max(old, len)),
                None => Some(len),
            };
        }
    }

    longest.unwrap() + 1
}

type DryCache<'a> = ahash::HashMap<(usize, usize, Cow<'a, BitVec>), Option<usize>>;

fn longest_path_dry(
    to_x: usize,
    to_y: usize,
    visited: &mut BitVec,
    start: usize,
    input: &[Vec<Cell>],
    cache: &mut DryCache,
) -> Option<usize> {
    if (to_x, to_y) == (start, 0) {
        return Some(0);
    }

    let mut longest = None;

    for (nx, ny) in neighbours(to_x, to_y) {
        let c = input[ny][nx];
        let flat_idx = nx + ny * input[0].len();
        if c != Cell::Forest && !visited[flat_idx] {
            let mut visited = visited.clone();
            visited.set(flat_idx, true);
            let len = match cache.get(&(nx, ny, Cow::Borrowed(&visited))) {
                Some(&l) => l,
                None => {
                    let v = longest_path_dry(nx, ny, &mut visited, start, input, cache);
                    cache.insert((to_x, to_y, Cow::Owned(visited)), v);
                    v
                }
            };
            let Some(len) = len else { continue };
            longest = match longest {
                Some(old) => Some(std::cmp::max(old, len)),
                None => Some(len),
            };
        }
    }

    longest.map(|x| x + 1)
}

pub fn part1(input: Parsed) {
    let (start, end) = ends(&input);
    let mut cache = HashMap::new();

    let mut set = im::HashSet::new();
    set.insert((end, input.len() - 1));

    let longest_len = longest_path(end, input.len() - 2, &mut set, start, &input, &mut cache) + 1;

    print_res!("Longest path: {longest_len}");
}

pub fn part2(input: Parsed) {
    let (start, end) = ends(&input);
    let mut cache = ahash::HashMap::default();

    let mut set = bitvec![0; input[0].len() * input.len()];
    let flat_idx = end + (input.len() - 1) * input[0].len();
    set.set(flat_idx, true);

    let longest_len =
        longest_path_dry(end, input.len() - 2, &mut set, start, &input, &mut cache).unwrap() + 1;

    print_res!("Longest path: {longest_len}");
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

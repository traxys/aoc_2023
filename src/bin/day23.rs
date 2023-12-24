use std::{collections::HashMap, time::Instant};

use aoc_2023::{load, print_res};
use bstr::{BString, ByteSlice};
use itertools::Itertools;
use petgraph::{algo::all_simple_paths, prelude::*};

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

pub fn part1(input: Parsed) {
    let (start, end) = ends(&input);
    let mut cache = HashMap::new();

    let mut set = im::HashSet::new();
    set.insert((end, input.len() - 1));

    let longest_len = longest_path(end, input.len() - 2, &mut set, start, &input, &mut cache) + 1;

    print_res!("Longest path: {longest_len}");
}

fn intersections(input: &Parsed) -> Vec<(usize, usize)> {
    let mut points = Vec::default();

    for (y, line) in input.iter().enumerate().skip(1).take(input.len() - 2) {
        for (x, &c) in line.iter().enumerate().skip(1).take(input[0].len() - 2) {
            if c != Cell::Forest
                && neighbours(x, y)
                    .into_iter()
                    .map(|(x, y)| input[y][x])
                    .filter(|&c| c != Cell::Forest)
                    .count()
                    >= 3
            {
                points.push((x, y));
            }
        }
    }

    points
}

fn first_intersection(
    mut current: (usize, usize),
    mut from: (usize, usize),
    input: &Parsed,
    intersections: &[(usize, usize)],
) -> ((usize, usize), usize) {
    let mut len = 1;
    loop {
        if intersections.contains(&current) {
            return (current, len);
        } else {
            let ((nx, ny),) = neighbours(current.0, current.1)
                .into_iter()
                .filter(|&(x, y)| (x, y) != from && input[y][x] != Cell::Forest)
                .collect_tuple()
                .unwrap();
            from = current;
            current = (nx, ny);
            len += 1;
        }
    }
}

pub fn part2(input: Parsed) {
    let (start, end) = ends(&input);

    let intersections = intersections(&input);

    let (first, first_len) = first_intersection((start, 1), (start, 0), &input, &intersections);
    let (last, last_len) = first_intersection(
        (end, input.len() - 2),
        (end, input.len() - 1),
        &input,
        &intersections,
    );

    let neigh_idx = |neighbour| {
        intersections
            .iter()
            .enumerate()
            .find(|&(_, &c)| c == neighbour)
            .unwrap()
            .0
    };

    let mut graph = UnGraphMap::new();

    for (i, &(ix, iy)) in intersections.iter().enumerate() {
        if (ix, iy) == first || (ix, iy) == last {
            continue;
        }

        for (nx, ny) in neighbours(ix, iy)
            .into_iter()
            .filter(|&(x, y)| (x, y) != (ix, iy) && input[y][x] != Cell::Forest)
        {
            let (neighbour, dist) = first_intersection((nx, ny), (ix, iy), &input, &intersections);
            let neighbour = neigh_idx(neighbour);

            graph.add_edge(i, neighbour, dist);
        }
    }

    let mut largest_len = 0;

    for path in all_simple_paths::<Vec<_>, _>(&graph, neigh_idx(first), neigh_idx(last), 0, None) {
        let len: usize = path
            .windows(2)
            .map(|w| *graph.edge_weight(w[0], w[1]).unwrap())
            .sum();
        largest_len = std::cmp::max(largest_len, len + first_len + last_len);
    }

    print_res!("Largest path possible: {largest_len}");
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

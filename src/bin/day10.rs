use std::{collections::HashSet, time::Instant};

use aoc_2023::{load, print_res};
use bstr::{BString, ByteSlice};
use color_eyre::eyre::eyre;
use itertools::Itertools;

type Parsed<'a> = ((usize, usize), Vec<BString>);

pub fn parsing(input: &BString) -> color_eyre::Result<Parsed> {
    let grid = input.lines().map(BString::from).collect_vec();
    let (start, _) = grid
        .iter()
        .enumerate()
        .flat_map(|(y, l)| l.iter().enumerate().map(move |(x, c)| ((x, y), c)))
        .find(|(_, c)| **c == b'S')
        .ok_or_else(|| eyre!("No start position"))?;
    Ok((start, grid))
}

fn start_char((sx, sy): (usize, usize), grid: &[BString]) -> u8 {
    let top = sy != 0 && [b'|', b'7', b'F'].contains(&grid[sy - 1][sx]);
    let bot = sy != grid.len() - 1 && [b'|', b'J', b'L'].contains(&grid[sy + 1][sx]);
    let left = sx != 0 && [b'-', b'L', b'F'].contains(&grid[sy][sx - 1]);
    let right = sx != grid[0].len() - 1 && [b'-', b'J', b'7'].contains(&grid[sy][sx + 1]);

    match (top, bot, left, right) {
        (true, true, false, false) => b'|',
        (true, false, true, false) => b'J',
        (true, false, false, true) => b'L',
        (false, true, true, false) => b'7',
        (false, true, false, true) => b'F',
        (false, false, true, true) => b'-',
        _ => unreachable!("Should only have two connections"),
    }
}

fn pipe_ends((x, y): (usize, usize), grid: &[BString]) -> ((usize, usize), (usize, usize)) {
    match grid[y][x] {
        b'|' => ((x, y - 1), (x, y + 1)),
        b'-' => ((x - 1, y), (x + 1, y)),
        b'L' => ((x, y - 1), (x + 1, y)),
        b'J' => ((x, y - 1), (x - 1, y)),
        b'7' => ((x, y + 1), (x - 1, y)),
        b'F' => ((x, y + 1), (x + 1, y)),
        c => unreachable!("Invalid pipe: {}", c as char),
    }
}

fn next(prev: (usize, usize), cur: (usize, usize), grid: &[BString]) -> (usize, usize) {
    let (a, b) = pipe_ends(cur, grid);

    if a == prev {
        b
    } else {
        a
    }
}

pub fn part1((start, mut grid): Parsed) {
    let start_pipe = start_char(start, &grid);
    grid[start.1][start.0] = start_pipe;

    let mut current = pipe_ends(start, &grid).0;
    let mut prev = start;

    let mut loop_len = 1;

    while current != start {
        (prev, current) = (current, next(prev, current, &grid));

        loop_len += 1;
    }

    print_res!("Furthest loop distance: {}", loop_len / 2 + (loop_len % 2));
}

pub fn part2((start, mut grid): Parsed) {
    let start_pipe = start_char(start, &grid);
    grid[start.1][start.0] = start_pipe;

    let mut edges = Vec::new();

    let mut current = pipe_ends(start, &grid).0;
    let mut prev = start;

    let mut loop_coords = HashSet::new();
    loop_coords.insert(current);

    let current_pipe = grid[current.1][current.0];
    let mut vertex = if current_pipe == b'|' || current_pipe == b'-' {
        None
    } else {
        Some(current)
    };

    let mut first_vertex = vertex;

    while current != start {
        (prev, current) = (current, next(prev, current, &grid));

        loop_coords.insert(current);
        let pipe = grid[current.1][current.0];

        if pipe != b'|' && pipe != b'-' {
            let next_vertex = Some(current);

            if let (Some(old), Some(new)) = (vertex, next_vertex) {
                edges.push((old, new));
            }

            vertex = next_vertex;
            first_vertex = first_vertex.or(vertex);
        }
    }

    edges.push((vertex.unwrap(), first_vertex.unwrap()));

    let mut inside_points = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if loop_coords.contains(&(x, y)) {
                continue;
            }

            let any_range = |a, b| std::cmp::min(a, b)..=std::cmp::max(a, b);
            let intersection_count = edges
                .iter()
                .filter(|&&((ax, ay), (bx, by))| {
                    if ax == bx && any_range(ay, by).contains(&y) && ax > x {
                        if y == ay {
                            by > y
                        } else if y == by {
                            ay > y
                        } else {
                            true
                        }
                    } else {
                        false
                    }
                })
                .count();

            if intersection_count % 2 == 1 {
                inside_points += 1;
            }
        }
    }

    print_res!("Number of points inside: {inside_points}");
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

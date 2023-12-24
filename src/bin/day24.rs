use std::time::Instant;

use aoc_2023::{load, print_res};
use bstr::{BString, ByteSlice};
use itertools::Itertools;

#[derive(Debug)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug)]
pub struct Hailstone {
    pos: Vec3,
    vel: Vec3,
}

type Parsed = Vec<Hailstone>;

pub fn parsing(input: &BString) -> color_eyre::Result<Parsed> {
    std::str::from_utf8(input)?
        .lines()
        .map(|l| {
            let (pos, vel) = l
                .split_once(" @ ")
                .ok_or_else(|| color_eyre::eyre::eyre!("Missing line separator in: {l}"))?;

            let vec3 = |s: &str| {
                let Some((x, y, z)) = s.split(", ").map(str::trim).map(str::parse).collect_tuple()
                else {
                    color_eyre::eyre::bail!("Invalid vec3: {s}")
                };

                Ok(Vec3 {
                    x: x?,
                    y: y?,
                    z: z?,
                })
            };

            Ok(Hailstone {
                pos: vec3(pos)?,
                vel: vec3(vel)?,
            })
        })
        .collect()
}

impl Hailstone {
    fn collision_2d(&self, other: &Self) -> Option<(f64, f64)> {
        let x1 = self.pos.x;
        let y1 = self.pos.y;

        let x2 = self.pos.x + self.vel.x;
        let y2 = self.pos.y + self.vel.y;

        let x3 = other.pos.x;
        let y3 = other.pos.y;

        let x4 = other.pos.x + other.vel.x;
        let y4 = other.pos.y + other.vel.y;

        let denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

        if denom == 0 {
            return None;
        }

        let (x1, x2, x3, x4) = (x1 as i128, x2 as i128, x3 as i128, x4 as i128);
        let (y1, y2, y3, y4) = (y1 as i128, y2 as i128, y3 as i128, y4 as i128);

        let p_nom_x = (x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4);
        let p_nom_y = (x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4);

        let denom = denom as f64;
        Some((p_nom_x as f64 / denom, p_nom_y as f64 / denom))
    }

    // x(t) = x_0 + v_x_0 * t
    // x(t) - x_0 = v_x_0 * t
    // (x(t) - x_0) / v_x_0 = t
    fn time(&self, x: f64) -> f64 {
        (x - self.pos.x as f64) / self.vel.x as f64
    }
}

pub fn part1(input: Parsed) {
    let min = 200000000000000.;
    let max = 400000000000000.;

    let range = min..=max;

    let mut collision_count = 0;

    for (i, a) in input.iter().enumerate() {
        for b in &input[i + 1..] {
            if let Some((c_x, c_y)) = a.collision_2d(b) {
                if a.time(c_x) >= 0.
                    && b.time(c_x) >= 0.
                    && range.contains(&c_x)
                    && range.contains(&c_y)
                {
                    collision_count += 1;
                }
            }
        }
    }

    print_res!("Number of x,y collisions: {collision_count}");
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

use std::path::PathBuf;

use bstr::{BString, ByteSlice};
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    part: u32,
    #[arg(short, long)]
    input: PathBuf,
}

#[derive(Debug)]
pub struct Context {
    pub part: u32,
    pub input: BString,
}

#[macro_export]
macro_rules! print_res {
    ($($tt:tt)*) => {
        if (!std::env::var("AOC_BENCH").is_ok()) {
            println!($($tt)*)
        }
    };
}

#[macro_export]
macro_rules! print_res_part {
    ($($tt:tt)*) => {
        if (!std::env::var("AOC_BENCH").is_ok()) {
            print!($($tt)*)
        }
    };
}

pub fn parse_u64_bytes(b: &[u8]) -> u64 {
    b.iter()
        .map(|d| {
            if d.is_ascii_digit() {
                (d - b'0') as u64
            } else {
                panic!("Invalid digit in {}", b.as_bstr())
            }
        })
        .fold(0, |acc, d| acc * 10 + d)
}

pub fn load() -> color_eyre::Result<Context> {
    color_eyre::install()?;

    let args = Args::parse();

    let input = std::fs::read(args.input)?.into();

    Ok(Context {
        part: args.part,
        input,
    })
}

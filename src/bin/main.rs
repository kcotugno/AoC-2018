extern crate aoc;

use std::env;
use std::fs;

fn main() {
    let mut args = env::args();
    if args.len() < 3 {
        eprintln!("not enough args");
    }

    args.next();
    let challenge: u8 = args.next().unwrap().parse().unwrap();
    let part: u8 = args.next().unwrap().parse().unwrap();

    let lines: Vec<String> = fs::read_to_string(format!("input/{:02}.txt", challenge))
        .unwrap()
        .lines()
        .filter(|v| !v.trim().is_empty())
        .map(|v| v.to_string())
        .collect();

    println!(
        "{}",
        aoc::run(
            &(lines.iter().map(|s| s.as_str()).collect::<Vec<&str>>()),
            challenge,
            part
        )
    );
}

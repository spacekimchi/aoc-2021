use itertools::Itertools;
use std::collections::HashMap;
use std::io::{self, Read, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut bit_counter: HashMap<usize, i32> = HashMap::new();

    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            let count = bit_counter.entry(i).or_insert(0);
            *count += if c == '1' { 1 } else { -1 };
        }
    }

    let gamma = bit_counter.keys().sorted().map(|k| *bit_counter.get(k).unwrap() >= 0).collect_vec();
    let epsilon = gamma.iter().map(|k| !(*k)).collect_vec();

    let ans = vec_to_int(&gamma) * vec_to_int(&epsilon);

    writeln!(io::stdout(), "{}", ans)?;

    Ok(())
}

fn vec_to_int(bits: &[bool]) -> u32 {
    bits.iter().fold(0, |acc, &b| acc * 2 + (b as u32))
}


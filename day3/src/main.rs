use itertools::Itertools;
use std::collections::HashMap;
use std::io::{self, Read, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    println!("i am first line");
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    println!("here");

    part1(&input)?;
    part2(&input)?;

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

fn part2(input: &str) -> Result<()> {
    let lines = input.lines().collect_vec();
    let oo = calculate_rating(&lines, |a, b| if a.len() >= b.len() { a } else { b });
    let coo = calculate_rating(&lines, |a, b| if a.len() >= b.len() { b } else { a });
    writeln!(io::stdout(), "{}", oo * coo)?;
    Ok(())
}

fn calculate_rating<'a>(bits: &[&'a str], bit_criteria: impl Fn(Vec<&'a str>, Vec<&'a str>) -> Vec<&'a str>) -> u32 {
    let mut rem = bits.to_vec();
    let mut i = 0;
    while rem.len() > 1 {
        let (b1, b0): (Vec<&str>, Vec<&str>) = rem.iter().partition(|b| b.chars().nth(i).unwrap() == '1');
        rem = bit_criteria(b1, b0);
        i += 1;
    }

    println!("{:?}", rem);
    
    let v = vec_to_int(&rem.pop().unwrap().chars().map(|k| k == '1').collect_vec());
    println!("{}", v);
    v
}


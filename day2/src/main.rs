use std::io::{self, Read, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

struct Pos {
    horizontal: i32,
    vertical: i32,
    aim: i32,
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut sub_pos = Pos {
        horizontal: 0,
        vertical: 0,
        aim: 0,
    };

    for line in input.lines() {
        let (dir, val) = line.split_once(' ').unwrap();
        match (dir, val.parse::<i32>().unwrap()) {
            ("forward", val) => {
                sub_pos.horizontal += val;
            },
            ("up", val) => {
                sub_pos.vertical -= val;
            },
            ("down", val) => {
                sub_pos.vertical += val;
            },
            _ => unreachable!(),
        }
    }
    writeln!(io::stdout(), "{}", sub_pos.horizontal * sub_pos.vertical)?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut pos = Pos {
        vertical: 0,
        horizontal: 0,
        aim: 0,
    };

    for line in input.lines() {
        let (dir, val) = line.split_once(' ').unwrap();
        match (dir, val.parse::<i32>().unwrap()) {
            ("forward", val) => {
                pos.horizontal += val;
                pos.vertical += pos.aim * val;
            },
            ("down", val) => {
                pos.aim += val;
            },
            ("up", val) => {
                pos.aim -= val;
            },
            _ => unreachable!(),
        }
    }
    writeln!(io::stdout(), "{}", pos.horizontal * pos.vertical)?;

    Ok(())
}


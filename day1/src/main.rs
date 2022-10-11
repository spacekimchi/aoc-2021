use std::io::{self, Read, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let vec: Vec<u32> = input.lines().map(|x| x.parse().unwrap()).collect();
    counter(&vec, 2 as usize)?;
    counter(&vec, 4 as usize)?;

    Ok(())
}

fn counter(input: &[u32], window_sz: usize) -> Result<()> {
    let count = input.windows(window_sz).filter(|w| w[window_sz - 1] > w[0]).count();
    writeln!(io::stdout(), "{}", count)?;

    Ok(())
}


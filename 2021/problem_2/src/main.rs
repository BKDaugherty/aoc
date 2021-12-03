use anyhow::{anyhow, Result};
use std::convert::TryFrom;
use std::io;
use std::io::BufRead;

enum Command {
    // Should this be able to be negative?
    Forward(u32),
    // Should these be the same variant?
    Down(u32),
    Up(u32),
}

impl TryFrom<String> for Command {
    type Error = anyhow::Error;
    fn try_from(string: String) -> Result<Command> {
        // Split string by spaces?
        let split: Vec<&str> = string.split_whitespace().collect();

        let integral = split
            .get(1)
            .ok_or(anyhow!("Must have integral"))?
            .parse::<u32>()?;

        match *(split
            .get(0)
            .ok_or(anyhow!("Must have cardinal direction"))?)
        {
            "forward" => Ok(Command::Forward(integral)),
            "down" => Ok(Command::Down(integral)),
            "up" => Ok(Command::Up(integral)),
            cmd => Err(anyhow!("unknown cmd variant {}", cmd)),
        }
    }
}

fn main() -> Result<()> {
    let stdin = io::stdin();
    let sequence: Result<Vec<Command>> = stdin
        .lock()
        .lines()
        .map(|l| Command::try_from(l.unwrap()))
        .collect();

    let mut aim = 0;
    let mut x = 0;
    let mut depth = 0;

    let sequence = sequence?;

    for command in sequence {
        match command {
            Command::Forward(dist) => {
                x += dist;
                depth += aim * dist
            }
            Command::Down(dist) => {
                aim += dist;
            }
            Command::Up(dist) => {
                aim -= dist;
            }
        }
    }
    println!("final position: {},{}", x, depth);
    println!("Product: {}", x * depth);
    Ok(())
}

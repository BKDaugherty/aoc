use anyhow::Result;
use std::io;
use std::io::BufRead;
use structopt::{clap::arg_enum, StructOpt};

fn calculate_increasing(sequence: &Vec<u32>) -> usize {
    let mut increasing = 0;
    for i in 1..(sequence.len()) {
        if sequence[i - 1] < sequence[i] {
            increasing += 1;
        }
    }
    increasing
}

fn calculate_windows(sequence: &Vec<u32>) -> usize {
    let mut increasing = 0;
    for i in 3..(sequence.len()) {
        let window_1 = sequence[i - 3] + sequence[i - 2] + sequence[i - 1];
        let window_2 = sequence[i] + sequence[i - 1] + sequence[i - 2];

        if window_1 < window_2 {
            increasing += 1;
        }
    }
    increasing
}

arg_enum! {
    #[derive(Debug)]
    enum Problem {
    Sequence,
    Windows
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "Problem1", about = "Solving AOC problem 1.")]
struct Args {
    #[structopt(long)]
    problem_type: Problem,
}

fn main() {
    let args = Args::from_args();
    let stdin = io::stdin();
    let sequence: Vec<u32> = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap().parse::<u32>().unwrap())
        .collect();

    match args.problem_type {
        Problem::Sequence => {
            let increasing = calculate_increasing(&sequence);
            println!("Increasing count: {}", increasing);
        }
        Problem::Windows => {
            let increasing = calculate_windows(&sequence);
            println!("Increasing count for windows: {}", increasing);
        }
    }
}

use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp;
use std::collections::HashMap;
use std::io;
use std::io::BufRead;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Line {
    a: Point,
    b: Point,
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.a.y == self.b.y
    }
    fn is_vertical(&self) -> bool {
        self.a.x == self.b.x
    }
    fn parse_from_input(line: String) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new("\\d+").expect("Create regex");
        }
        let mut numbers = Vec::new();
        for mat in RE.captures_iter(&line) {
            for cap in mat.iter() {
                if let Some(number) = cap {
                    numbers.push(number.as_str().parse::<usize>().expect("Should parse"));
                }
            }
        }
        match numbers.len() {
            4 => Ok(Line {
                a: Point {
                    x: numbers[0],
                    y: numbers[1],
                },
                b: Point {
                    x: numbers[2],
                    y: numbers[3],
                },
            }),
            other => Err(anyhow!("invalid amount of digits found {}", other)),
        }
    }
}

fn main() -> Result<()> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let lines = handle.lines();
    let lines = lines
        .map(|line| Line::parse_from_input(line?))
        .collect::<Result<Vec<Line>>>();
    let lines = lines?;

    let mut grid = HashMap::new();
    for line in &lines {
        if line.is_horizontal() {
            for x in std::cmp::min(line.a.x, line.b.x)..std::cmp::max(line.a.x, line.b.x) + 1 {
                let point = Point { x, y: line.a.y };
                *grid.entry(point).or_insert(0) += 1;
            }
        } else if line.is_vertical() {
            for y in std::cmp::min(line.a.y, line.b.y)..std::cmp::max(line.a.y, line.b.y) + 1 {
                let point = Point { x: line.a.x, y };
                *grid.entry(point).or_insert(0) += 1;
            }
        } else {
            let xs = if line.a.x <= line.b.x {
                (line.a.x..line.b.x + 1).collect::<Vec<usize>>()
            } else {
                (line.b.x..line.a.x + 1).rev().collect::<Vec<usize>>()
            };

            let ys = if line.a.y <= line.b.y {
                (line.a.y..line.b.y + 1).collect::<Vec<usize>>()
            } else {
                (line.b.y..line.a.y + 1).rev().collect::<Vec<usize>>()
            };

            for i in 0..xs.len() {
                let point = Point { x: xs[i], y: ys[i] };
                *grid.entry(point).or_insert(0) += 1;
            }
        }
    }
    let mut num_points = 0;
    for (_, count) in grid {
        if count >= 2 {
            num_points += 1;
        }
    }

    println!("Num points with at least 2: {}", num_points);
    Ok(())
}

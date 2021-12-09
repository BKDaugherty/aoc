use anyhow::{anyhow, Result};
use maplit::{hashmap, hashset};
use std::collections::{HashMap, HashSet};
use std::io;
use std::io::BufRead;

const X_SIZE: isize = 100;
const Y_SIZE: isize = 100;

fn read_input() -> Result<Grid> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let lines = handle.lines();
    let mut heatmap = Vec::new();
    for line in lines {
        let line = line.expect("Line should exist");
        for c in line.chars() {
            heatmap.push(c.to_digit(10).expect("Should be digit") as usize);
        }
    }
    Ok(Grid { storage: heatmap })
}

struct Grid {
    storage: Vec<usize>,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Point {
    x: isize,
    y: isize,
    value: usize,
}

impl Grid {
    fn access(&self, x: isize, y: isize) -> Option<usize> {
        if x >= X_SIZE || y >= Y_SIZE || x < 0 || y < 0 {
            None
        } else {
            Some(self.storage[(y * X_SIZE + x) as usize])
        }
    }
    fn get_point(&self, x: isize, y: isize) -> Option<Point> {
        match self.access(x, y) {
            Some(value) => Some(Point { x, y, value }),
            None => None,
        }
    }
}

fn main() -> Result<()> {
    let grid = read_input()?;
    let mut low_points = Vec::new();
    for x in 0..X_SIZE {
        for y in 0..Y_SIZE {
            let point = grid.access(x, y).expect("Should exist");
            let mut is_lowpoint = true;
            for neighbor in &[-1, 1] {
                if let Some(value) = grid.access(x + neighbor, y) {
                    if value <= point {
                        is_lowpoint = false;
                    }
                }
                if let Some(value) = grid.access(x, y + neighbor) {
                    if value <= point {
                        is_lowpoint = false;
                    }
                }
            }
            if is_lowpoint {
                low_points.push(Point { x, y, value: point });
            }
        }
    }
    let num_low_points = low_points.len();

    let total_risk = low_points.iter().map(|x| x.value).sum::<usize>() + num_low_points;
    println!("Total Risk: {}", total_risk);

    let mut basin_sizes = Vec::new();
    for low_point in low_points {
        let size = explore_basin(&grid, low_point.clone());
        basin_sizes.push(size);
    }

    basin_sizes.sort();
    let mut rs = basin_sizes.iter().rev();

    let product = rs.next().unwrap() * rs.next().unwrap() * rs.next().unwrap();

    println!("Basin Product: {}", product);

    Ok(())
}

fn explore_basin(grid: &Grid, origin: Point) -> usize {
    let mut size = 0;
    let mut stack = Vec::new();
    let mut explored = HashSet::new();
    stack.push(origin);
    loop {
        match stack.pop() {
            Some(point) => {
                if explored.contains(&point) {
                    continue;
                } else {
                    explored.insert(point.clone());
                }
                size += 1;
                for inc in &[-1, 1] {
                    if let Some(neighbor) = grid.get_point(point.x + inc, point.y) {
                        if neighbor.value != 9 {
                            stack.push(neighbor)
                        }
                    }

                    if let Some(neighbor) = grid.get_point(point.x, point.y + inc) {
                        if neighbor.value != 9 {
                            stack.push(neighbor)
                        }
                    }
                }
            }
            None => {
                break;
            }
        };
    }
    size
}

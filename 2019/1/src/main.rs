use std::io;
use std::io::BufRead;
use anyhow::{Result};
use std::time::{Instant};
fn get_fuel_required(module_weight: i32) -> i32 {
    // Rounds down on int division
    (module_weight / 3) - 2
}

fn get_fuel_required_rec(module_weight: i32) -> i32 {
    let fuel = get_fuel_required(module_weight);
    if fuel > 0 {
	fuel + get_fuel_required_rec(fuel)
    } else {
	0
    }
}

fn main() -> Result<()> {
    let now = Instant::now();
    let stdin = io::stdin();
    let required_fuel : i32 = stdin.lock()
	.lines()
	.map(| l : Result<String, io::Error> |
	     get_fuel_required_rec(l.unwrap()
			       .parse::<i32>()
			       .unwrap()))
	.sum();
    println!("You need {} fuel, which took us {}ns", required_fuel, now.elapsed().as_nanos());
    Ok(())
}

#[test]
fn test_rec_fuel() -> Result<()> {
    assert_eq!(get_fuel_required_rec(20), 4);
    Ok(())
}

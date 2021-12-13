use anyhow::Result;
use env_logger::Env;
use grid::Grid;
use log::info;
use std::collections::HashSet;

#[derive(Debug, Hash, PartialEq, Eq)]
struct Index {
    x: isize,
    y: isize,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum SimulationGoal {
    TotalFlash,
    SimultaneousSearch,
}

fn simulate(steps: usize, mut grid: Grid, goal: SimulationGoal) -> usize {
    let mut flash_count = 0;
    for step in 0..steps {
        let mut to_flash = Vec::new();
        for x in 0..grid.length {
            for y in 0..grid.height {
                // Increase octopus
                *grid.access_mut(x as isize, y as isize).unwrap() += 1;
                if grid.access(x as isize, y as isize).unwrap() > 9 {
                    to_flash.push((x as isize, y as isize));
                }
            }
        }
        let mut flashed = HashSet::new();
        // Process flashes
        while let Some((x, y)) = to_flash.pop() {
            let start = Index { x, y };
            if flashed.contains(&start) {
                continue;
            }
            flashed.insert(start);
            flash_count += 1;
            for x_dif in -1..2 {
                for y_dif in -1..2 {
                    let index = Index {
                        x: x + x_dif,
                        y: y + y_dif,
                    };
                    if !flashed.contains(&index) {
                        if let Some(value) = grid.access_mut(x + x_dif, y + y_dif) {
                            *value += 1;
                            if *value > 9 {
                                to_flash.push((x + x_dif, y + y_dif))
                            }
                        }
                    }
                }
            }
        }
	let flashed_this_step = flashed.len();
        flashed.into_iter().for_each(|index| {
            *grid.access_mut(index.x, index.y).unwrap() = 0;
        });
        if let (SimulationGoal::SimultaneousSearch, 100) = (goal, flashed_this_step) {
            return step;
        } 
    }
    flash_count
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let test_grid = Grid::from_vec_str(
        5,
        5,
        vec!["11111", "19991", "19191", "19991", "11111"]
            .iter()
            .map(|x| x.to_string())
            .collect(),
    )?;
    let test_flash_count = simulate(2, test_grid, SimulationGoal::TotalFlash);
    assert_eq!(test_flash_count, 9);

    let test_grid_2 = Grid::from_vec_str(
        10,
        10,
        vec![
            "5483143223",
            "2745854711",
            "5264556173",
            "6141336146",
            "6357385478",
            "4167524645",
            "2176841721",
            "6882881134",
            "4846848554",
            "5283751526",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect(),
    )?;

    let test_sim_step = simulate(200, test_grid_2, SimulationGoal::SimultaneousSearch);
    assert_eq!(test_sim_step + 1, 195);

    let mut grid = Grid::from_stdin(10, 10)?;
    // let flash_count = simulate(100, grid, SimulationGoal::TotalFlash);

    let sim_step = simulate(2000, grid, SimulationGoal::SimultaneousSearch);

    println!("sim_step: {}", sim_step + 1);
    Ok(())
}

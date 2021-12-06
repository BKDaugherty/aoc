use anyhow::{anyhow, Result};
use std::collections::HashMap;

fn state_builder(seed: Vec<usize>) -> HashMap<usize, usize> {
    let mut state = HashMap::new();
    for value in seed {
        *state.entry(value).or_insert(0) += 1;
    }
    state
}

fn advance(state: HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut next_state = HashMap::new();
    for (stage, count) in state {
        match stage {
            0 => {
                *next_state.entry(6).or_insert(0) += count;
                *next_state.entry(8).or_insert(0) += count;
            }
            x => {
                *next_state.entry((x - 1)).or_insert(0) += count;
            }
        }
    }
    next_state
}

fn main() -> Result<()> {
    let seed = vec![
        5, 1, 1, 4, 1, 1, 4, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 4, 2, 1, 1, 1, 3, 5, 1, 1, 1, 5, 4,
        1, 1, 1, 2, 2, 1, 1, 1, 2, 1, 1, 1, 2, 5, 2, 1, 2, 2, 3, 1, 1, 1, 1, 1, 1, 1, 1, 5, 1, 1,
        4, 1, 1, 1, 5, 4, 1, 1, 3, 3, 2, 1, 1, 1, 5, 1, 1, 4, 1, 1, 5, 1, 1, 5, 1, 2, 3, 1, 5, 1,
        3, 2, 1, 3, 1, 1, 4, 1, 1, 1, 1, 2, 1, 2, 1, 1, 2, 1, 1, 1, 4, 4, 1, 5, 1, 1, 3, 5, 1, 1,
        5, 1, 4, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 3, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 2, 1, 1,
        1, 5, 1, 1, 1, 1, 4, 1, 1, 1, 1, 4, 1, 1, 1, 1, 3, 1, 2, 1, 2, 1, 3, 1, 3, 4, 1, 1, 1, 1,
        1, 1, 1, 5, 1, 1, 1, 1, 1, 1, 1, 1, 4, 1, 1, 2, 2, 1, 2, 4, 1, 1, 3, 1, 1, 1, 5, 1, 3, 1,
        1, 1, 5, 5, 1, 1, 1, 1, 2, 3, 4, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 5, 1, 4, 3, 1, 1, 1,
        2, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 3, 1, 2, 2, 1, 4, 1, 5,
        1, 5, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 5, 1, 1, 1, 4, 3, 1, 1, 4,
    ];

    let mut state = state_builder(seed);
    for _ in 0..256 {
        state = advance(state)
    }
    let total = count(&state);

    println!("num fish {}", total);
    Ok(())
}

fn count(state: &HashMap<usize, usize>) -> usize {
    let mut total = 0;
    for (stage, count) in state {
        total += count;
    }
    total
}

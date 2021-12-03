use anyhow::{anyhow, Result};
use std::collections::HashSet;
use std::io;
use std::io::BufRead;

#[derive(Clone, Debug)]
enum Common {
    Equal,
    NotEqual(String),
}

#[derive(Clone, Copy, Debug)]
enum RatingType {
    Oxygen,
    CO2,
}

#[derive(Clone)]
enum RatingResult {
    // Returns the rating
    Found(usize),
    // Returns the index we were searching on so we can begin again
    NotFound(usize),
}

fn main() -> Result<()> {
    let stdin = io::stdin();
    let sequence = stdin.lock().lines();

    // Build table
    let mut columns = Vec::new();
    for line in sequence {
        let line = line?;
        let line = line.as_bytes();
        for i in 0..line.len() {
            if columns.get(i) == None {
                columns.push(Vec::new());
            }
            columns[i].push(
                char::from(line[i])
                    .to_digit(2)
                    .expect("Should be able to go to digit"),
            );
        }
    }

    let mut gamma_factor_string = String::new();
    for column in &columns {
        let to_add = match most_common(&column) {
            Common::Equal => "1".to_string(),
            Common::NotEqual(value) => value,
        };
        gamma_factor_string.push_str(&to_add);
    }
    let epsilon_rate_string = logical_not(&gamma_factor_string)?;

    let gamma_factor = usize::from_str_radix(&gamma_factor_string, 2)?;
    let epsilon_rate = usize::from_str_radix(&epsilon_rate_string, 2)?;
    println!("Gamma: {}, Epsilon: {}", gamma_factor, epsilon_rate);
    println!("Product: {}", gamma_factor * epsilon_rate);

    let og = find_rating(&columns, 0, RatingType::Oxygen)?;
    let co2 = find_rating(&columns, 0, RatingType::CO2)?;
    println!("OG: {}, C02: {}", og, co2);
    println!("Life Support Rating: {}", og * co2);

    Ok(())
}

fn grab_binary_value(columns: &Vec<Vec<u32>>, index: usize) -> Result<usize> {
    let mut binary_string = String::new();
    for column in columns {
        binary_string.push_str(&column[index].to_string())
    }
    Ok(usize::from_str_radix(&binary_string, 2)?)
}

fn search_column(column: &Vec<u32>, to_match: u32) -> HashSet<usize> {
    let mut new_set = HashSet::new();
    for (index, entry) in column.iter().enumerate() {
        if *entry == to_match {
            new_set.insert(index);
        }
    }
    new_set
}

fn most_common(column: &Vec<u32>) -> Common {
    let ones = column.into_iter().sum::<u32>();
    let zeroes = column.len() as u32 - ones;
    if ones > zeroes {
        Common::NotEqual("1".to_string())
    } else if ones < zeroes {
        Common::NotEqual("0".to_string())
    } else {
        Common::Equal
    }
}

fn logical_not(b_str: &String) -> Result<String> {
    let mut new_string = String::new();
    for c in b_str.chars() {
        let c2 = match c {
            '0' => '1',
            '1' => '0',
            _ => {
                return Err(anyhow!("Unknown char found"));
            }
        };
        new_string.push(c2);
    }
    Ok(new_string)
}

fn find_rating(
    columns: &Vec<Vec<u32>>,
    starting_index: usize,
    rating_type: RatingType,
) -> Result<usize> {
    let result = find_rating_aux(columns, starting_index, rating_type)?;

    match result {
        RatingResult::Found(result) => Ok(result),
        RatingResult::NotFound(index) => find_rating(columns, index + 1, rating_type),
    }
}

fn find_rating_aux(
    columns: &Vec<Vec<u32>>,
    string_index: usize,
    rating_type: RatingType,
) -> Result<RatingResult> {
    let column = &columns[string_index];

    let most_common = most_common(column);
    let to_match = match (most_common, rating_type) {
        (Common::Equal, RatingType::Oxygen) => "1".to_string(),
        (Common::Equal, RatingType::CO2) => "0".to_string(),
        (Common::NotEqual(value), RatingType::Oxygen) => value,
        (Common::NotEqual(value), RatingType::CO2) => logical_not(&value)?,
    }
    .parse::<u32>()
    .expect("Should be a number");

    let valid_indices = search_column(column, to_match);
    // Stop searching if the valid_indices only has one left
    if valid_indices.len() == 1 {
        let index = valid_indices.iter().next().expect("Must have value");
        let binary_value = grab_binary_value(columns, *index)?;
        return Ok(RatingResult::Found(binary_value));
    } else if valid_indices.len() == 0 {
        return Ok(RatingResult::NotFound(string_index));
    } else {
        // Reconstruct columns, and make recursive call
        let mut new_columns = Vec::new();
        for (column_index, column) in columns.iter().enumerate() {
            new_columns.push(Vec::new());
            for index in &valid_indices {
                new_columns[column_index].push(column[*index])
            }
        }
        find_rating_aux(&new_columns, string_index + 1, rating_type)
    }
}

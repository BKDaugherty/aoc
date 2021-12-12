use anyhow::Result;
use std::io;
use std::io::BufRead;

fn score(failing: Vec<char>) -> usize {
    let mut total = 0;
    for c in failing {
        total += match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => {
                panic!("Oh no!");
            }
        };
    }
    total
}

enum LineQuality {
    Failing(char),
    Incomplete(usize),
}

fn process_line(line: String) -> LineQuality {
    let mut processing_stack = Vec::new();
    for c in line.chars() {
        match c {
            ']' | '>' | ')' | '}' => {
                let last_opening_character = processing_stack.pop();
                match last_opening_character {
                    Some(value) => match (value, c) {
                        ('(', ')') | ('[', ']') | ('<', '>') | ('{', '}') => {}
                        _ => {
                            return LineQuality::Failing(c);
                        }
                    },
                    None => {
                        return LineQuality::Failing(c);
                    }
                }
            }
            _ => {
                processing_stack.push(c);
            }
        }
    }

    let mut total = 0;
    loop {
        let opener = processing_stack.pop();
        if let Some(opener) = opener {
            total = total * 5
                + match opener {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
		    _ => panic!("Oh no!")
                };
        } else {
            break;
        }
    }
    LineQuality::Incomplete(total)
}

fn main() -> Result<()> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let lines = handle.lines();

    let mut incomplete_scores= Vec::new();
    let mut failing_characters = Vec::new();
    for line in lines {
        let line = line?;
        match process_line(line.clone()) {
            LineQuality::Failing(failing) => {
                failing_characters.push(failing);
            }
            LineQuality::Incomplete(score) => {
                incomplete_scores.push(score);
            }
        }
    }

    incomplete_scores.sort();

    println!(
        "Middle Score: {}",
        incomplete_scores[incomplete_scores.len() / 2]
    );
    let total = score(failing_characters);
    println!("Total Failing : {}", total);

    Ok(())
}

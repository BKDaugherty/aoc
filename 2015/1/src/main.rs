use std::time::{Instant};
use std::io::{self, Read};
use anyhow::{anyhow, Result};



fn main() -> Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let now = Instant::now();
    let result = naive_count(&buffer);
    println!("Answer: {}, Took: {}ns", result, now.elapsed().as_nanos());
    
    let now = Instant::now();
    let result = naive_pos(&buffer)?;
    println!("Answer_Pos: {}, Took:{}ns", result, now.elapsed().as_nanos());
    Ok(())
}

fn naive_count(parens : &str) -> i32 {
    let mut count = 0;
    for p in parens.chars() {
	count += match p {
	    '(' => 1,
	    ')' => -1,
	    _ => 0
	}
    }
    return count;
}

fn naive_pos(parens : &str) -> Result<u32> {
    let mut pos = 1;
    let mut count = 0;
    for p in parens.chars() {
	count += match p {
	    '(' => 1,
	    ')' => -1,
	    _ => 0
	};
	if count == -1 {
	    return Ok(pos);
	}
	pos += 1;
    }
    Err(anyhow!("Never visited basement"))
}

use std::io;
use std::io::BufRead;
use anyhow::{Result, anyhow};
use std::time::Instant;

fn main() -> Result<()> {

    let stdin = io::stdin();
    let mut buffer = String::new();
    let mut handle = stdin.lock();
    handle.read_line(&mut buffer)?;

    let program : Vec<usize> = buffer.split(",")
	.flat_map(|l| l.parse::<usize>()).collect();
    let now = Instant::now();

    match naive_solve(&program) {
	Ok((noun, verb)) => {println!("Answer: {} {} {}", noun, verb, (noun * 100 + verb)); Ok(()) },
	Err(e) => Err(e)
    }
}

fn naive_solve(program : &Vec<usize>) -> Result<(usize, usize)> {
    for noun in 0..100 {
	for verb in 0..100 {
	    match run_program(program, noun, verb) {
		Ok(result) => {
		    if result == 19690720 {
			return Ok((noun, verb))
		    }
		},
		_ => {}
	    }
	}
    }
    return Err(anyhow!("Couldn't find solution"));
}

fn run_program(program : &Vec<usize>, noun: usize, verb: usize) -> Result<usize> {
    let mut attempt = program.clone();
    attempt[1] = noun;
    attempt[2] = verb;
    evaluate_program(attempt)
}

#[derive(Debug)]
struct Instruction {
    addr_arg1: usize,
    addr_arg2: usize,
    addr_dest: usize,
    opcode: usize
}

#[derive(Debug)]
struct ProgramMod {
    to_store: usize,
    destination: usize,
}

fn process_instruction(i : &Instruction, program : &Vec<usize> ) -> Result<ProgramMod> {
    let l = program.len();
    if i.addr_arg1 < l && i.addr_arg2 < l && i.addr_dest < l {
	let destination = i.addr_dest;
	let op1 = program[i.addr_arg1];
	let op2 = program[i.addr_arg2];
	let to_store = match i.opcode {
	    1 => Ok(op1 + op2),
	    2 => Ok(op1 * op2),
	    _ => { Err(anyhow!("Invalid OPCODE")) },
	}?;
	// println!("Creating Program Mod {:?}", ProgramMod{to_store, destination});
	Ok(ProgramMod{to_store, destination})
    } else {
	Err(anyhow!("Out of bounds -- SEGFAULT"))
    }
}


fn evaluate_program(mut program : Vec<usize>) -> Result<usize> {
    let mut idx = 0;
    while idx < program.len() {
	let opcode = program[idx];
	match opcode {
	    1 | 2 => {
		// Check if idx can go over...
		if idx + 3 > program.len() {
		    return Err(anyhow!("Out of bounds!"));
		}
		let i = Instruction {
		    opcode,
		    addr_arg1: program[idx + 1],
		    addr_arg2: program[idx + 2],
		    addr_dest: program[idx + 3],
		};
		// println!("Applying Instruction {:?}", i);
		let program_mod = process_instruction(&i, &program)?;
		program[program_mod.destination] = program_mod.to_store;
	    },
	    99 => {
		return Ok(program[0]);
	    },
	    invalid_opcode => {return Err(anyhow!("Invalid opcode {}", invalid_opcode)) },
	};
	idx += 4;
    }
    return Err(anyhow!("Invalid Program... No Exit code!"));
}

#[test]
fn check_example_programs() {
    assert_eq!(evaluate_program(vec!(1,0,0,0,99)).unwrap(), 2);
    assert_eq!(evaluate_program(vec!(2,3,0,3,99)).unwrap(), 2);
    assert_eq!(evaluate_program(vec!(1,1,1,4,99,5,6,0,99)).unwrap(), 30);
}

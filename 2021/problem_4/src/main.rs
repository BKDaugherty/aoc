use anyhow::{anyhow, Result};
use std::collections::{HashMap, HashSet};
use std::io;
use std::io::BufRead;

struct Board {
    height: usize,
    length: usize,
    number_to_position: HashMap<usize, usize>,
    // This could just be a set?
    positions_marked: HashSet<usize>,
}

impl Board {
    fn new(height: usize, length: usize, board_data: Vec<Vec<usize>>) -> Self {
        let mut number_to_position = HashMap::new();
        for (row_idx, row) in board_data.iter().enumerate() {
            for (idx, value) in row.iter().enumerate() {
                // This will handle duplicates quite poorly
                number_to_position.insert(*value, idx + (row_idx * length));
            }
        }
        Self {
            height,
            length,
            number_to_position,
            positions_marked: HashSet::new(),
        }
    }
    fn mark_board(&mut self, number: usize) {
        match self.number_to_position.get(&number) {
            Some(position) => {
                self.positions_marked.insert(*position);
            }
            None => {}
        }
    }

    fn check_win(&self, number: usize) -> bool {
        match self.number_to_position.get(&number) {
            Some(position) => self.check_board_complete(*position),
            None => false,
        }
    }

    // Assumes all rows are equal length and all columns are equal length
    fn check_board_complete(&self, position: usize) -> bool {
        let row_start = position - (position % self.length);
        let column_start = position % self.length;

        let column_indices: Vec<usize> = (column_start..)
            .step_by(self.length)
            .take(self.height)
            .collect();
        let row_indices: Vec<usize> = (row_start..).take(self.length).collect();
        if self.check_group_marked(&column_indices) || self.check_group_marked(&row_indices) {
            true
        } else {
            false
        }
    }

    fn check_group_marked(&self, group: &Vec<usize>) -> bool {
        for index in group {
            match self.positions_marked.contains(index) {
                false => {
                    return false;
                }
                true => {}
            }
        }
        true
    }

    fn get_board_score(&self) -> usize {
        let mut sum = 0;
        for (number, position) in &self.number_to_position {
            if !self.positions_marked.contains(position) {
                sum += number;
            }
        }
        sum
    }
}

fn play_bingo(sequence: Vec<usize>, mut boards: Vec<Board>) -> Result<()> {
    let mut boards_pending = (0..boards.len()).collect::<HashSet<usize>>();
    for value in sequence {
        let mut to_remove = HashSet::new();
        for pending_board_idx in &boards_pending {
            let mut board = boards
                .get_mut(*pending_board_idx)
                .expect("Board should exist");
            board.mark_board(value);
            if board.check_win(value) {
                to_remove.insert(*pending_board_idx);
                let score = board.get_board_score() * value;
                println!("Board {} wins, score {}", pending_board_idx, score);
            }
        }
        boards_pending.retain(|x| !to_remove.contains(x));
    }
    Ok(())
}

fn main() -> Result<()> {
    let stdin = io::stdin();
    let mut sequence_buffer = String::new();
    let mut handle = stdin.lock();

    handle.read_line(&mut sequence_buffer)?;

    let sequence = sequence_buffer
        .trim()
        .split(',')
        .map(|value| value.parse::<usize>().expect("Should be an integral"))
        .collect::<Vec<usize>>();

    // Skip the first newline also
    handle.read_line(&mut sequence_buffer)?;

    let lines = handle.lines();
    let mut board_buffer = Vec::new();
    let mut boards = Vec::new();
    for line in lines {
        let line = line?;
        if &line == "" {
            boards.push(Board::new(5, 5, board_buffer));
            board_buffer = Vec::new();
        } else {
            board_buffer.push(
                line.split_whitespace()
                    .map(|value| value.parse::<usize>().expect("Should be an integral"))
                    .collect::<Vec<usize>>(),
            );
        }
    }

    play_bingo(sequence, boards)
}

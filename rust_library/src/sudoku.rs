use crate::types::*;
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq)]
pub enum PuzzleEvaluation {
    Incomplete,
    Wrong,
    Solved
}

pub fn contains_duplicates(list: [u8; 9]) -> bool {
    let no_blanks = list.iter().filter(|x| **x != 0);
    let set: HashSet<&u8> = no_blanks.clone().collect();
    return set.iter().count() != no_blanks.count();
}

/// TODO: report failed rule
pub fn check_rules(puzzle: Puzzle) -> bool {
    for row_index in 0..9 {
        let row = puzzle[row_index];
        if contains_duplicates(row) {
            //println!("row_index: {}", row_index);
            //println!("Failed row");
            return false
        }
    }

    for column_index in 0..9 {
        let mut column = [0; 9];
        for row_index in 0..9 {
            column[row_index] = puzzle[row_index][column_index]
        }
        if contains_duplicates(column) {
            //println!("Failed column");
            //println!("column_index: {}", column_index);
            return false
        }
    }

    for box_x_index in 0..3 {
        for box_y_index in 0..3 {
            let mut boxx = [0; 9];
            for inner_x in 0..3 {
                for inner_y in 0..3 {
                    let x = box_x_index * 3 + inner_x;
                    let y = box_y_index * 3 + inner_y;
                    boxx[inner_x + 3 * inner_y] = puzzle[x][y]
                }
            }
            if contains_duplicates(boxx) {
                //println!("Failed box");
                return false
            }
        }
    }

    return true;
}

pub fn filled_out(puzzle: Puzzle) -> bool {
    for row in puzzle {
        for column in row {
            if column == 0 {
                return false
            }
        }
    }
    return true;
}

pub fn evaluate_puzzle(puzzle: Puzzle) -> PuzzleEvaluation {
    if !check_rules(puzzle) {
        return PuzzleEvaluation::Wrong;
    }
    else if !filled_out(puzzle) {
        return PuzzleEvaluation::Incomplete; 
    }
    else {
        return PuzzleEvaluation::Solved;
    }
}
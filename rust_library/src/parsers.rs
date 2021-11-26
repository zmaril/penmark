use pest::Parser; 

use crate::types::*;

#[derive(Parser)]
#[grammar = "parsers/sudoku.pest"] 
pub struct SudokuParser;

pub fn parse_sudoku(input: &str) -> Puzzle {
    let r = crate::Rule{};
    let rules = vec![r];
    let layout = Grid{};
    let givens = vec![Cell{}];
    let puzzle = Puzzle{rules, layout, givens };
    return puzzle;
}
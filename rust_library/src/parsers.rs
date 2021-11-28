use pest::Parser; 

use crate::types::*;

#[derive(Parser)]
#[grammar = "parsers/sudoku.pest"] 
pub struct SudokuParser;

pub fn parse_sudoku(input: &str) -> Puzzle {
    return 0;
}
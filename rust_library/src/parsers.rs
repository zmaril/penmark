use pest::Parser; 

use crate::types::*;

#[derive(Parser)]
#[grammar = "parsers/sudoku.pest"] 
pub struct SudokuParser;

pub fn parse_sudoku(input: &str) -> Puzzle {
    let successful_parse = SudokuParser::parse(Rule::sudoku, input);
    println!("{:?}", successful_parse);

    let mut grid = [[0u8; 9]; 9];
    println!("{:?}", grid);

    let lines = successful_parse.expect("unsuccessful parse").next().unwrap();
    let mut row = 0;
    for line in lines.into_inner() {
        let mut column = 0; 
        for cell in line.into_inner(){
            let contents = match cell.as_rule() {
                Rule::digit => cell.as_str().parse::<u8>().unwrap(),
                Rule::open => 0,
                _ => 0 //unreachable!()
            };
            grid[row][column] = contents;
            column +=1;
        }
        row +=1;
    } 

    // let r = crate::Rule{};
    // let rules = vec![r];
    // let layout = Grid{};
    // let givens = vec![Cell{}];
    // let puzzle = Puzzle{rules, layout, givens };
    return grid;
}

pub mod types { 

use std::collections::HashSet;
pub struct Puzzle {
    layout: Grid,
    givens: Vec<Cell>,
    rules: Vec<Rule>
}

pub struct Rule {

}

pub struct Cell {
    coordinates: Coordinate,
    value: CellValue 
}

pub struct Pair {
    values: (u8, u8)
}

pub struct Triple {
    values: (u8, u8, u8)
}

pub struct Candidates {
    values: HashSet<u8>
}

enum CellValue {
    u8, 
    Pair,
    Triple,
    Candidates 
}

pub struct Coordinate {
    x: u8, 
    y: u8
}

pub struct Grid {

}

pub struct Pattern {

}

pub struct Solution  {
}
}
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq)]
pub struct Puzzle {
    pub layout: Grid,
    pub givens: Vec<Cell>,
    pub rules: Vec<Rule>
}

#[derive(Debug, Eq, PartialEq)]
pub struct Rule {

}

#[derive(Debug, Eq, PartialEq)]
pub struct Cell {
     //coordinates: Coordinate,
     //value: CellValue 
}

// pub struct Pair {
//     values: (u8, u8)
// }

// pub struct Triple {
//     values: (u8, u8, u8)
// }

// pub struct Candidates {
//     values: HashSet<u8>
// }

// pub enum CellValue {
//     u8, 
//     Pair,
//     Triple,
//     Candidates 
// }

// pub struct Coordinate {
//     x: u8, 
//     y: u8
// }

#[derive(Debug, Eq, PartialEq)]
pub struct Grid {

}

// pub struct Pattern {

// }

// pub struct Solution  {
// }
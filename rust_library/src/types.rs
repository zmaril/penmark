#[derive(Debug, Eq, PartialEq)]
pub struct PuzzleO {
    pub layout: Grid,
    pub givens: Vec<Cell>,
    pub rules: Vec<Rule>
}

pub type Puzzle = [[u8; 9]; 9];

//TODO make it display/debug better
// impl fmt::Display for Puzzle {
//     // This trait requires `fmt` with this exact signature.
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // Write strictly the first element into the supplied output
//         // stream: `f`. Returns `fmt::Result` which indicates whether the
//         // operation succeeded or failed. Note that `write!` uses syntax which
//         // is very similar to `println!`.
//         for x in 0..9 {
//             for y in 0..9 {
//                 write!(f, "{}", self[x][y])
//             }
//             write!(f,"\n")
//         }
//     }
// }


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
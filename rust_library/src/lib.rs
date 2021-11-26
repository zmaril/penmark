#![feature(proc_macro_hygiene, decl_macro)]
extern crate pest;
#[macro_use]
extern crate pest_derive;

mod types; 
mod parsers;

pub use self::types::*;
pub use self::parsers::*;
/* 

Solution

Cell 
.coordinate <Coordinate Type> 
.value

Candidates 
Pairs 



Interfaces
Grid
- Layout 
- Values
* mapping from coordinates to values


VisualGrid
*/
/*
Coordinate 
- CartesianCoordinate 
- HexCoordinate 

Implementations 
- CartesianGrid
- HexGrid
*/
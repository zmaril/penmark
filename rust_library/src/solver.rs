use crate::types::*;
use crate::sudoku::*;

fn get_next_empty_cell(grid: Puzzle) -> (usize, usize) {
    for x in 0..9 {
        for y in 0..9 {
            if grid[x][y] == 0 {
                return (x,y);
            }
        }
    }
    return (10, 10)
}

fn solve_helper(grid: Puzzle) -> (PuzzleEvaluation, Puzzle) {
    //println!("\nsolver_hepler {:?}", grid);
    match evaluate_puzzle(grid) {
        PuzzleEvaluation::Incomplete => {
            let next_empty_cell = get_next_empty_cell(grid);
            //println!("next empty cell: {:?}",next_empty_cell);
            for i in 1..9{
                let mut guess = grid.clone();
                guess[next_empty_cell.0][next_empty_cell.1] = i;
                //println!("next guess: {:?}", guess);
                //println!("eval'd: {:?}", evaluate_puzzle(guess));
                match evaluate_puzzle(guess) {
                    PuzzleEvaluation::Wrong => {},
                    PuzzleEvaluation::Solved => {
                        return (PuzzleEvaluation::Solved, guess)
                    },
                    PuzzleEvaluation::Incomplete => {
                        return solve_helper(guess);
                    }
                }
            }
            return (PuzzleEvaluation::Wrong, grid)
        },
        PuzzleEvaluation::Wrong => 
            return (PuzzleEvaluation::Wrong, grid),
        PuzzleEvaluation::Solved => 
            return (PuzzleEvaluation::Solved, grid)
    }
}

pub fn solve_sudoku(puzzle: Puzzle) -> (PuzzleEvaluation, Puzzle) {
    let mut solution : Puzzle = [[0; 9]; 9];
    for x in 0..9 {
        for y in 0..9 {
            if puzzle[x][y] != 0 {
                solution[x][y] = puzzle[x][y];
            };
        }
    }
    let helped = solve_helper(solution); 
    // let _i = match evaluate_puzzle(solution) {
    //     PuzzleEvaluation::Incomplete => 0,
    //     PuzzleEvaluation::Solved => 0,
    //     PuzzleEvaluation::Wrong => {
    //         return 
    //     }
    // };
    return helped;
}
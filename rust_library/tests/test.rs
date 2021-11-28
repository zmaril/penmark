use penmark::*; 

#[test]
fn basic_sudoku() {
    // let rules = vec![Rule{}];
    // let givens = vec![Cell{}];
    // let layout = Grid{};
    // let sudoku = Puzzle{rules, givens, layout}; 

    
    let result = 2 + 2;
    assert_eq!(result, 4);
}

#[test]
fn parsed_sudoku() {
    let sudoku = "
    1 2 3 | . . . | . . . 
    1 2 3 | . . . | . . . 
    1 2 3 | . . . | . . . 
    - - - + - - - + - - - 
    1 2 3 | . . . | . . . 
    1 2 3 | . . . | . . . 
    1 2 3 | . . . | . . . 
    - - - + - - - + - - - 
    1 2 3 | . . . | . . . 
    1 2 3 | . . . | . . . 
    1 2 3 | . . . | . . . 
    ";

    let results = parse_sudoku(sudoku);
    let expected : [[u8; 9]; 9]= [[1,2,3,0,0,0,0,0, 0]; 9];
    assert_eq!(results, expected);
}

#[test]
fn wrong_sudoku_by_column(){
    let sudoku = "
    1 . . | . . . | . . . 
    1 . . | . . . | . . . 
    . . . | . . . | . . . 
    - - - + - - - + - - - 
    . . . | . . . | . . . 
    . . . | . . . | . . . 
    . . . | . . . | . . . 
    - - - + - - - + - - - 
    . . . | . . . | . . . 
    . . . | . . . | . . . 
    . . . | . . . | . . . 
    ";

    
    let results = parse_sudoku(sudoku);
    assert_eq!(penmark::sudoku::PuzzleEvaluation::Wrong, penmark::sudoku::evaluate_puzzle(results));
}

#[test]
fn wrong_sudoku_by_row(){
    let sudoku = "
    1 1 . | . . . | . . . 
    . . . | . . . | . . . 
    . . . | . . . | . . . 
    - - - + - - - + - - - 
    . . . | . . . | . . . 
    . . . | . . . | . . . 
    . . . | . . . | . . . 
    - - - + - - - + - - - 
    . . . | . . . | . . . 
    . . . | . . . | . . . 
    . . . | . . . | . . . 
    ";

    
    let results = parse_sudoku(sudoku);
    assert_eq!(penmark::sudoku::PuzzleEvaluation::Wrong, penmark::sudoku::evaluate_puzzle(results));
}

#[test]
fn wrong_sudoku_by_box(){
    let sudoku = "
    1 . . | . . . | . . . 
    . 1 . | . . . | . . . 
    . . . | . . . | . . . 
    - - - + - - - + - - - 
    . . . | . . . | . . . 
    . . . | . . . | . . . 
    . . . | . . . | . . . 
    - - - + - - - + - - - 
    . . . | . . . | . . . 
    . . . | . . . | . . . 
    . . . | . . . | . . . 
    ";

    
    let results = parse_sudoku(sudoku);
    assert_eq!(penmark::sudoku::PuzzleEvaluation::Wrong, penmark::sudoku::evaluate_puzzle(results));
}

#[test]
fn incomplete_sudoku(){
    let sudoku = "
    . . . | . . . | . . . 
    . . . | . . . | . . . 
    . . . | . . . | . . . 
    - - - + - - - + - - - 
    . . . | . . . | . . . 
    . . . | . . . | . . . 
    . . . | . . . | . . . 
    - - - + - - - + - - - 
    . . . | . . . | . . . 
    . . . | . . . | . . . 
    . . . | . . . | . . . 
    ";

    
    let results = parse_sudoku(sudoku);
    assert_eq!(penmark::sudoku::PuzzleEvaluation::Incomplete, penmark::sudoku::evaluate_puzzle(results));
}

#[test]
fn solved_sudoku(){
    let sudoku = "
    1 2 3 | 4 5 6 | 7 8 9 
    4 5 6 | 7 8 9 | 1 2 3 
    7 8 9 | 1 2 3 | 4 5 6 
    - - - + - - - + - - - 
    3 1 2 | 9 7 8 | 6 4 5 
    6 4 5 | 3 1 2 | 9 7 8 
    9 7 8 | 6 4 5 | 3 1 2 
    - - - + - - - + - - - 
    2 3 1 | 8 9 7 | 5 6 4 
    5 6 4 | 2 3 1 | 8 9 7  
    8 9 7 | 5 6 4 | 2 3 1 
    ";

    let results = parse_sudoku(sudoku);
    assert_eq!(penmark::sudoku::PuzzleEvaluation::Solved, penmark::sudoku::evaluate_puzzle(results));
}

#[test]
fn can_solve_sudoku1() {
    let sudoku = "
    1 2 3 | 4 5 6 | 7 8 9 
    4 5 6 | 7 8 9 | 1 2 3 
    7 8 9 | 1 2 3 | 4 5 6 
    - - - + - - - + - - - 
    3 1 2 | 9 7 8 | 6 4 5 
    6 4 5 | 3 1 2 | 9 7 8 
    9 7 8 | 6 4 5 | 3 1 2 
    - - - + - - - + - - - 
    2 3 1 | 8 9 7 | 5 6 4 
    5 6 4 | 2 3 1 | 8 9 7  
    8 9 7 | 5 6 4 | 2 3 . 
    ";

    let results = parse_sudoku(sudoku);
    let (eval, grid) = solve_sudoku(results);

    assert_eq!(penmark::sudoku::PuzzleEvaluation::Solved, eval);
    assert_eq!(grid[8][8], 1);
}


#[test]
fn can_solve_sudoku2() {
    let sudoku = "
    1 2 3 | 4 5 6 | 7 8 9 
    4 5 6 | 7 8 9 | 1 2 3 
    7 8 9 | 1 2 3 | 4 5 6 
    - - - + - - - + - - - 
    3 1 2 | 9 7 8 | 6 4 5 
    6 4 5 | 3 1 2 | 9 7 8 
    9 7 8 | 6 4 5 | 3 1 2 
    - - - + - - - + - - - 
    2 3 1 | 8 9 7 | 5 6 4 
    5 6 4 | 2 3 1 | 8 9 7  
    8 9 7 | 5 6 4 | . 3 1 
    ";

    let results = parse_sudoku(sudoku);
    let (eval, grid) = solve_sudoku(results);

    assert_eq!(penmark::sudoku::PuzzleEvaluation::Solved, eval);
    assert_eq!(grid[8][6], 2);
}

#[test]
fn can_solve_sudoku3() {
    let sudoku = "
    1 2 3 | 4 5 6 | 7 8 9 
    4 5 6 | 7 8 9 | 1 2 3 
    7 8 9 | 1 2 3 | 4 5 6 
    - - - + - - - + - - - 
    3 1 2 | 9 7 8 | 6 4 5 
    6 4 5 | 3 1 2 | 9 7 8 
    9 7 8 | 6 4 5 | 3 1 2 
    - - - + - - - + - - - 
    2 3 1 | 8 9 7 | 5 6 4 
    5 6 4 | 2 3 1 | 8 9 7  
    8 9 7 | 5 6 4 | . 3 . 
    ";

    let results = parse_sudoku(sudoku);
    let (eval, grid) = solve_sudoku(results);

    assert_eq!(penmark::sudoku::PuzzleEvaluation::Solved, eval);
    assert_eq!(grid[8][8], 1);
    assert_eq!(grid[8][6], 2);
    println!("{:?}", grid);
}
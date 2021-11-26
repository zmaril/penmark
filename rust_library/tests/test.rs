use penmark::*; 

#[test]
fn basic_sudoku() {
    let rules = vec![Rule{}];
    let givens = vec![Cell{}];
    let layout = Grid{};
    let sudoku = Puzzle{rules, givens, layout}; 

    
    let result = 2 + 2;
    assert_eq!(result, 4);
}
use dancing_links::{latin_square, sudoku, ExactCover};
use std::panic;

const N: usize = 9;
const R: usize = 3;
const C: usize = 3;

pub fn gen_possibilities(grid: &Vec<Vec<u8>>, possibilities: &mut Vec<sudoku::Possibility>) {
    for row in 0..N {
        for column in 0..N {
            if grid[row][column] != 0 {
                possibilities.push(sudoku::Possibility {
                    row: row,
                    column: column,
                    square: (row / R) * R + (column / C),
                    value: grid[row][column] as usize,
                });
            }else{
                for value in 1..N+1{
                    possibilities.push(sudoku::Possibility {
                        row: row,
                        column: column,
                        square: (row / R) * R + (column / C),
                        value: value,
                    });
                }
            }
        }
    }
}

pub fn gen_constraints(constraints: &mut Vec<sudoku::Constraint>) {
    for i in 0..N {
        for j in 0..N {
            constraints.push(sudoku::Constraint::Latin(
                latin_square::Constraint::RowColumn { row: i, column:j },
            ));
            constraints.push(sudoku::Constraint::Latin(
                latin_square::Constraint::RowNumber { row: i, value: j+1 },
            ));
            constraints.push(sudoku::Constraint::Latin(
                latin_square::Constraint::ColumnNumber { column: i, value: j+1 },
            ));
            constraints.push(sudoku::Constraint::SquareNumber { square: i, value: j+1 });
        }
    }
}

pub fn grid_from_possibilities(grid: &mut Vec<Vec<u8>>, possibilities: &Vec<&sudoku::Possibility>){
    for possibility in possibilities{
        grid[possibility.row][possibility.column] = possibility.value as u8;
    }
}

pub fn string_to_grid(string: &str) -> Vec<Vec<u8>> {
    let mut grid = Vec::new();

    for _ in 0..9 {
        grid.push(vec![0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }

    for (i, c) in string.chars().enumerate() {
        grid[i / 9][i % 9] = c.to_digit(10).unwrap() as u8
    }

    grid
}

pub fn grid_to_string(grid: &Vec<Vec<u8>>) -> String {
    let mut string = String::new();
    for r in grid {
        for el in r {
            string = format!("{}{}", string.as_str(), el.to_string().as_str());
        }
    }
    String::from(string)
}

pub fn _read_string() ->  String{
    let mut string: String = String::new();

    std::io::stdin().read_line(&mut string)
        .ok()
        .expect("Error read line!");

    return String::from(string.trim());
}

pub fn solve_sudoku(sudoku: &str) -> Vec<String>{
    let mut grid = string_to_grid(
        sudoku
    );
    let mut possibilities = Vec::<sudoku::Possibility>::new();
    gen_possibilities(&grid, &mut possibilities);
    let mut constraints = Vec::<sudoku::Constraint>::new();
    gen_constraints(&mut constraints);
    let s = sudoku::Sudoku {
        possibilities,
        constraints,
    };
    let solver = s.solver();
    let result = panic::catch_unwind(panic::AssertUnwindSafe(|| solver.map(move |possibilities| {grid_from_possibilities(&mut grid, &possibilities); grid_to_string(&grid)}).collect()));
    match result{
        Ok(a) => a,
        Err(_) => Vec::new()
    }
}

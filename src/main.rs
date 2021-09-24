use std::collections::{HashMap, HashSet};


const SIDE_LENGTH: u8 = 9;
const R: u8 = 3;
const C: u8 = 3;

type RowKeys = (u8, u8, u8);
type ColumnKey = (u8, (u8, u8));


struct Solver{
  columns: HashMap<ColumnKey, HashSet<RowKeys>>,
  rows: HashMap<RowKeys, Vec<ColumnKey>>,
  solution: Vec<RowKeys>
}

impl Iterator for Solver{
  type Item = Vec<RowKeys>;

  fn next(&mut self) -> Option<Self::Item>{
    if self.columns.len() == 0{
      return Some(self.solution.to_owned())
    }else{
      let column = get_min_column(&mut self.columns);
      for row in self.columns[&column].to_owned() {
          self.solution.push(row);
          let mut cols = select(&mut self.columns, &mut self.rows, row);

          let sols = self.next();
          for s in sols {
              return Some(s)
          }
          deselect(&mut self.columns, &mut self.rows, row, &mut cols);
          self.solution.pop();
      }
    }

    None
  }
}

// returns vector of solutions
fn solve_sudoku(grid: &mut Vec<Vec<u8>>){
    let mut pre_columns = Vec::<ColumnKey>::new();

    for i in 0..SIDE_LENGTH {
        for j in 0..SIDE_LENGTH {
            pre_columns.push((0, (i, j))); // row, column
            pre_columns.push((1, (i, j+1))); // row, number
            pre_columns.push((2, (i, j+1))); // column, number
            pre_columns.push((3, (i, j+1))); // box, number
        }
    }

    let mut rows = HashMap::<RowKeys, Vec<ColumnKey>>::new();

    for row in 0..SIDE_LENGTH {
        for column in 0..SIDE_LENGTH {
            for number in 1..SIDE_LENGTH + 1 {
                let b = (row / R) * R + (column / C);
                rows.insert(
                    (row, column, number),
                    vec![
                      (0, (row, column)),
                      (1, (row, number)),
                      (2, (column, number)),
                      (3, (b, number))
                    ],
                );
            }
        }
    }

    let mut columns = HashMap::<ColumnKey, HashSet<RowKeys>>::new();
    exact_cover(&mut pre_columns, &mut rows, &mut columns);

    for (i, row) in grid.iter().enumerate() {
        for (j, number) in row.iter().enumerate() {
            if *number != 0 {
                select(&mut columns, &mut rows, (i as u8, j as u8, *number));
            }
        }
    }
    // print!("{:?}", columns);
    let mut solver = Solver{
      columns,
      rows,
      solution: Vec::<RowKeys>::new()
    };

    let solution = solver.next().unwrap();
    for (row, column, number) in solution {
        grid[row as usize][column as usize] = number
    }

    // solutions
}

fn get_min_column(columns: &mut HashMap<ColumnKey, HashSet<RowKeys>>) -> ColumnKey {
    let mut min_column = (0, (0, 0));
    let min_num = 999999;
    for (key, val) in columns.iter() {
        if val.len() < min_num {
            min_column = *key;
        }
    }
    min_column
}

fn deselect(
    columns: &mut HashMap<ColumnKey, HashSet<RowKeys>>,
    rows: &mut HashMap<RowKeys, Vec<ColumnKey>>,
    row: RowKeys,
    cols: &mut Vec<HashSet<RowKeys>>,
) {
    for j in rows[&row].iter().rev() {
        columns.insert(*j, cols.pop().unwrap());
        for i in columns[&j].clone() {
            for k in &rows[&i] {
                if k != j {
                    columns.get_mut(&k).unwrap().insert(i);
                }
            }
        }
    }
}

fn select(
    columns: &mut HashMap<ColumnKey, HashSet<RowKeys>>,
    rows: &mut HashMap<RowKeys, Vec<ColumnKey>>,
    row: RowKeys,
) -> Vec<HashSet<RowKeys>> {
    let mut cols = Vec::new();

    for j in &rows[&row] {
        /*Line below crash if no solution*/
        for i in columns[&j].clone() {
            for k in &rows[&i] {
                if k != j {
                    columns.get_mut(&k).unwrap().remove(&i);
                }
            }
        }
        cols.push(columns.remove(&j).unwrap());
    }

    cols
}

fn exact_cover(
    pre_columns: &mut Vec<ColumnKey>,
    rows: &mut HashMap<RowKeys, Vec<ColumnKey>>,
    columns: &mut HashMap<ColumnKey, HashSet<RowKeys>>,
) {
    for el in pre_columns {
        columns.insert(*el, HashSet::new());
    }

    for (i, row) in rows.iter() {
        for j in row {
            columns.get_mut(j).unwrap().insert(*i);
        }
    }
}

fn string_to_grid(string: &str) -> Vec<Vec<u8>> {
    let mut grid = Vec::new();

    for _ in 0..9 {
        grid.push(vec![0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }

    for (i, c) in string.chars().enumerate() {
        grid[i / 9][i % 9] = c.to_digit(10).unwrap() as u8
    }

    grid
}

fn grid_to_string(grid: &Vec<Vec<u8>>) -> String {
    let mut string = String::new();
    for r in grid {
        for el in r {
            string = format!("{}{}", string.as_str(), el.to_string().as_str());
        }
    }
    String::from(string)
}

fn _read_string() ->  String{
    let mut string: String = String::new();

    std::io::stdin().read_line(&mut string)
        .ok()
        .expect("Error read line!");

    return String::from(string.trim());
}

fn main() {
    let mut grid = string_to_grid(
        "060400910042001687100608050070260890000304006900810005219700008700580000080023009",
        // _read_string().as_str()
    );
    // print!("{:?}", grid);
    solve_sudoku(&mut grid);
    // let string_solutions: Vec<String> = solutions.iter().map(|x| grid_to_string(x)).collect();
    print!("{:?}", grid_to_string(&grid));
}
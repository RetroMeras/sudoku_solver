use std::collections::HashSet;
use std::cmp::Eq;
use std::hash::Hash;

struct Sudoku {
  sudoku: [[u8; 9]; 9],
  possibilities: Vec<Vec<HashSet<u8>>>,
}

fn get_difference<T: Eq + Hash + Copy>(
    val: &HashSet<T>,
    intersection: &HashSet<T>,
    ln: usize) -> HashSet<T>{
  if ln == intersection.len(){
    val.difference(intersection).into_iter().cloned().collect()
  }else{
    val.iter().cloned().collect()
  }
}

fn intersect(to_intersect: &mut Vec<&HashSet<u8>>) -> HashSet<u8>{
  let mut intersection: HashSet<u8> = vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9].iter().cloned().collect();
  for a in to_intersect{
    intersection = intersection.intersection(&a).into_iter().cloned().collect();
  }
  intersection
}

impl Sudoku {
  fn init(sudoku_string: &str) -> Sudoku {
    let mut sudoku = [[0u8; 9]; 9];
    let possibilities = vec![
      vec![
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
          .into_iter()
          .collect::<HashSet<u8>>();
        9
      ];
      9
    ];
    for i in sudoku_string.chars().enumerate() {
      sudoku[i.0 / 9][i.0 % 9] = i.1.to_digit(10).unwrap() as u8
    }
    Sudoku {
      sudoku,
      possibilities,
    }
  }
  fn remove_placed(&mut self) {
    for i in 0..9 {
      for j in 0..9 {
        if self.sudoku[i][j] != 0 {
          self.possibilities[i][j] = vec![].into_iter().collect();
        }
      }
    }
  }
  // making all manipulations with posibilities
  fn fill(&mut self) {
    for i in 0..9 {
      for j in 0..9 {
        for k in 0..9 {
          self.possibilities[i][j].remove(&self.sudoku[i][k]);
          self.possibilities[i][j].remove(&self.sudoku[k][j]);
          self.possibilities[i][j].remove(&self.sudoku[(i / 3) * 3 + k / 3][(j / 3) * 3 + k % 3]);
        }
      }
    }
    for i in 0..9{
      for j in 0..9{
        self.check_intersections(i, j);
      }
    }
  }
  // places values and returning true if does any chenges
  fn check(&mut self) -> bool {
    let mut ans = false;
    for i in 0..9 {
      for j in 0..9 {
        if self.possibilities[i][j].len() == 1 {
          self.sudoku[i][j] = self.possibilities[i][j].iter().cloned::<u8>().collect::<Vec<u8>>()[0];
          self.possibilities[i][j] = vec![].into_iter().collect();
          ans = true;
        }
      }
    }
    ans
  }
  fn fill_and_check(&mut self) -> bool {
    self.fill();
    println!("{}", self);
    self.check()
  }
  fn solve(&mut self) {
    self.remove_placed();
    println!("{}", self);
    while self.fill_and_check() {}
  }

  //checks if any other value cant be placed in this square
  fn check_intersections(&mut self, x: usize, y: usize) {
    let mut to_horizontal_intersect: Vec<&HashSet<u8>> = vec![];
    let mut to_vertical_intersect: Vec<&HashSet<u8>> = vec![];
    let mut to_box_intersect: Vec<&HashSet<u8>> = vec![];
    for k in 0..9 {
      if !self.possibilities[x][k].is_empty() && k != y {
        to_horizontal_intersect.push(&self.possibilities[x][k])
      }
      if !self.possibilities[k][y].is_empty() && k != x{
        to_vertical_intersect.push(&self.possibilities[k][y])
      }
      if !self.possibilities[(x / 3) * 3 + k / 3][(y / 3) * 3 + k % 3].is_empty()
            && (x / 3) * 3 + k / 3 != x
            && (y / 3) * 3 + k % 3 != y{
              to_box_intersect.push(&self.possibilities[(x / 3) * 3 + k / 3][(y / 3) * 3 + k % 3])
      }
    }

    let intersection_h = intersect(&mut to_horizontal_intersect);
    let intersection_v = intersect(&mut to_vertical_intersect);
    let intersection_b = intersect(&mut to_box_intersect);
    // println!("{:?} {:?} {}", (x, y), intersection_h, to_horizontal_intersect.len());
    // println!("{:?} {:?} {}", (x, y), intersection_v, to_vertical_intersect.len());
    // println!("{:?} {:?} {}", (x, y), intersection_b, to_box_intersect.len());
    if intersection_h.len() == to_horizontal_intersect.len() {
      self.possibilities[x][y] = get_difference(&self.possibilities[x][y], &intersection_h, to_horizontal_intersect.len());
    }else if intersection_v.len() == to_vertical_intersect.len(){
      self.possibilities[x][y] = get_difference(&self.possibilities[x][y], &intersection_v, to_vertical_intersect.len());
    }else if intersection_b.len() == to_box_intersect.len(){
      self.possibilities[x][y] = get_difference(&self.possibilities[x][y], &intersection_b, to_box_intersect.len());
    }
  }


}

impl std::fmt::Display for Sudoku {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "sudoku:\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\npossibilities:\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n",
      self.sudoku[0],
      self.sudoku[1],
      self.sudoku[2],
      self.sudoku[3],
      self.sudoku[4],
      self.sudoku[5],
      self.sudoku[6],
      self.sudoku[7],
      self.sudoku[8],
      self.possibilities[0],
      self.possibilities[1],
      self.possibilities[2],
      self.possibilities[3],
      self.possibilities[4],
      self.possibilities[5],
      self.possibilities[6],
      self.possibilities[7],
      self.possibilities[8],
    )
  }
}

fn main() {
  let mut sudoku = Sudoku::init(
    &"000850009086029000500000300000002008807000406053400900908243051000006073631780094".to_owned(),
  );
  sudoku.remove_placed();
  println!("{}", sudoku);
  sudoku.solve();
  println!("{}", sudoku);
  // sudoku.solve();
  // println!("{}", sudoku);
}

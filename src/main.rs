use std::cmp::Eq;
use std::collections::HashSet;
use std::hash::Hash;
// use std::io::{stdin};

struct Sudoku {
  sudoku: [[u8; 9]; 9],
  possibilities: Vec<Vec<HashSet<u8>>>,
}

fn get_difference<T: Eq + Hash + Copy>(
  val: &HashSet<T>,
  intersection: &HashSet<T>,
  ln: usize,
) -> HashSet<T> {
  if ln == intersection.len() {
    val.difference(intersection).into_iter().cloned().collect()
  } else {
    val.iter().cloned().collect()
  }
}

// fn print_indexes(){
//   for i in 0..9{
//     for j in 0..9{
//       print!("({} {}) ", i, j);
//     }
//     println!();
//   }
// }

fn intersect(to_intersect: &mut Vec<&HashSet<u8>>) -> HashSet<u8> {
  let mut intersection: HashSet<u8> = vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9].iter().cloned().collect();
  for a in to_intersect {
    intersection = intersection.intersection(&a).into_iter().cloned().collect();
  }
  intersection
}

fn unit(to_unit: &mut Vec<&HashSet<u8>>) -> HashSet<u8> {
  let mut unit: HashSet<u8> = vec![].iter().cloned().collect();
  for a in to_unit {
    unit = unit.union(&a).into_iter().cloned().collect();
  }
  unit
}

fn difference(a: &HashSet<u8>, b: &HashSet<u8>) -> HashSet<u8> {
  a.difference(b).into_iter().cloned().collect()
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
    for (i, c) in sudoku_string.chars().enumerate() {
      sudoku[i / 9][i % 9] = c.to_digit(10).unwrap() as u8
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
  // making all manipulations with possibilities
  fn fill(&mut self) {
    for i in 0..9 {
      for j in 0..9 {
        for k in 0..9 {
          self.possibilities[i][j].remove(&self.sudoku[i][k]);
          self.possibilities[i][j].remove(&self.sudoku[k][j]);
          self.possibilities[i][j].remove(&self.sudoku[(i / 3) * 3 + k / 3][(j / 3) * 3 + k % 3]);
        }
        self.check_intersections_and_unions(i, j);
      }
    }
  }
  // places values and returning true if does any chenges
  fn check(&mut self) -> bool {
    let mut ans = false;
    for i in 0..9 {
      for j in 0..9 {
        if self.possibilities[i][j].len() == 1 {
          self.sudoku[i][j] = self.possibilities[i][j]
            .iter()
            .cloned::<u8>()
            .collect::<Vec<u8>>()[0];
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
  fn check_intersections_and_unions(&mut self, x: usize, y: usize) {
    let mut horizontal: Vec<&HashSet<u8>> = vec![];
    let mut vertical: Vec<&HashSet<u8>> = vec![];
    let mut boxes: Vec<&HashSet<u8>> = vec![];
    for k in 0..9 {
      if !self.possibilities[x][k].is_empty() && k != y {
        horizontal.push(&self.possibilities[x][k])
      }
      if !self.possibilities[k][y].is_empty() && k != x {
        vertical.push(&self.possibilities[k][y])
      }
      if !self.possibilities[(x / 3) * 3 + k / 3][(y / 3) * 3 + k % 3].is_empty()
        && (x / 3) * 3 + k / 3 != x
        && (y / 3) * 3 + k % 3 != y
      {
        boxes.push(&self.possibilities[(x / 3) * 3 + k / 3][(y / 3) * 3 + k % 3])
      }
    }

    let intersection_h = intersect(&mut horizontal);
    let intersection_v = intersect(&mut vertical);
    let intersection_b = intersect(&mut boxes);
    let union_h = difference(&self.possibilities[x][y], &unit(&mut horizontal));
    let union_v = difference(&self.possibilities[x][y], &unit(&mut vertical));
    let union_b = difference(&self.possibilities[x][y], &unit(&mut boxes));
    // println!("{:?} {:?} {}", (x, y), intersection_h, to_horizontal_intersect.len());
    // println!("{:?} {:?} {}", (x, y), intersection_v, to_vertical_intersect.len());
    // println!("{:?} {:?} {}", (x, y), intersection_b, to_box_intersect.len());
    if intersection_h.len() == horizontal.len() {
      self.possibilities[x][y] =
        get_difference(&self.possibilities[x][y], &intersection_h, horizontal.len());
    } else if intersection_v.len() == vertical.len() {
      self.possibilities[x][y] =
        get_difference(&self.possibilities[x][y], &intersection_v, vertical.len());
    } else if intersection_b.len() == boxes.len() {
      self.possibilities[x][y] =
        get_difference(&self.possibilities[x][y], &intersection_b, boxes.len());
    }
    if !union_h.is_empty() {
      self.possibilities[x][y] = union_h.iter().cloned().collect();
    } else if !union_v.is_empty() {
      self.possibilities[x][y] = union_v.iter().cloned().collect();
    } else if !union_b.is_empty() {
      self.possibilities[x][y] = union_b.iter().cloned().collect();
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
  // let mut sudoku = String::with_capacity(81);
  // stdin().read_line(&mut sudoku).unwrap();
  // println!("{}", sudoku);
  // let mut sudoku = Sudoku::init(
  //   &sudoku[..81],
  // );
  let mut sudoku = Sudoku::init(
    &"070530000801600207000000010000406000300000745080000006405000070003100029000000500".to_owned(),
  );
  sudoku.remove_placed();
  // print_indexes();
  println!("{}", sudoku);
  sudoku.solve();
  println!("{}", sudoku);
  // sudoku.solve();
  // println!("{}", sudoku);
}

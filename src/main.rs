mod lib;
use lib::*;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::time::Instant;
use std::cmp::{min, max};

#[derive(Debug, Deserialize)]
struct SudokuJSON{
  sudoku: String,
  solution: String,
  mask: String,
  difficulty: String
}
fn read_json(mode: &str) -> Vec<SudokuJSON>{
  let mut file = File::open(format!("data_{}.json", mode)).unwrap();
  let mut data = String::new();
  file.read_to_string(&mut data).unwrap();

  serde_json::from_str(&data).unwrap()
}

fn main(){
  // env::set_var("RUST_BACKTRACE", "1");
  let mods = ["easy", "medium", "hard", "expert"];
  for mode in mods {
    let sudoku_vec = read_json(mode);
    let mut av_time = 0;
    let mut max_time = 0;
    let mut min_time = u128::MAX;
    for sudoku in sudoku_vec.iter().map(|x| {&x.sudoku}){
      // println!("{}", sudoku.replace("0", " "));
      let start = Instant::now();
      let _solutions = solve_sudoku(sudoku);
      let duration = start.elapsed().as_nanos();
      av_time += duration;
      min_time = min(min_time, duration);
      max_time = max(max_time, duration);
    }
    println!("mode: {}\n\tavg: {}\n\tmax: {}\n\tmin: {}", mode, av_time as f64 /sudoku_vec.len() as f64, max_time, min_time);
  }
}
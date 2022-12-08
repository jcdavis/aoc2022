use std::env;
use std::fs;

use regex::Regex;

fn main() {
  let args: Vec<String> = env::args().collect();
  let input = fs::read_to_string(&args[1]).unwrap();
  let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();

  let total = input.lines().map(|line| {
    let c = re.captures(line).unwrap();
    ((c[1].parse::<i32>().unwrap(), c[2].parse::<i32>().unwrap()),
      (c[3].parse::<i32>().unwrap(), c[4].parse::<i32>().unwrap()))
  }).filter( |tuples| {
      let ((left_min, left_max), (right_min, right_max)) = tuples;
      (left_min <= right_min && right_min <= left_max) ||
        (right_min <= left_min && left_min <= right_max)
  }).count();

  println!("{}", total);
}

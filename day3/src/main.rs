use std::env;
use std::fs;
use std::collections::HashSet;

fn main() {
  let args: Vec<String> = env::args().collect();
  let input = fs::read_to_string(&args[1]).unwrap();
  let lines: Vec<&str> = input.lines().collect();

  let sum: u32 = lines.iter().map(|line| {
    let chars: Vec<char> = line.chars().collect();
    let mid = chars.len()/2;
    let left: HashSet<&char> = chars[0..mid].iter().collect();
    let right: HashSet<&char> = chars[mid..].iter().collect();
    left.intersection(&right).map(|c| {
      let as_num = **c as u32;
      if as_num < 91 {
        as_num - 38
      } else {
        as_num - 96
      }
    }).sum::<u32>()
  }).sum();
  println!("{}", sum);
}

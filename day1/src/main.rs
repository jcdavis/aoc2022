use std::env;
use std::fs;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
  let args: Vec<String> = env::args().collect();
  let input = fs::read_to_string(&args[1]).unwrap();
  let lines: Vec<&str> = input.lines().collect();
  let mut cal: i32 = 0;
  let mut heap = BinaryHeap::new();

  for line in &lines {
    if !line.is_empty() {
      cal += line.parse::<i32>().unwrap();
    } else {
      heap.push(Reverse(cal));
      if heap.len() > 3 {
        heap.pop();
      }
      cal = 0;
    }
  }
  println!("{:#?}", heap.iter().map(|r| r.0).sum::<i32>());
}

use std::env;
use std::fs;

use regex::Regex;


fn main() {
  let args: Vec<String> = env::args().collect();
  let input = fs::read_to_string(&args[1]).unwrap();
  let lines: Vec<&str> = input.lines().collect();
  let chars: Vec<Box<Vec<char>>> = lines.iter().map(|line| Box::new(line.chars().collect())).collect();

  let mut stacks: Vec<Box<Vec<char>>> = Vec::new();
  stacks.push(Box::new(Vec::new()));

  let mut i: usize = 0;

  while !chars[i].starts_with(&[' ', '1']) {
    i += 1;
  }

  for col in (1..chars[i].len()).step_by(4) {
    let mut row = i-1;
    let mut boxes = Vec::new();
    while chars[row][col] != ' ' {
      boxes.push(chars[row][col]);

      if row == 0 {
        break;
      } else {
        row -= 1;
      }
    }
    println!("parsed {:?}", boxes);
    stacks.push(Box::new(boxes));
  }

  let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
  for line in &lines[i+2..] {
    let cap = re.captures(line).unwrap();
    let boxes: usize = cap[1].parse().unwrap();
    let from: usize = cap[2].parse().unwrap();
    let to: usize = cap[3].parse().unwrap();

    let mut tmp_stack = Vec::new();
    for _ in 0..boxes {
      let c = stacks[from].pop().unwrap();
      tmp_stack.push(c);
    }

    for _ in 0..boxes {
      let c = tmp_stack.pop().unwrap();
      stacks[to].push(c);
    }
  }

  for stack in &stacks[1..] {
    print!("{:?}", stack.last().unwrap());
  }
  println!("");
}

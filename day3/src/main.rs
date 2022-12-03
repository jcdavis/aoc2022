use std::env;
use std::fs;
use std::collections::HashSet;

fn priority(c: char) -> u32 {
  let as_num = c as u32;
  if as_num < 91 {
    as_num - 38
  } else {
    as_num - 96
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let input = fs::read_to_string(&args[1]).unwrap();
  let lines: Vec<&str> = input.lines().collect();
  let mut sum: u32 = 0;
  let mut i: usize = 0;

  while i < lines.len() {
    let s1: HashSet<char> = lines[i].chars().collect();
    let s2: HashSet<char> = lines[i+1].chars().collect();
    let s3: HashSet<char> = lines[i+2].chars().collect();
    
    let in_all: Vec<&char> = s1.iter().filter(|c| s2.contains(c) && s3.contains(c)).collect();
    assert!(in_all.len() == 1);
    sum += priority(*in_all[0]);
    i += 3;
  }
  println!("{}", sum);
}

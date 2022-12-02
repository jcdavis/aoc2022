use std::env;
use std::fs;

fn main() {
  let args: Vec<String> = env::args().collect();
  let input = fs::read_to_string(&args[1]).unwrap();
  let lines: Vec<&str> = input.lines().collect();

  let result: i32 = lines.iter().map(|line| {
    let chars: Vec<char> = line.chars().collect();
    let them = (chars[0] as i32) - ('A' as i32);
    let us = (chars[2] as i32) - ('X' as i32);
    let win_score = match (us - them + 3) % 3 {
      0 => 3, //Tie
      1 => 6, //Win
      2 => 0, //Draw
      _ => panic!("wtf"),
    };
    win_score + us+1
  }).sum();
  println!("{}", result);
}

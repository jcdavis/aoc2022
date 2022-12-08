use std::env;
use std::fs;


fn main() {
  let args: Vec<String> = env::args().collect();
  let input: Vec<char> = fs::read_to_string(&args[1]).unwrap().chars().collect();
  let mut buffer = Vec::new();

  for i in 0..4 {
    buffer.push(input[i]);
  }

  let mut pos: usize = 0;
  for i in 4..input.len() {
    buffer[pos] = input[i];
    pos = (pos+1)%4;

    if buffer.iter().collect::<std::collections::HashSet<&char>>().len() == 4 {
      println!("{} {:?}", i, buffer);
      return;
    }
  }
}

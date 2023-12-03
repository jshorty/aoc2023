use std::fs;

fn read_lines() -> Vec<String> {
  return fs::read_to_string("input/day4.txt").unwrap().lines().map(String::from).collect();
}

pub fn day4() {
  read_lines();
  println!("Part 1: {}", 0);
  println!("Part 2: {}", 0);
}

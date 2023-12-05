use std::fs;

pub fn day5() {
  let input = fs::read_to_string("input/day5.txt").unwrap();
  let sections : Vec<&str> = input.split("\n\n").collect();

  println!("Part 1: {}\nPart 2: {}", sections[0], 0);
}

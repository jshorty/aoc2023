use std::fs;

fn read_lines() -> Vec<String>{
  return fs::read_to_string("input/day1.csv").unwrap().lines().map(String::from).collect();
}

fn find_calibration_value(line: String) -> i32 {
  // println!("{}", line);
  let mut all_digits = String::new();
  for c in line.chars() {
    if c.is_digit(10) {
      all_digits.push(c);
    }
  }
  let mut digits = all_digits.chars();
  if digits.clone().count() == 1 {
    all_digits.push(*digits.next().get_or_insert('0'));
    digits = all_digits.chars();
  }
  let mut result = String::new();
  result.push(*digits.next().get_or_insert('0'));
  result.push(*digits.last().get_or_insert('0'));
  return result.parse::<i32>().unwrap();
}

fn read_and_sum_calibration_values() -> i32 {
  let lines = read_lines();
  let mut sum = 0;
  for line in lines {
    let val = find_calibration_value(line);
    // println!("{}", val);
    sum += val;
  }
  return sum;
}

pub fn day1() {
  println!("{}", read_and_sum_calibration_values());
}

use std::fs;

fn read_lines() -> Vec<String>{
  return fs::read_to_string("input/day1.csv").unwrap().lines().map(String::from).collect();
}

fn find_calibration_value(raw_line: String, digits_only: bool) -> i32 {
  // println!("{}", raw_line);
  let mut line = raw_line.clone();

  if !digits_only {
    line = line.replace("one", "o1e")
               .replace("two", "t2o")
               .replace("three", "t3e")
               .replace("four", "4")
               .replace("five", "5e")
               .replace("six", "6")
               .replace("seven", "7n")
               .replace("eight", "e8t")
               .replace("nine", "n9e");
  }

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

fn read_and_sum_calibration_values(digits_only: bool) -> i32 {
  let lines = read_lines();
  let mut sum = 0;
  for line in lines {
    let val = find_calibration_value(line, digits_only);
    // println!("{}", val);
    sum += val;
  }
  return sum;
}

pub fn day1() {
  println!("Part 1: {}", read_and_sum_calibration_values(true));
  println!("Part 2: {}", read_and_sum_calibration_values(false));
}

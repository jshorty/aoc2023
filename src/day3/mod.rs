use std::fs;

fn read_lines() -> Vec<String> {
  return fs::read_to_string("input/day3.txt").unwrap().lines().map(String::from).collect();
}

fn touches_symbol(x: i32, y: i32, symbols: &mut Vec<(i32, i32, char, i32, i32)>, num: i32) -> bool {
  for coords in symbols {
    if x == coords.0 && y == coords.1 {
      return false;
    }

    let dist_x = (x - coords.0).abs();
    let dist_y = (y - coords.1).abs();
    if dist_x < 2 && dist_y < 2 {
      coords.3 += 1;
      if coords.4 == 0 {
        coords.4 = num;
      } else {
        coords.4 *= num;
      }
      return true;
    }
  }
  return false;
}

fn sum_part_numbers() -> (i32, Vec<(i32, i32, char, i32, i32)>) {
  let lines = read_lines();

  let mut symbols : Vec<(i32, i32, char, i32, i32)> = Vec::new();
  let mut numbers = Vec::new();
  let mut digit_coords = Vec::new();

  let mut y = 0;
  for line in lines {
    let mut x = 0;
    let mut current_digit = "".to_string();
    let chars = line.chars();
    for c in chars {
      if c.is_digit(10) {
        if current_digit == "".to_string() {
          digit_coords.push((x, y));
        }
        current_digit.push_str(&c.to_string());
      } else if c != '.' {
        symbols.push((x, y, c, 0, 0));
        if current_digit != "".to_string() {
          numbers.push(current_digit);
          current_digit = "".to_string();
        }
      } else if current_digit != "".to_string() {
        numbers.push(current_digit);
        current_digit = "".to_string();
      }
      x += 1;
    }
    if current_digit != "".to_string() {
      numbers.push(current_digit);
    }
    y += 1;
  }

  let mut sum = 0;
  let mut i = 0;
  for number_str in numbers {
    let length = number_str.len() as i32;
    let pos = digit_coords[i];

    let mut touches = false;
    let num = number_str.parse::<i32>().unwrap();

    let mut offset : i32 = 0;
    while offset < length {
      if touches_symbol(pos.0 + offset, pos.1, &mut symbols, num) {
        touches = true;
        break
      }
      offset += 1;
    }
    if touches {
      // println!("ADDING {} FROM LINE {}", num, pos.1 + 1);
      sum += num;
    }
    i += 1;
  }
  return (sum, symbols);
}

pub fn day3() {
  let result = sum_part_numbers();
  println!("Part 1: {}", result.0);

  let symbols = result.1;
  let mut sum_gear_ratios = 0;
  for sym in symbols {
    if sym.2 == '*' && sym.3 == 2 {
      sum_gear_ratios += sym.4;
    }
  }
  println!("Part 2: {}", sum_gear_ratios);
}

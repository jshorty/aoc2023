use std::fs;

fn read_lines() -> Vec<String> {
  return fs::read_to_string("input/day2.txt").unwrap().lines().map(String::from).collect();
}

fn evaluate_line(line: String) -> (i32, i32) {
  let mut split: Vec<String> = line.split(&[':', ';'][..]).map(String::from).collect();

  let game_id = split.remove(0).strip_prefix("Game ").unwrap().parse::<i32>().unwrap();

  let mut r = 0;
  let mut g = 0;
  let mut b = 0;

  for subset in split {
    let cube_counts: Vec<String> = subset.split(",").map(str::trim).map(String::from).collect();
    for cube_count in cube_counts {
      if cube_count.ends_with(" red") {
        let count = cube_count.strip_suffix(" red").unwrap().parse::<i32>().unwrap();
        if r < count {
          r = count;
        }
      } else if cube_count.ends_with(" green") {
        let count = cube_count.strip_suffix(" green").unwrap().parse::<i32>().unwrap();
        if g < count {
          g = count;
        }
      } else if cube_count.ends_with(" blue") {
        let count = cube_count.strip_suffix(" blue").unwrap().parse::<i32>().unwrap();
        if b < count {
          b = count;
        }
      }
    }
  }

  if r <= 12 && g <= 13 && b <= 14 {
    return (game_id, r * g * b);
  } else {
    return (0, r * g * b);
  }
}

pub fn day2() {
  let lines = read_lines();
  let mut game_id_sum = 0;
  let mut cube_power_sum = 0;
  for line in lines {
    let result = evaluate_line(line);
    game_id_sum += result.0;
    cube_power_sum += result.1;
  }

  println!("Part 1: {}", game_id_sum);
  println!("Part 2: {}", cube_power_sum);
}

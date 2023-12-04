use std::fs;

fn read_lines() -> Vec<String> {
  return fs::read_to_string("input/day4.txt").unwrap().lines().map(String::from).collect();
}

fn score_scratchcard(line: String) -> i32 {
  //println!("Parsing line: {}", line);
  let sections : Vec<String> = line.split(&[':', '|'][..]).map(String::from).collect();
  let winning_numbers : Vec<i32> = sections[1].split(' ').filter(|s| s.len() > 0).map(|s| s.parse::<i32>().unwrap()).collect();
  let card_numbers : Vec<i32> = sections[2].split(' ').filter(|s| s.len() > 0).map(|s| s.parse::<i32>().unwrap()).collect();

  let mut score = 0;
  for n in winning_numbers {
    if card_numbers.contains(&n) {
      if score == 0 {
        score = 1;
      } else {
        score *= 2;
      }
    }
  }
  return score;
}

pub fn day4() {
  let lines = read_lines();
  let mut total_score = 0;
  for line in lines {
    total_score += score_scratchcard(line);
  }
  println!("Part 1: {}", total_score);
  println!("Part 2: {}", 0);
}

use std::collections::HashMap;
use std::fs;

fn read_lines() -> Vec<String> {
  return fs::read_to_string("input/day4.txt").unwrap().lines().map(String::from).collect();
}

fn score_scratchcard(line: &String) -> i32 {
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

fn win_count_scratchcard(line: &String) -> i32 {
  let sections : Vec<String> = line.split(&[':', '|'][..]).map(String::from).collect();
  let winning_numbers : Vec<i32> = sections[1].split(' ').filter(|s| s.len() > 0).map(|s| s.parse::<i32>().unwrap()).collect();
  let card_numbers : Vec<i32> = sections[2].split(' ').filter(|s| s.len() > 0).map(|s| s.parse::<i32>().unwrap()).collect();

  let mut win_count = 0;
  for n in winning_numbers {
    if card_numbers.contains(&n) {
      win_count += 1;
    }
  }
  return win_count;
}

pub fn day4() {
  let lines = read_lines();
  let mut total_score = 0;
  for line in &lines {
    total_score += score_scratchcard(line);
  }

  let mut cards : Vec<i32> = Vec::new();
  let mut card_number = 1;
  let mut win_counts = HashMap::new();
  for line in &lines {
    cards.push(card_number);
    win_counts.insert(card_number, win_count_scratchcard(line));
    card_number += 1;
  }

  let mut cards_to_process : Vec<i32> = cards.clone();
  let initial_cards = cards.len() as i32;
  let mut final_cards = 0 as i32;

  while cards_to_process.len() > 0 {
    let mut next_cards_to_process : Vec<i32> = Vec::new();
    for card in cards_to_process {
      let mut wins = win_counts.get(&card).unwrap().clone();
      let mut next_card = card + 1;
      while wins > 0 && next_card <= initial_cards {
        next_cards_to_process.push(next_card.clone());
        wins -= 1;
        next_card += 1;
      }
      final_cards += 1;
    }
    cards_to_process = next_cards_to_process;
  }

  // for (k, v) in win_counts {
  //   println!("WIN COUNT FOR CARD {}: {}", k, v);
  // };

  println!("Part 1: {}", total_score);
  println!("Part 2: {}", final_cards);
}

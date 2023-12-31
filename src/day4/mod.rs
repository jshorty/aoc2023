use std::collections::HashMap;
use std::fs;

fn score_scratchcard(line: &String) -> (i32, i32) {
  let sections : Vec<String> = line.split(&[':', '|'][..]).map(String::from).collect();
  let winning_numbers : Vec<i32> = sections[1].split(' ').filter(|s| s.len() > 0).map(|s| s.parse::<i32>().unwrap()).collect();
  let card_numbers : Vec<i32> = sections[2].split(' ').filter(|s| s.len() > 0).map(|s| s.parse::<i32>().unwrap()).collect();

  let mut win_count = 0;
  for n in winning_numbers {
    if card_numbers.contains(&n) { win_count += 1; }
  }
  return (win_count, 2_i32.pow(win_count as u32));
}

pub fn day4() {
  let lines : Vec<String> = fs::read_to_string("input/day4.txt").unwrap().lines().map(String::from).collect();
  let mut cards : Vec<i32> = Vec::new();
  let mut win_counts = HashMap::new();
  let mut total_score = 0;

  let mut card_number = 0;
  for line in &lines {
    card_number += 1;
    cards.push(card_number);
    let result = score_scratchcard(line);
    win_counts.insert(card_number, result.0);
    total_score += result.1;
  }

  let mut final_cards = 0 as i32;
  while cards.len() > 0 {
    let mut next_cards_to_process : Vec<i32> = Vec::new();
    for card in cards {
      let mut wins = win_counts.get(&card).unwrap().clone();
      let mut next_card = card + 1;
      while wins > 0 && next_card <= (card_number) {
        next_cards_to_process.push(next_card.clone());
        wins -= 1;
        next_card += 1;
      }
      final_cards += 1;
    }
    cards = next_cards_to_process;
  }

  println!("Part 1: {}\nPart 2: {}", total_score, final_cards);
}

use std::collections::HashMap;
use std::fs;

// 7: 5oak, 6: 4oak, 5: fh, 4: 3oak, 3: 2p, 2: 1p, 1: hc
fn parse_hand(hand: &str) -> i32 {
  let mut counts = HashMap::<char, i32>::new();
  for card in hand.chars() {
    if counts.get(&card).is_none() {
      counts.insert(card, 1);
    } else {
      counts.entry(card).and_modify(|c| *c += 1);
    }
  }
  let distinct = counts.values().len();
  let max = counts.values().max().unwrap();

  match distinct {
    1 => return 7,
    2 => return max + 2,
    3 => return max + 1,
    4 => return 2,
    _ => return 1,
  }
}

fn parse_tiebreaker(hand: &str) -> (i32, i32, i32, i32, i32) {
  let mut result = Vec::<i32>::new();
  for card in hand.chars() {
    match card {
      'A' => result.push(14),
      'K' => result.push(13),
      'Q' => result.push(12),
      'J' => result.push(11),
      'T' => result.push(10),
      _ => result.push(card.to_digit(10).unwrap() as i32)
    }
  }
  return (result[0], result[1], result[2], result[3], result[4]);
}

pub fn day7() {
  let lines : Vec<String> = fs::read_to_string("input/day7.txt").unwrap().lines().map(String::from).collect();
  let mut hand_data = Vec::<(i32, (i32, i32, i32, i32, i32), i32, String)>::new();
  for line in lines {
    let split : Vec<&str> = line.split(' ').collect();
    let hand_score = parse_hand(split[0]);
    let tiebreaker = parse_tiebreaker(split[0]);
    let bid = split[1].parse::<i32>().unwrap();
    hand_data.push((hand_score, tiebreaker, bid, split[0].to_string().clone()));
  }

  hand_data.sort_by_key( |k| (k.0, k.1) );
  let mut i = 1;
  let mut sum = 0;
  for hand in hand_data {
    println!("{}: {}", hand.3, hand.0);
    sum += hand.2 * i;
    i += 1;
  }
  println!("Part 1: {}", sum);
}

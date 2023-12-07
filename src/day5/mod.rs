use std::fs;
use std::ops::Range;

fn parse_seeds(line: &str) -> Vec<i64> {
  let mut split = line.split(' ').collect::<Vec<&str>>();
  split.remove(0);
  return split.iter().map( |s| s.parse::<i64>().unwrap() ).collect();
}

fn parse_seed_ranges(line: &str) -> Vec<Range<i64>> {
  let mut split = line.split(' ').collect::<Vec<&str>>();
  split.remove(0);
  let mut result = Vec::new();
  while split.len() > 0 {
    let range_start = split.remove(0).parse::<i64>().unwrap();
    let range_len = split.remove(0).parse::<i64>().unwrap();
    result.push(range_start..range_start + range_len);
  }
  return result;
}

fn parse_map(chunk: &str) -> Vec<((i64, i64), i64)> {
  //println!("PARSING CHUNK:\n{}", chunk);
  let mut lines : Vec<&str> = chunk.split('\n').collect();
  lines.remove(0);
  let mut results : Vec<((i64, i64), i64)> = Vec::new();
  for line in lines {
    if line.is_empty() { continue; }
    let vals : Vec<i64> = line.split(' ').filter( |s| !s.is_empty() ).map( |s| s.parse::<i64>().unwrap() ).collect();
    let input_range = (vals[1], vals[1] + vals[2] - 1);
    let output_offset = vals[0] - vals[1];
    results.push((input_range, output_offset));
  }
  return results;
}

fn find_mapping(input: i64, map: Vec<((i64, i64), i64)>) -> i64 {
  for (input_range, output_offset) in map {
    if input >= input_range.0 && input <= input_range.1 {
      return input + output_offset;
    }
  }
  return input;
}

pub fn day5() {
  let input = fs::read_to_string("input/day5.txt").unwrap();
  let mut sections : Vec<&str> = input.split("\n\n").collect();

  let seeds = parse_seeds(sections.first().unwrap());
  let seed_ranges = parse_seed_ranges(sections.remove(0));

  let maps = sections.iter().map( |s| parse_map(s) );

  let mut locations = Vec::new();
  for seed in seeds {
    let mut val = seed.clone();
    for map in maps.clone() {
      //let old_val = val.clone();
      val = find_mapping(val, map);
      //println!("{}. mapped {} to {}", i, old_val, val);
    }
    locations.push(val);
  }

  let mut min_loc = locations.remove(0);
  for loc in locations {
    if loc < min_loc {
      min_loc = loc;
    }
  }

  let mut locations2 = Vec::new();
  for seed_range in seed_ranges {
    println!("NEXT SEED RANGE!");
    for seed in seed_range {
      let mut val = seed.clone();
      for map in maps.clone() {
        val = find_mapping(val, map);
        //println!("{}. mapped {} to {}", i, old_val, val);
      }
      locations2.push(val);
    }
  }

  let mut min_loc2 = locations2.remove(0);
  for loc in locations2 {
    if loc < min_loc {
      min_loc2 = loc;
    }
  }

  println!("Part 1: {}\nPart 2: {}", min_loc, min_loc2);
}

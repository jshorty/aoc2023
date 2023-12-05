use std::fs;

fn parse_seeds(line: &str) -> Vec<i64> {
  let mut split = line.split(' ').collect::<Vec<&str>>();
  split.remove(0);
  return split.iter().map( |s| s.parse::<i64>().unwrap() ).collect();
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

  let seeds = parse_seeds(sections.remove(0));

  let maps = sections.iter().map( |s| parse_map(s) );

  // let seed_to_soil = parse_map(sections[1]);
  // let soil_to_fert = parse_map(sections[2]);
  // let fert_to_water = parse_map(sections[3]);
  // let water_to_lite = parse_map(sections[4]);
  // let lite_to_temp = parse_map(sections[5]);
  // let temp_to_humid = parse_map(sections[6]);
  // let humid_to_loc = parse_map(sections[7]);

  let mut locations = Vec::new();
  for seed in seeds {
    let mut i = 1;
    let mut val = seed.clone();
    for map in maps.clone() {
      let old_val = val.clone();
      val = find_mapping(val, map);
      println!("{}. mapped {} to {}", i, old_val, val);
      i += 1;
    }
    locations.push(val);
  }

  let mut min_loc = locations.remove(0);
  for loc in locations {
    if loc < min_loc {
      min_loc = loc;
    }
  }

  println!("Part 1: {}\nPart 2: {}", min_loc, 0);
}

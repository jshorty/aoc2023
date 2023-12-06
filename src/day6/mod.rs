fn victory_options(race: (i32, i32)) -> Vec<i32> {
  let mut options = Vec::new();
  for s in 1..race.0 {

    if (race.0 - s) * s > race.1 {
      options.push(s);
    }
  }
  return options;
}

pub fn day6() {
  let races = [(42, 308), (89, 1170), (91, 1291), (89, 1467)];
  let options = races.map( |r| victory_options(r) );
  let mut options_product = 1 as i32;
  for options_set in options {
    options_product *= options_set.len() as i32;
  }

  println!("Part 1: {}\nPart 2: {}", options_product, 0);
}

fn victory_options(race: (i64, i64)) -> i64 {
  println!("Evaluating {} loops", race.0);
  let mut options = 0;
  for s in 1..race.0 {
    if (race.0 - s) * s > race.1 {
      options += 1
    }
  }
  return options;
}

pub fn day6() {
  let races = [(42, 308), (89, 1170), (91, 1291), (89, 1467)];
  let one_big_race = (42899189, 308117012911467);

  let options = races.map( |r| victory_options(r) );
  let mut options_product = 1;
  for options_set in options {
    options_product *= options_set;
  }

  let big_race_options = victory_options(one_big_race);

  println!("Part 1: {}\nPart 2: {}", options_product, big_race_options);
}

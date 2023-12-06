fn victory_options(race: (i64, i64)) -> i64 {
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
  println!("Part 1: {}", races.map( |r| victory_options(r) ).iter().product::<i64>());
  println!("Part 2: {}", victory_options(one_big_race));
}

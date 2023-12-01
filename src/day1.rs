use csv::Reader;
use csv::StringRecordsIter;

fn read_input() {
  let mut rdr = Reader::from_path("../input/day1.csv");
  rdr.records()
}

fn find_calibration_value(line: str) {
  let val = "";
  let iter = line.chars();
  val.push(iter.next());
  val.push(iter.last());
  val.parse().unwrap();
}

fn sum_calibration_values<'a>(lines: &'a StringRecordsIter<'a, R: 'a>) {
  let mut sum;
  for line in lines {
    sum = sum + find_calibration_value(line);
  }
  sum;
}

fn day1() {
  sum_calibration_values(read_input())
}

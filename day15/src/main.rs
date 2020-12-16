use std::collections::HashMap;

fn main() {
  let before = std::time::Instant::now();
  let args: Vec<String> = std::env::args().collect();
  if args.len() < 2 {
    panic!("Not enough arguments");
  }
  let before_parsing = std::time::Instant::now();
  let data: Vec<u64> = std::fs::read_to_string(&args[1])
    .unwrap()
    .lines()
    .next()
    .unwrap()
    .split(',')
    .map(|n| n.parse().unwrap())
    .collect();
  let parsing_time = before_parsing.elapsed();

  let before_p1 = std::time::Instant::now();
  let p1 = get_nth(&data, 2020);
  let p1_time = before_p1.elapsed();

  let before_p2 = std::time::Instant::now();
  let p2 = get_nth(&data, 30000000);
  let p2_time = before_p2.elapsed();

  let total_time = before.elapsed();

  println!("Parsing: elapsed time: {:.2?}", parsing_time);
  println!("Part1: {:>4} | elapsed time: {:.2?}", p1, p1_time);
  println!("Part2: {:>4} | elapsed time: {:.2?}", p2, p2_time,);
  println!("Total elapsed time: {:.2?}", total_time);
  println!("Print elapsed time: {:.2?}", before.elapsed());
}

fn get_nth(data: &Vec<u64>, nth: u64) -> u64 {
  let mut last_occurrence = HashMap::<u64, u64>::new();
  let mut last_number = data[0];
  let mut round = 1;
  for n in &data[1..] {
    last_occurrence.insert(last_number, round);
    last_number = *n;
    round += 1;
  }
  while round < nth {
    let last_round = *last_occurrence.get(&last_number).unwrap_or(&round);
    last_occurrence.insert(last_number, round);
    last_number = round - last_round;
    round += 1;
  }
  last_number
}

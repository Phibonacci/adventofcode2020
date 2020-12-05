fn main() {
  let before = std::time::Instant::now();
  let args: Vec<String> = std::env::args().collect();
  if args.len() < 2 {
    panic!("Not enough arguments");
  }
  let filename = &args[1];
  println!("Loading file {}", filename);
  let data = parse_file(filename);
  let mut sits = part1(&data);
  part2(&mut sits);
  println!("Total elapsed time: {:.2?}", before.elapsed());
}

fn parse_file(filename: impl AsRef<std::path::Path>) -> Vec<String> {
  let before = std::time::Instant::now();

  let file = std::fs::File::open(filename).expect("File not found");
  let buf = std::io::BufReader::new(file);
  use std::io::prelude::*;
  let content = buf
    .lines()
    .map(|l| l.expect("Could not parse line"))
    .collect();
  println!("Parsing: elapsed time: {:.2?}", before.elapsed());
  content
}

fn part1(data: &Vec<String>) -> Vec<u32> {
  let before = std::time::Instant::now();
  let sits_list: Vec<u32> = data.iter().map(|id| parse_id(id)).collect();
  let max = sits_list
    .iter()
    .fold(0, |acc, value| if acc > *value { acc } else { *value });
  println!(
    "Result part1: {} | elapsed time: {:.2?}",
    max,
    before.elapsed()
  );
  sits_list
}

struct SpotRanges {
  row: (u32, u32),
  column: (u32, u32),
}

impl Default for SpotRanges {
  fn default() -> SpotRanges {
    SpotRanges {
      row: (0, 127),
      column: (0, 7),
    }
  }
}

fn parse_id(id: &String) -> u32 {
  let spot = id
    .chars()
    .fold((0, 0), |position, step| parse_step(position, step));
  spot.0 * 8 + spot.1
}

fn parse_step(position: (u32, u32), step: char) -> (u32, u32) {
  match step {
    'F' => (position.0 << 1, position.1),
    'B' => (position.0 << 1 | 1, position.1),
    'L' => (position.0, position.1 << 1),
    'R' => (position.0, position.1 << 1 | 1),
    _ => unreachable!(),
  }
}

fn part2(sits: &mut Vec<u32>) -> () {
  let before = std::time::Instant::now();
  sits.sort_unstable();
  let mut previous_sit = sits[0];
  for sit in sits {
    if previous_sit + 1 < *sit {
      println!(
        "Result part1: {} | elapsed time: {:.2?}",
        previous_sit + 1,
        before.elapsed()
      );
      return;
    }
    previous_sit = *sit;
  }
  panic!("There is no sit left! You will have to leave Dr. Dao, please follow us.")
}

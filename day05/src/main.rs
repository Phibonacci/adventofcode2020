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
    .fold(SpotRanges::default(), |range, step| parse_step(range, step));
  spot.row.0 * 8 + spot.column.0
}

fn parse_step(range: SpotRanges, step: char) -> SpotRanges {
  match step {
    'F' => SpotRanges {
      row: (range.row.0, (range.row.1 - range.row.0) / 2 + range.row.0),
      column: range.column,
    },
    'B' => SpotRanges {
      row: (
        (range.row.1 - range.row.0) / 2 + range.row.0 + 1,
        range.row.1,
      ),
      column: range.column,
    },
    'L' => SpotRanges {
      row: range.row,
      column: (
        range.column.0,
        (range.column.1 - range.column.0) / 2 + range.column.0,
      ),
    },
    'R' => SpotRanges {
      row: range.row,
      column: (
        (range.column.1 - range.column.0) / 2 + range.column.0 + 1,
        range.column.1,
      ),
    },
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
  panic!("There is not sit left! You will have to leave Dr. Dao, please follow us.")
}

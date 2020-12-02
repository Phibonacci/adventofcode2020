#[macro_use]
extern crate lazy_static;
extern crate regex;

fn main() {
  let before = std::time::Instant::now();
  let args: Vec<String> = std::env::args().collect();
  if args.len() < 2 {
    panic!("Not enough arguments");
  }
  let filename = &args[1];
  println!("Loading file {}", filename);
  let data = parse_file(filename);
  part1(&data);
  part2(&data);
  println!("Total elapsed time: {:.2?}", before.elapsed());
}

struct Entry {
  min: u32,
  max: u32,
  character: char,
  password: String,
}

fn parse_file(filename: impl AsRef<std::path::Path>) -> Vec<Entry> {
  let before = std::time::Instant::now();

  let file = std::fs::File::open(filename).expect("File not found");
  let buf = std::io::BufReader::new(file);
  use std::io::prelude::*;
  let content = buf
    .lines()
    .map(|l| l.expect("Could not parse line"))
    .map(|l| parse_line(&l))
    .collect();
  println!("Parsing: elapsed time: {:.2?}", before.elapsed());
  content
}

fn parse_line(line: &String) -> Entry {
  use regex::Regex;
  lazy_static! {
    static ref PASSWORD_REGEX: Regex =
      Regex::new(r"(?P<min>\d*)-(?P<max>\d*) (?P<character>-?[a-z]): (?P<password>-?.*)").unwrap();
  }
  let coordinates_caps = PASSWORD_REGEX.captures(line).unwrap();
  Entry {
    min: coordinates_caps["min"]
      .parse::<u32>()
      .expect("Could not parse min value"),
    max: coordinates_caps["max"]
      .parse::<u32>()
      .expect("Could not parse max value"),
    character: coordinates_caps["character"]
      .parse::<char>()
      .expect("Could not parse password character"),
    password: coordinates_caps["password"]
      .parse::<String>()
      .expect("Could not parse password string"),
  }
}

fn part1(data: &Vec<Entry>) -> () {
  let before = std::time::Instant::now();
  let valid_count = data
    .iter()
    .filter(|entry| valid_sled_rental_password(entry))
    .count();
  println!(
    "Result part1: {} | elapsed time: {:.2?}",
    valid_count,
    before.elapsed()
  );
}

fn valid_sled_rental_password(entry: &Entry) -> bool {
  let count = entry
    .password
    .chars()
    .filter(|c| c == &entry.character)
    .count();
  count >= entry.min as usize && count <= entry.max as usize
}

fn part2(data: &Vec<Entry>) -> () {
  let before = std::time::Instant::now();
  let valid_count = data
    .iter()
    .filter(|entry| valid_toboggan_password(entry))
    .count();
  println!(
    "Result part2: {} | elapsed time: {:.2?}",
    valid_count,
    before.elapsed()
  );
}

fn valid_toboggan_password(entry: &Entry) -> bool {
  let min_has_char = has_char_at_position(&entry.password, entry.character, entry.min);
  let max_has_char = has_char_at_position(&entry.password, entry.character, entry.max);
  (min_has_char && !max_has_char) || (!min_has_char && max_has_char)
}

fn has_char_at_position(string: &String, c: char, position: u32) -> bool {
  position > 0
    && string.chars().count() >= position as usize
    && string.as_bytes()[(position - 1) as usize] as char == c
}

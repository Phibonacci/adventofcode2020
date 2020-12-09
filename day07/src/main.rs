#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

fn main() {
  let before = std::time::Instant::now();
  let args: Vec<String> = std::env::args().collect();
  if args.len() < 2 {
    panic!("Not enough arguments");
  }
  let filename = &args[1];
  println!("Loading file {}", filename);
  let buf = std::fs::read_to_string(filename).unwrap();
  let data = parse_file(buf);
  println!(
    "Part1: {}",
    data
      .iter()
      .filter(|key| { has_shiny(&data, key.0) })
      .count()
  );
  println!("Part2: {}", count(&data, &"shiny gold"));
  println!("Total elapsed time: {:.2?}", before.elapsed());
}

fn parse_file(buf: String) -> HashMap<String, Vec<(u32, String)>> {
  let before = std::time::Instant::now();
  lazy_static! {
    static ref REGEX: regex::Regex = regex::Regex::new(r"(\d*)? ?([a-z]* [a-z]*) bag").unwrap();
  }
  let result = buf
    .lines()
    .map(|l| {
      let mut v: Vec<(u32, String)> = REGEX
        .captures_iter(l)
        .filter_map(|group| {
          if &group[2] == "no other" {
            None
          } else {
            Some((group[1].parse::<u32>().unwrap_or(0), group[2].to_string()))
          }
        })
        .collect();
      if v.len() == 0 {
        println!("v empty")
      }
      let key = v.remove(0).1;
      (key, v)
    })
    .collect();
  println!("Parsing: elapsed time: {:.2?}", before.elapsed());
  result
}

fn has_shiny(bags: &HashMap<String, Vec<(u32, String)>>, key: &str) -> bool {
  bags[key]
    .iter()
    .any(|bag| bag.1 == "shiny gold" || has_shiny(bags, &bag.1))
}

fn count(bags: &HashMap<String, Vec<(u32, String)>>, key: &str) -> u32 {
  bags[key]
    .iter()
    .map(|bag| bag.0 + bag.0 * count(bags, &bag.1))
    .sum()
}

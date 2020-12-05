#[macro_use]
extern crate lazy_static;

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

fn parse_file(
  filename: impl AsRef<std::path::Path>,
) -> Vec<std::collections::HashMap<String, String>> {
  let before = std::time::Instant::now();

  let file = std::fs::File::open(filename).expect("File not found");
  let buf = std::io::BufReader::new(file);
  use std::io::prelude::*;
  let mut passeports = Vec::new();
  for line in buf.lines().map(|l| l.expect("Could not parse line")) {
    fold_lines(&mut passeports, parse_line(&line));
  }
  println!("Parsing: elapsed time: {:.2?}", before.elapsed());
  passeports
}

fn fold_lines(
  passeports: &mut Vec<std::collections::HashMap<String, String>>,
  entry: std::collections::HashMap<String, String>,
) {
  if passeports.len() == 0 || entry.len() == 0 {
    passeports.push(entry);
  } else {
    passeports.last_mut().unwrap().extend(entry);
  }
}

fn parse_line(line: &String) -> std::collections::HashMap<String, String> {
  if line.len() == 0 {
    return std::collections::HashMap::new();
  }
  use regex::Regex;
  lazy_static! {
    static ref CREDENTIAL_REGEX: Regex =
      Regex::new(r"(?P<key>[a-z]{3}):(?P<value>[a-z,#,\d]+)").unwrap();
  }
  let credential_capture = CREDENTIAL_REGEX.captures_iter(line);
  credential_capture
    .map(|credential| {
      (
        credential["key"].to_string(),
        credential["value"].to_string(),
      )
    })
    .collect::<std::collections::HashMap<_, _>>()
}

fn part1(data: &Vec<std::collections::HashMap<String, String>>) -> () {
  let before = std::time::Instant::now();
  println!(
    "Result part1: {} | elapsed time: {:.2?}",
    data.iter().filter(|p| valid_passport_fields(p)).count(),
    before.elapsed()
  );
}

fn valid_passport_fields(p: &std::collections::HashMap<String, String>) -> bool {
  p.len() == 8 && p.contains_key("cid")
    || (p.len() == 7 && !p.contains_key("cid"))
      && p.contains_key("byr")
      && p.contains_key("iyr")
      && p.contains_key("eyr")
      && p.contains_key("hgt")
      && p.contains_key("hcl")
      && p.contains_key("ecl")
      && p.contains_key("pid")
}

fn part2(data: &Vec<std::collections::HashMap<String, String>>) -> () {
  let before = std::time::Instant::now();
  println!(
    "Result part1: {} | elapsed time: {:.2?}",
    data
      .iter()
      .filter(|p| valid_passport_fields(p))
      .filter(|p| valid_passport_values(p))
      .count(),
    before.elapsed()
  );
}

fn valid_passport_values(passport: &std::collections::HashMap<String, String>) -> bool {
  for field in passport {
    match field.0.as_str() {
      "byr" => {
        if !valid_byr(field.1) {
          return false;
        }
      }
      "iyr" => {
        if !valid_iyr(field.1) {
          return false;
        }
      }
      "eyr" => {
        if !valid_eyr(field.1) {
          return false;
        }
      }
      "hgt" => {
        if !valid_hgt(field.1) {
          return false;
        }
      }
      "hcl" => {
        if !valid_hcl(field.1) {
          return false;
        }
      }
      "ecl" => {
        if !valid_ecl(field.1) {
          return false;
        }
      }
      "pid" => {
        if !valid_pid(field.1) {
          return false;
        }
      }
      "cid" => {
        if !valid_cid(field.1) {
          return false;
        }
      }
      _ => unreachable!(),
    }
  }
  true
}

fn valid_byr(value: &String) -> bool {
  vali_d_ate(value, 1920, 2002)
}

fn valid_iyr(value: &String) -> bool {
  vali_d_ate(value, 2010, 2020)
}

fn valid_eyr(value: &String) -> bool {
  vali_d_ate(value, 2020, 2030)
}

fn vali_d_ate(value: &String, min: u32, max: u32) -> bool {
  use regex::Regex;
  lazy_static! {
    static ref REGEX: Regex = Regex::new(r"^\d{4}$").unwrap();
  }
  if !REGEX.is_match(value) {
    return false;
  }
  let date: u32 = value.parse().unwrap();
  date >= min && date <= max
}

fn valid_hgt(value: &String) -> bool {
  use regex::Regex;
  lazy_static! {
    static ref REGEX: Regex = Regex::new(r"^(?P<value>\d*)(?P<type>cm|in)$").unwrap();
  }
  return match REGEX.captures(value) {
    Some(captures) => {
      let length: u32 = captures["value"].parse().unwrap();
      match &captures["type"] {
        "cm" => length >= 150 && length <= 193,
        "in" => length >= 59 && length <= 76,
        _ => unreachable!(),
      }
    }
    None => false,
  };
}

fn valid_hcl(value: &String) -> bool {
  use regex::Regex;
  lazy_static! {
    static ref REGEX: Regex = Regex::new(r"^#(?:\d|[a-f]){6}$").unwrap();
  }
  REGEX.is_match(value)
}

fn valid_ecl(value: &String) -> bool {
  use regex::Regex;
  lazy_static! {
    static ref REGEX: Regex = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
  }
  REGEX.is_match(value)
}

fn valid_pid(value: &String) -> bool {
  use regex::Regex;
  lazy_static! {
    static ref REGEX: Regex = Regex::new(r"^\d{9}$").unwrap();
  }
  REGEX.is_match(value)
}

fn valid_cid(_value: &String) -> bool {
  true
}

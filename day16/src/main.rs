use lazy_static::lazy_static;
use std::collections::HashMap;

fn main() {
  let before = std::time::Instant::now();
  let args: Vec<String> = std::env::args().collect();
  if args.len() < 2 {
    panic!("Not enough arguments");
  }
  let data = std::fs::read_to_string(&args[1]).unwrap();
  let before_parsing = std::time::Instant::now();
  let ticket_infos = parse_data(&data);
  let parsing_time = before_parsing.elapsed();

  let before_p1 = std::time::Instant::now();
  let p1 = part1(&ticket_infos);
  let p1_time = before_p1.elapsed();

  let before_p2 = std::time::Instant::now();
  let p2 = part2(&ticket_infos);
  let p2_time = before_p2.elapsed();

  let total_time = before.elapsed();

  println!("Parsing: elapsed time: {:.2?}", parsing_time);
  println!("Part1: {:>4} | elapsed time: {:.2?}", p1, p1_time);
  println!("Part2: {:>4} | elapsed time: {:.2?}", p2, p2_time,);
  println!("Total elapsed time: {:.2?}", total_time);
  println!("Print elapsed time: {:.2?}", before.elapsed());
}

#[derive(Default)]
struct TicketInfos {
  ticket_rules: HashMap<String, Vec<(u64, u64)>>,
  our_ticket: Vec<u64>,
  tickets: Vec<Vec<u64>>,
}

impl TicketInfos {
  fn new() -> Self {
    Default::default()
  }
}

fn parse_data(data: &str) -> TicketInfos {
  let mut ticket_infos = TicketInfos::new();
  let mut section = 0;
  for line in data.lines() {
    match line {
      "" => (),
      "your ticket:" => section = 1,
      "nearby tickets:" => section = 2,
      _ => parse_line(section, line, &mut ticket_infos),
    }
  }
  ticket_infos
}

fn parse_line(section: u64, line: &str, ticket_infos: &mut TicketInfos) -> () {
  match section {
    0 => {
      let entry = parse_rule(line);
      ticket_infos.ticket_rules.insert(entry.0, entry.1);
    }
    1 => ticket_infos.our_ticket = parse_ticket(line),
    2 => ticket_infos.tickets.push(parse_ticket(line)),
    _ => unreachable!(),
  }
}

fn parse_rule(line: &str) -> (String, Vec<(u64, u64)>) {
  use regex::Regex;
  lazy_static! {
    static ref REGEX_RULE: Regex = Regex::new(r"^([a-z, ]+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
  }
  let captures = REGEX_RULE.captures(line).unwrap();
  (
    captures[1].to_string(),
    vec![
      (captures[2].parse().unwrap(), captures[3].parse().unwrap()),
      (captures[4].parse().unwrap(), captures[5].parse().unwrap()),
    ],
  )
}

fn parse_ticket(line: &str) -> Vec<u64> {
  line.split(',').map(|n| n.parse().unwrap()).collect()
}

fn part1(infos: &TicketInfos) -> u64 {
  let mut result = 0;
  for ticket in &infos.tickets {
    for value in ticket {
      if !is_valid_value(*value, &infos.ticket_rules) {
        result += value;
      }
    }
  }
  result
}

fn part2(infos: &TicketInfos) -> u64 {
  let mut field_rules = init_field_rules(&infos.ticket_rules, infos.our_ticket.len());
  for ticket in infos
    .tickets
    .iter()
    .chain(std::iter::once(&infos.our_ticket))
  {
    if !is_ticket_valid(ticket, &infos.ticket_rules) {
      continue;
    }
    remove_invalid_rules(ticket, &mut field_rules, &infos.ticket_rules)
  }

  let mut to_filter: Vec<&str> = infos.ticket_rules.keys().map(|r| r.as_str()).collect();
  let mut clean = true;
  while clean {
    clean = false;
    let mut unique_rule = "".to_string();
    for rules in field_rules.values() {
      if rules.len() == 1 && to_filter.contains(rules.first().unwrap()) {
        unique_rule = rules.first().unwrap().to_string();
        to_filter.retain(|r| r != rules.first().unwrap());
      }
    }
    for rules in field_rules.values_mut() {
      if rules.len() > 1 {
        rules.retain(|r| *r != unique_rule);
        if rules.len() > 1 {
          clean = true;
        }
      }
    }
  }

  infos
    .our_ticket
    .iter()
    .enumerate()
    .filter(|value| {
      field_rules[&(value.0 as u64)]
        .iter()
        .any(|rule| rule.contains("departure"))
    })
    .fold(1, |acc, value| value.1 * acc)
}

fn remove_invalid_rules(
  ticket: &Vec<u64>,
  field_rules: &mut HashMap<u64, Vec<&str>>,
  ticket_rules: &HashMap<String, Vec<(u64, u64)>>,
) {
  for value in ticket.into_iter().enumerate() {
    field_rules
      .get_mut(&(value.0 as u64))
      .unwrap()
      .retain(|rule| validate_rule(*value.1, &ticket_rules[*rule]));
  }
}

fn init_field_rules(
  rules: &HashMap<String, Vec<(u64, u64)>>,
  fields_count: usize,
) -> HashMap<u64, Vec<&str>> {
  let mut field_rules = HashMap::new();
  let all_rules: Vec<&str> = rules.keys().map(|k| k.as_str()).collect();
  for i in 0..fields_count {
    field_rules.insert(i as u64, all_rules.clone());
  }
  field_rules
}

fn is_ticket_valid(ticket: &Vec<u64>, rules: &HashMap<String, Vec<(u64, u64)>>) -> bool {
  for value in ticket {
    if !is_valid_value(*value, rules) {
      return false;
    }
  }
  true
}

fn is_valid_value(value: u64, rules: &HashMap<String, Vec<(u64, u64)>>) -> bool {
  for rule in rules.values() {
    if validate_rule(value, rule) {
      return true;
    }
  }
  false
}

fn validate_rule(value: u64, ranges: &Vec<(u64, u64)>) -> bool {
  for range in ranges {
    if is_in_range(value, range) {
      return true;
    }
  }
  false
}

fn is_in_range(value: u64, range: &(u64, u64)) -> bool {
  value >= range.0 && value <= range.1
}

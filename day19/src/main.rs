fn main() {
  let before = std::time::Instant::now();
  let args: Vec<String> = std::env::args().collect();
  if args.len() < 2 {
    panic!("Not enough arguments");
  }
  let data = std::fs::read_to_string(&args[1]).unwrap();
  let before_parsing = std::time::Instant::now();
  let (rules, messages) = parse_data(&data);
  let parsing_time = before_parsing.elapsed();

  let before_p1 = std::time::Instant::now();
  let p1 = part1(&rules, &messages);
  let p1_time = before_p1.elapsed();

  let before_p2 = std::time::Instant::now();
  let p2 = part2(&rules, &messages);
  let p2_time = before_p2.elapsed();

  let total_time = before.elapsed();

  println!("Parsing: elapsed time: {:.2?}", parsing_time);
  println!("Part1: {:>4} | elapsed time: {:.2?}", p1, p1_time);
  println!("Part2: {:>4} | elapsed time: {:.2?}", p2, p2_time,);
  println!("Total elapsed time: {:.2?}", total_time);
  println!("Print elapsed time: {:.2?}", before.elapsed());
}

fn parse_data(data: &str) -> (Vec<Vec<Rule>>, Vec<&str>) {
  let mut rules = Vec::<Vec<Rule>>::new();
  let mut messages = Vec::new();
  let mut section = 0;
  for line in data.lines() {
    if line.is_empty() {
      section = 1;
      continue;
    }
    if section == 0 {
      let main_split = line.split(':').collect::<Vec<&str>>();
      let pos: usize = main_split[0].parse().unwrap();
      if pos >= rules.len() {
        rules.resize_with(pos + 1, Default::default);
      }
      rules[pos] = parse_rule(main_split[1]);
    } else {
      messages.push(line);
    }
  }
  (rules, messages)
}

fn parse_rule(rule_string: &str) -> Vec<Rule> {
  rule_string
    .split('|')
    .collect::<Vec<&str>>()
    .iter()
    .map(|r| parse_sub_rule(r))
    .collect()
}

fn parse_sub_rule(rule_string: &str) -> Rule {
  let first_quote = rule_string.find('"');
  if first_quote != None {
    Rule::CHAR(rule_string.chars().nth(first_quote.unwrap() + 1).unwrap())
  } else {
    Rule::RULES(
      rule_string
        .split(' ')
        .collect::<Vec<&str>>()
        .iter()
        .filter(|r| **r != "")
        .map(|r| r.parse().unwrap())
        .collect(),
    )
  }
}

#[derive(Clone)]
enum Rule {
  RULES(Vec<usize>),
  CHAR(char),
}

fn part1(rules: &Vec<Vec<Rule>>, messages: &Vec<&str>) -> usize {
  messages
    .iter()
    .filter(|m| {
      check_rules(rules, **m, &vec![0], 0)
        .iter()
        .any(|index| *index == m.len())
    })
    .count()
}

fn part2(rules: &Vec<Vec<Rule>>, messages: &Vec<&str>) -> usize {
  let mut rules_updated = rules.clone();
  rules_updated[8] = vec![Rule::RULES(vec![42]), Rule::RULES(vec![42, 8])];
  rules_updated[11] = vec![Rule::RULES(vec![42, 31]), Rule::RULES(vec![42, 11, 31])];
  messages
    .iter()
    .filter(|m| {
      check_rules(&rules_updated, **m, &vec![0], 0)
        .iter()
        .any(|index| *index == m.len())
    })
    .count()
}

fn check_rules(
  rules: &Vec<Vec<Rule>>,
  message: &str,
  message_indexes: &Vec<usize>,
  rule_id: usize,
) -> Vec<usize> {
  let mut result = Vec::new();
  for message_index in message_indexes {
    for rule in &rules[rule_id] {
      match rule {
        Rule::CHAR(c) => match message.chars().nth(*message_index) {
          Some(current_char) => {
            if current_char == *c {
              result.push(message_index + 1);
            }
          }
          None => (),
        },
        Rule::RULES(rule_option) => {
          for index in check_sub_rule(rules, rule_option, 0, message, &vec![*message_index]) {
            result.push(index);
          }
        }
      }
    }
  }
  result
}

fn check_sub_rule(
  rules: &Vec<Vec<Rule>>,
  sub_rules: &Vec<usize>,
  sub_rule_id: usize,
  message: &str,
  message_indexes: &Vec<usize>,
) -> Vec<usize> {
  if sub_rule_id >= sub_rules.len() {
    return message_indexes.clone();
  }
  let indexes = check_rules(rules, message, &message_indexes, sub_rules[sub_rule_id]);
  let result = check_sub_rule(rules, sub_rules, sub_rule_id + 1, message, &indexes);
  result
}

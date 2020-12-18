use std::collections::VecDeque;

fn main() {
  let before = std::time::Instant::now();
  let args: Vec<String> = std::env::args().collect();
  if args.len() < 2 {
    panic!("Not enough arguments");
  }
  let data = std::fs::read_to_string(&args[1]).unwrap();
  let before_parsing = std::time::Instant::now();
  let expressions: Vec<Vec<char>> = data
    .lines()
    .map(|l| l.chars().filter(|c| *c != ' ').collect())
    .collect();
  let parsing_time = before_parsing.elapsed();

  let before_p1 = std::time::Instant::now();
  let p1 = part1(&expressions);
  let p1_time = before_p1.elapsed();

  let before_p2 = std::time::Instant::now();
  let p2 = part2(&expressions);
  let p2_time = before_p2.elapsed();

  let total_time = before.elapsed();

  println!("Parsing: elapsed time: {:.2?}", parsing_time);
  println!("Part1: {:>4} | elapsed time: {:.2?}", p1, p1_time);
  println!("Part2: {:>4} | elapsed time: {:.2?}", p2, p2_time,);
  println!("Total elapsed time: {:.2?}", total_time);
  println!("Print elapsed time: {:.2?}", before.elapsed());
}

fn part1(expressions: &Vec<Vec<char>>) -> u64 {
  expressions.iter().map(|e| calculate(e, false)).sum()
}

fn part2(expressions: &Vec<Vec<char>>) -> u64 {
  expressions.iter().map(|e| calculate(e, true)).sum()
}

fn calculate(expression: &Vec<char>, precedence: bool) -> u64 {
  let rpn = reverse_polish_notation(expression, precedence);
  let mut number_stack = Vec::new();
  for c in rpn {
    if c == '+' || c == '*' {
      let r = number_stack.pop().unwrap();
      let l = number_stack.pop().unwrap();
      match c {
        '+' => number_stack.push(r + l),
        '*' => number_stack.push(r * l),
        _ => (),
      }
    } else {
      number_stack.push(c as u64 - '0' as u64);
    }
  }
  number_stack.pop().unwrap()
}

fn reverse_polish_notation(expression: &Vec<char>, precedence: bool) -> VecDeque<char> {
  let mut output = VecDeque::new();
  let mut operators = Vec::new();
  for token in expression {
    if *token as u64 >= '0' as u64 && *token as u64 <= '9' as u64 {
      output.push_front(*token);
    } else if *token == '+' || *token == '*' {
      while !operators.is_empty()
        && *operators.last().unwrap() != '('
        && (!precedence
          || *operators.last().unwrap() == *token
          || *operators.last().unwrap() == '+')
      {
        let op = operators.pop().unwrap();
        output.push_front(op);
      }
      operators.push(*token);
    } else if *token == '(' {
      operators.push(*token);
    } else if *token == ')' {
      while *operators.last().unwrap() != '(' {
        let op = operators.pop().unwrap();
        output.push_front(op);
      }
      if !operators.is_empty() && *operators.last().unwrap() == '(' {
        operators.pop();
      }
    }
  }
  while !operators.is_empty() {
    let op = operators.pop().unwrap();
    output.push_front(op);
  }
  output.iter().rev().cloned().collect()
}

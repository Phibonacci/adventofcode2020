use std::collections::HashSet;

fn main() {
  let before_p1 = std::time::Instant::now();
  let args: Vec<String> = std::env::args().collect();
  if args.len() < 2 {
    panic!("Not enough arguments");
  }
  let buffer = std::fs::read_to_string(&args[1]).unwrap();
  let declarations: Vec<&str> = buffer.split("\n\n").collect();

  let p1: usize = declarations
    .iter()
    .map(|group| count(group, |a, b| a | b))
    .sum();
  println!("Part1: {} |  elapsed time: {:.2?}", p1, before_p1.elapsed());
  let before_p2 = std::time::Instant::now();
  let p2: usize = declarations
    .iter()
    .map(|group| count(group, |a, b| a & b))
    .sum();
  println!("Part2: {} |  elapsed time: {:.2?}", p2, before_p2.elapsed());
}

fn count(group: &str, op: fn(&HashSet<char>, &HashSet<char>) -> HashSet<char>) -> usize {
  let lines: Vec<&str> = group.lines().collect();
  lines
    .iter()
    .fold(lines[0].chars().collect(), |acc: HashSet<char>, current| {
      op(&current.chars().into_iter().collect(), &acc)
    })
    .len()
}

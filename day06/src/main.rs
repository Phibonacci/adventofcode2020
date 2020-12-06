fn main() {
  let before_p1 = std::time::Instant::now();
  let args: Vec<String> = std::env::args().collect();
  if args.len() < 2 {
    panic!("Not enough arguments");
  }
  let declarations: Vec<Vec<String>> = std::fs::read_to_string(&args[1])
    .expect(&format!("File \"{}\" not found", &args[1]))
    .split("\n\n")
    .map(|l| l.to_string())
    .map(|s| s.lines().map(|l| l.to_string()).collect())
    .collect();

  let p1 = count(&declarations, false);
  println!("Part1: {} |  elapsed time: {:.2?}", p1, before_p1.elapsed());
  let before_p2 = std::time::Instant::now();
  let p2 = count(&declarations, true);
  println!("Part2: {} |  elapsed time: {:.2?}", p2, before_p2.elapsed());
}

fn count(declarations: &Vec<Vec<String>>, unanimous: bool) -> usize {
  declarations
    .iter()
    .map(|group| {
      group
        .iter()
        .fold(
          group[0].chars().collect(),
          |acc: std::collections::HashSet<char>, current| {
            current
              .chars()
              .into_iter()
              .chain(acc.clone())
              .filter(|c| !unanimous || (acc.contains(c) && current.contains(*c)))
              .collect()
          },
        )
        .len()
    })
    .sum()
}

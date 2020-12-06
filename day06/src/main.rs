fn main() {
  let before_p1 = std::time::Instant::now();
  let args: Vec<String> = std::env::args().collect();
  if args.len() < 2 {
    panic!("Not enough arguments");
  }
  let filename = &args[1];
  let declarations: Vec<String> = std::fs::read_to_string(filename)
    .expect(&format!("File \"{}\" not found", filename))
    .split("\n\n")
    .map(|l| l.to_string())
    .collect();

  let p1 = declarations
    .iter()
    .map(|s| s.chars().into_iter().filter(|c| *c != '\n').collect())
    .fold(0, |total, group: std::collections::HashSet<char>| {
      total + group.len()
    });
  println!("Part1: {} |  elapsed time: {:.2?}", p1, before_p1.elapsed());
  let before_p2 = std::time::Instant::now();
  let p2 = declarations
    .iter()
    .map(|s| s.lines().map(|l| l.to_string()).collect())
    .fold(0, |sum, group: Vec<String>| {
      group
        .iter()
        .fold(
          group[0].chars().collect(),
          |unanimous: Vec<char>, current| {
            current
              .chars()
              .into_iter()
              .filter(|c| unanimous.contains(c))
              .collect()
          },
        )
        .len()
        + sum
    });
  println!("Part2: {} |  elapsed time: {:.2?}", p2, before_p2.elapsed());
}

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

#[derive(PartialEq, Eq)]
enum Tile {
  Tree,
  Open,
}

fn parse_file(filename: impl AsRef<std::path::Path>) -> Vec<Vec<Tile>> {
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

fn parse_line(line: &String) -> Vec<Tile> {
  line
    .chars()
    .filter(|c| *c == '.' || *c == '#')
    .map(|c| if c == '.' { Tile::Open } else { Tile::Tree })
    .collect()
}

fn part1(data: &Vec<Vec<Tile>>) -> () {
  let before = std::time::Instant::now();
  println!(
    "Result part1: {} | elapsed time: {:.2?}",
    slopes(data, 3, 1),
    before.elapsed()
  );
}

fn part2(data: &Vec<Vec<Tile>>) -> () {
  let before = std::time::Instant::now();

  println!(
    "Result part2: {} | elapsed time: {:.2?}",
    slopes(data, 1, 1)
      * slopes(data, 3, 1)
      * slopes(data, 5, 1)
      * slopes(data, 7, 1)
      * slopes(data, 1, 2),
    before.elapsed()
  )
}

fn slopes(data: &Vec<Vec<Tile>>, right: u32, down: u32) -> u32 {
  let mut trees = 0;
  let mut x = right as usize;
  let mut y = down as usize;
  while y < data.len() {
    if data[y][x as usize % data[y].len()] == Tile::Tree {
      trees += 1
    }
    x += right as usize;
    y += down as usize;
  }
  trees
}

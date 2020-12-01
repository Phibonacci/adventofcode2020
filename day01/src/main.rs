fn main() {
  let before = std::time::Instant::now();
  let args: Vec<String> = std::env::args().collect();
  if args.len() < 2 {
    panic!("Not enough arguments");
  }
  let filename = &args[1];
  println!("Loading file {}", filename);
  let data = parse_file(filename);
  part1(& data);
  part2(& data);
  println!("Total elapsed time: {:.2?}", before.elapsed());
}

fn parse_file(filename : impl AsRef<std::path::Path>) -> Vec<u32> {
  let before = std::time::Instant::now();
  let file = std::fs::File::open(filename).expect("File not found");
  let buf = std::io::BufReader::new(file);
  use std::io::prelude::*;
  let content = buf.lines()
      .map(|l| l.expect("Could not parse line"))
      .map(|l| l.parse::<u32>().expect("Could not parse number"))
      .collect();
  println!("Parsing: elapsed time: {:.2?}", before.elapsed());
  content
}

fn part1(data: & Vec<u32>) -> () {
  let before = std::time::Instant::now();
  for index_left_hand in 0..(data.len() - 1) {
    for index_right_hand in index_left_hand..data.len() {
      let left_hand = data[index_left_hand];
      let right_hand = data[index_right_hand];
      if left_hand + right_hand == 2020 {
        println!("Result part1: {:>10} | elapsed time: {:.2?}",
          left_hand * right_hand,
          before.elapsed());
        return
      }
    }
  }
  println!("Part2: result not found");
}

fn part2(data: & Vec<u32>) -> () {
  let before = std::time::Instant::now();
  for index_first_number in 0..(data.len() - 2) {
    for index_second_number in index_first_number..data.len() {
      for index_third_number in index_second_number..data.len() {
        let first_number = data[index_first_number];
        let second_number = data[index_second_number];
        let third_number = data[index_third_number];
        if first_number + second_number + third_number == 2020 {
          println!("Result part2: {:>10} | elapsed time: {:.2?}",
            first_number * second_number * third_number,
            before.elapsed());
          return
        }
      }
    }
  }
  println!("Part2: result not found");
}

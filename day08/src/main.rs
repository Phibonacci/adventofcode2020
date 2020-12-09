use std::collections::HashSet;

#[derive(Clone, Copy)]
enum Instruction {
  JMP,
  ACC,
  NOP,
}

fn main() {
  let before = std::time::Instant::now();
  let before_parsing = std::time::Instant::now();
  let args: Vec<String> = std::env::args().collect();
  if args.len() < 2 {
    panic!("Not enough arguments");
  }
  let buf = std::fs::read_to_string(&args[1]).unwrap();
  let mut data = parse_file(&buf);
  let parsing_time = before_parsing.elapsed();
  let before_p1 = std::time::Instant::now();
  let p1 = accumulator_value(&data).1;
  let p1_time = before_p1.elapsed();
  let before_p2 = std::time::Instant::now();
  let p2 = fix_program(&mut data);
  let p2_time = before_p2.elapsed();
  let total_time = before.elapsed();
  println!("Parsing: elapsed time: {:.2?}", parsing_time);
  println!("Part1: {:>4} | elapsed time: {:.2?}", p1, p1_time);
  println!("Part2: {:>4} | elapsed time: {:.2?}", p2, p2_time,);
  println!("Total elapsed time: {:.2?}", total_time);
  println!("Print elapsed time: {:.2?}", before.elapsed());
}

fn parse_file(buf: &String) -> Vec<(Instruction, i32)> {
  buf
    .lines()
    .map(|l| {
      let s: Vec<&str> = l.split(' ').collect();
      (
        match s[0] {
          "jmp" => Instruction::JMP,
          "acc" => Instruction::ACC,
          "nop" => Instruction::NOP,
          _ => unreachable!(),
        },
        s[1].parse::<i32>().unwrap(),
      )
    })
    .collect()
}

fn accumulator_value(data: &Vec<(Instruction, i32)>) -> (usize, i32) {
  let mut visited = HashSet::<i32>::new();
  let mut offset = 0;
  let mut accumulator = 0;
  while !visited.contains(&offset) {
    if offset as usize >= data.len() {
      break;
    }
    visited.insert(offset);
    let current = &data[offset as usize];
    match current.0 {
      Instruction::NOP => offset += 1,
      Instruction::ACC => {
        accumulator += current.1;
        offset += 1;
      }
      Instruction::JMP => offset += current.1,
    }
  }
  (offset as usize, accumulator)
}

fn fix_program(data: &mut Vec<(Instruction, i32)>) -> i32 {
  let mut accumulator = 0;
  for i in 0..data.len() {
    let current_instruction = data[i].0.clone();
    match data[i].0 {
      Instruction::JMP => data[i].0 = Instruction::NOP,
      Instruction::NOP => data[i].0 = Instruction::JMP,
      _ => (),
    }
    let result = accumulator_value(data);
    if result.0 >= data.len() {
      accumulator = result.1;
    }
    data[i].0 = current_instruction;
  }
  accumulator
}

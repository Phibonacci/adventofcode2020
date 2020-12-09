use std::collections::HashSet;

#[derive(Clone, Copy)]
enum Instruction {
  JMP,
  ACC,
  NOP,
}

fn main() {
  let before = std::time::Instant::now();
  let args: Vec<String> = std::env::args().collect();
  if args.len() < 2 {
    panic!("Not enough arguments");
  }
  let filename = &args[1];
  println!("Loading file {}", filename);
  let buf = std::fs::read_to_string(filename).unwrap();
  let mut data = parse_file(&buf);
  println!("Part1: {}", accumulator_value(&data).1);
  println!("Part2: {}", fix_program(&mut data));
  println!("Total elapsed time: {:.2?}", before.elapsed());
}

fn parse_file(buf: &String) -> Vec<(Instruction, i32)> {
  let before = std::time::Instant::now();
  let result = buf
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
    .collect();
  println!("Parsing: elapsed time: {:.2?}", before.elapsed());
  result
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
  for i in 0..data.len() {
    let current_instruction = data[i].0.clone();
    match data[i].0 {
      Instruction::JMP => data[i].0 = Instruction::NOP,
      Instruction::NOP => data[i].0 = Instruction::JMP,
      _ => (),
    }
    let result = accumulator_value(data);
    if result.0 >= data.len() {
      return result.1;
    }
    data[i].0 = current_instruction;
  }
  panic!("No valid program found");
}

use std::collections::HashMap;

fn main() {
  let before = std::time::Instant::now();
  let args: Vec<String> = std::env::args().collect();
  if args.len() < 2 {
    panic!("Not enough arguments");
  }
  let data = std::fs::read_to_string(&args[1]).unwrap();
  let before_parsing = std::time::Instant::now();
  let blocks = parse_file(&data);
  let parsing_time = before_parsing.elapsed();

  let before_p1 = std::time::Instant::now();
  let p1 = part1(&blocks);
  let p1_time = before_p1.elapsed();

  let before_p2 = std::time::Instant::now();
  let p2 = part2(&blocks);
  let p2_time = before_p2.elapsed();

  let total_time = before.elapsed();

  println!("Parsing: elapsed time: {:.2?}", parsing_time);
  println!("Part1: {:>4} | elapsed time: {:.2?}", p1, p1_time);
  println!("Part2: {:>4} | elapsed time: {:.2?}", p2, p2_time,);
  println!("Total elapsed time: {:.2?}", total_time);
  println!("Print elapsed time: {:.2?}", before.elapsed());
}

struct Block {
  mask: String,
  mem: Vec<(u64, u64)>,
}

fn parse_file(data: &str) -> Vec<Block> {
  use regex::Regex;
  let mask_regex = Regex::new(r"^mask = ([0,1,X]{36})$").unwrap();
  let write_regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();

  let mut blocks = Vec::new();
  for line in data.lines().map(|l| l.parse::<String>().unwrap()) {
    if line.contains("mask =") {
      blocks.push(Block {
        mask: mask_regex.captures(&line).unwrap()[1].to_string(),
        mem: Vec::new(),
      })
    } else {
      let write_capture = write_regex.captures(&line).unwrap();
      blocks.last_mut().unwrap().mem.push((
        write_capture[1].parse().unwrap(),
        write_capture[2].parse().unwrap(),
      ));
    }
  }
  blocks
}

fn part1(blocks: &Vec<Block>) -> u64 {
  let mut mem: HashMap<u64, u64> = HashMap::new();
  for block in blocks {
    for instruction in &block.mem {
      mem.insert(instruction.0, mask_value(&block.mask, instruction.1));
    }
  }
  mem.iter().map(|pair| pair.1).sum()
}

fn mask_value(mask: &String, value: u64) -> u64 {
  let mut shift = 0;
  let mut result = value;
  for c in mask.chars().rev() {
    match c {
      'X' => (),
      _ => result = result & !(1 << shift) | ((c as u64 - '0' as u64) << shift),
    }
    shift += 1;
  }
  result
}

fn part2(blocks: &Vec<Block>) -> u64 {
  let mut mem: HashMap<u64, u64> = HashMap::new();
  for block in blocks {
    for instruction in &block.mem {
      update_mem(&mut mem, &block.mask, instruction);
    }
  }
  mem.iter().map(|pair| pair.1).sum()
}

fn update_mem(mem: &mut HashMap<u64, u64>, mask: &str, instruction: &(u64, u64)) {
  let initial_value = mask_addr(mask, instruction.0);
  update_addr(initial_value, mem, mask, instruction.1);
}

fn update_addr(addr: u64, mem: &mut HashMap<u64, u64>, mask: &str, value: u64) {
  mem.insert(addr, value);
  let mut current_addr = addr;
  let mut looking = true;
  while looking {
    looking = false;
    for i in 0..mask.len() {
      if mask.chars().rev().nth(i).unwrap() == 'X' {
        let bit = get_bit(current_addr, i);
        if bit == 1 {
          current_addr = set_bit(current_addr, i, 0);
        } else {
          current_addr = set_bit(current_addr, i, 1);
          mem.insert(current_addr, value);
          looking = true;
          break;
        }
      }
    }
  }
}

fn get_bit(n: u64, pos: usize) -> u64 {
  (!(n & !(1 << pos)) & n) >> pos
}

fn set_bit(n: u64, pos: usize, bit: u64) -> u64 {
  n & !(1 << pos) | (bit << pos)
}

fn mask_addr(mask: &str, addr: u64) -> u64 {
  let mut shift = 0;
  let mut result = addr;
  for c in mask.chars().rev() {
    match c {
      'X' => result = result & !(1 << shift) | (0 << shift),
      '1' => result = result & !(1 << shift) | (1 << shift),
      _ => (),
    }
    shift += 1;
  }
  result
}

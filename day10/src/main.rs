fn main() {
  let before = std::time::Instant::now();
  let before_parsing = std::time::Instant::now();
  let args: Vec<String> = std::env::args().collect();
  if args.len() < 2 {
    panic!("Not enough arguments");
  }
  let mut data: Vec<u64> = std::fs::read_to_string(&args[1])
    .unwrap()
    .lines()
    .map(|l| l.parse().unwrap())
    .collect();
  data.sort();
  data.insert(0, 0);
  data.push(data.last().unwrap() + 3);
  let parsing_time = before_parsing.elapsed();

  let before_p1 = std::time::Instant::now();
  let p1 = part1(&data);
  let p1_time = before_p1.elapsed();

  let before_p2 = std::time::Instant::now();
  let p2 = part2(&data);
  let p2_time = before_p2.elapsed();

  let total_time = before.elapsed();

  println!("Parsing: elapsed time: {:.2?}", parsing_time);
  println!("Part1: {:>4} | elapsed time: {:.2?}", p1, p1_time);
  println!("Part2: {:>4} | elapsed time: {:.2?}", p2, p2_time,);
  println!("Total elapsed time: {:.2?}", total_time);
  println!("Print elapsed time: {:.2?}", before.elapsed());
}

fn part1(data: &Vec<u64>) -> u64 {
  let mut previous_number = 0;
  let mut one_jolts = 0;
  let mut three_jolts = 0;
  for number in data.iter() {
    let jolts = number - previous_number;
    if jolts == 1 {
      one_jolts += 1;
    } else if jolts == 3 {
      three_jolts += 1;
    }
    previous_number = *number;
  }
  one_jolts * three_jolts
}

fn part2(data: &Vec<u64>) -> usize {
  let mut options_ahead = vec![0; data.len() - 1];
  for i in 0..data.len() - 1 {
    for j in i + 1..data.len() {
      if data[j] - data[i] <= 3 {
        options_ahead[i] += 1;
      } else {
        break;
      }
    }
  }
  let mut i = options_ahead.len() - 2;
  loop {
    let local_paths = options_ahead[i];
    options_ahead[i] = 0;
    for j in 1..local_paths + 1 {
      options_ahead[i] += options_ahead[i + j];
    }
    if i == 0 {
      break;
    }
    i -= 1;
  }
  options_ahead[0]
}

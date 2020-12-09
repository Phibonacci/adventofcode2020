fn main() {
  let before = std::time::Instant::now();
  let before_parsing = std::time::Instant::now();
  let args: Vec<String> = std::env::args().collect();
  if args.len() < 2 {
    panic!("Not enough arguments");
  }
  let data: Vec<u64> = std::fs::read_to_string(&args[1])
    .unwrap()
    .lines()
    .map(|l| l.parse().unwrap())
    .collect();
  let parsing_time = before_parsing.elapsed();

  let before_p1 = std::time::Instant::now();
  let p1 = data
    .iter()
    .enumerate()
    .find(|i| !is_valid_number(&data, i.0))
    .unwrap()
    .1;
  let p1_time = before_p1.elapsed();

  let before_p2 = std::time::Instant::now();
  let p2 = find_contiguous_sum(&data, p1);
  let p2_time = before_p2.elapsed();

  let total_time = before.elapsed();

  println!("Parsing: elapsed time: {:.2?}", parsing_time);
  println!("Part1: {:>4} | elapsed time: {:.2?}", p1, p1_time);
  println!("Part2: {:>4} | elapsed time: {:.2?}", p2, p2_time,);
  println!("Total elapsed time: {:.2?}", total_time);
  println!("Print elapsed time: {:.2?}", before.elapsed());
}

fn is_valid_number(data: &Vec<u64>, index: usize) -> bool {
  if index < 25 {
    true
  } else {
    let current = data[index];
    for i in index - 25..index - 1 {
      for j in i + 1..index {
        if data[i] + data[j] == current {
          return true;
        }
      }
    }
    false
  }
}

fn find_contiguous_sum(data: &Vec<u64>, looking: &u64) -> u64 {
  for i in 0..data.len() - 1 {
    let mut sum = data[i];
    let mut smallest = sum;
    let mut largest = sum;
    for j in i + 1..data.len() {
      sum += data[j];
      if data[j] < smallest {
        smallest = data[j];
      } else if data[j] > largest {
        largest = data[j];
      }
      if sum == *looking {
        return smallest + largest;
      }
      if sum > *looking {
        break;
      }
    }
  }
  panic!("No contiguous sum found");
}

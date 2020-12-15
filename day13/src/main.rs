fn main() {
  let before = std::time::Instant::now();
  let before_parsing = std::time::Instant::now();
  let args: Vec<String> = std::env::args().collect();
  if args.len() < 2 {
    panic!("Not enough arguments");
  }
  let data: Vec<String> = std::fs::read_to_string(&args[1])
    .unwrap()
    .lines()
    .map(|l| l.parse().unwrap())
    .collect();
  let arriving_time: i64 = data.first().unwrap().parse().unwrap();
  let bus_list: Vec<Option<i64>> = data
    .last()
    .unwrap()
    .split(',')
    .map(|n| {
      if n == "x" {
        None
      } else {
        Some(n.parse().unwrap())
      }
    })
    .collect();
  let parsing_time = before_parsing.elapsed();

  let before_p1 = std::time::Instant::now();
  let p1 = part1(&bus_list, arriving_time);
  let p1_time = before_p1.elapsed();

  let before_p2 = std::time::Instant::now();
  let p2 = part2(&bus_list);
  let p2_time = before_p2.elapsed();

  let total_time = before.elapsed();

  println!("Parsing: elapsed time: {:.2?}", parsing_time);
  println!("Part1: {:>4} | elapsed time: {:.2?}", p1, p1_time);
  println!("Part2: {:>4} | elapsed time: {:.2?}", p2, p2_time,);
  println!("Total elapsed time: {:.2?}", total_time);
  println!("Print elapsed time: {:.2?}", before.elapsed());
}

fn part1(bus_list: &Vec<Option<i64>>, arriving_time: i64) -> i64 {
  let mut best_bus = None;
  let mut best_bus_passage = 0;
  for option_bus in bus_list {
    match option_bus {
      None => continue,
      Some(bus) => {
        let bus_passage = next_passage(arriving_time, *bus);
        if best_bus == None || bus_passage < best_bus_passage {
          best_bus = Some(bus);
          best_bus_passage = bus_passage;
        }
      }
    }
  }
  best_bus.unwrap() * (best_bus_passage - arriving_time)
}

fn next_passage(arriving_time: i64, bus_id: i64) -> i64 {
  let remainder = if arriving_time % bus_id > 0 { 1 } else { 0 };
  (arriving_time / bus_id + remainder) * bus_id
}

fn part2(bus_list: &Vec<Option<i64>>) -> i64 {
  let buses: Vec<i64> = bus_list.iter().filter_map(|b| *b).rev().collect();
  let distances = find_distances(bus_list);
  let mut passages = vec![0; buses.len()];
  let mut range = buses[0];
  for i in 0..buses.len() - 1 {
    let next_factors = find_next_factors(buses[i], buses[i + 1], distances[i], passages[i], range);
    passages[i] = next_factors.0;
    passages[i + 1] = next_factors.1;
    range *= buses[i + 1];
  }

  passages.last().unwrap() * buses.last().unwrap()
}

fn find_distances(bus_list: &Vec<Option<i64>>) -> Vec<i64> {
  let mut current_distance = 1;
  let mut distances = Vec::new();
  for bus_opt in bus_list.iter() {
    match bus_opt {
      Some(_bus) => {
        distances.push(current_distance);
        current_distance = -1
      }
      None => current_distance -= 1,
    }
  }
  distances.reverse();
  distances
}

fn find_next_factors(
  x: i64,
  y: i64,
  difference: i64,
  last_x_factor: i64,
  range: i64,
) -> (i64, i64) {
  let mut a = last_x_factor;
  let mut b = 0;
  while !is_valid(x, y, difference, a, b) {
    a += range / x;
    b = (a * x + difference) / y;
  }
  (a, b)
}

fn is_valid(x: i64, y: i64, difference: i64, x_factor: i64, y_factor: i64) -> bool {
  x_factor * x + difference == y_factor * y
}

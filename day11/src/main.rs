fn main() {
  let before = std::time::Instant::now();
  let before_parsing = std::time::Instant::now();
  let args: Vec<String> = std::env::args().collect();
  if args.len() < 2 {
    panic!("Not enough arguments");
  }
  let data: Vec<Vec<char>> = std::fs::read_to_string(&args[1])
    .unwrap()
    .lines()
    .map(|l| l.parse::<String>().unwrap().chars().collect())
    .collect();
  let parsing_time = before_parsing.elapsed();

  let before_p1 = std::time::Instant::now();
  let p1 = solve(&data, false);
  let p1_time = before_p1.elapsed();

  let before_p2 = std::time::Instant::now();
  let p2 = solve(&data, true);
  let p2_time = before_p2.elapsed();

  let total_time = before.elapsed();

  println!("Parsing: elapsed time: {:.2?}", parsing_time);
  println!("Part1: {:>4} | elapsed time: {:.2?}", p1, p1_time);
  println!("Part2: {:>4} | elapsed time: {:.2?}", p2, p2_time,);
  println!("Total elapsed time: {:.2?}", total_time);
  println!("Print elapsed time: {:.2?}", before.elapsed());
}

fn solve(data: &Vec<Vec<char>>, look_afar: bool) -> usize {
  let mut seats = data.clone();
  let mut state_changed = true;
  while state_changed {
    let previous_seats = seats.clone();
    state_changed = false;
    for y in 0..seats.len() {
      for x in 0..seats[y].len() {
        if seat_becomes_taken(&previous_seats, x, y, look_afar) {
          state_changed = true;
          seats[y][x] = '#';
        } else if seat_becomes_free(&previous_seats, x, y, look_afar) {
          state_changed = true;
          seats[y][x] = 'L';
        }
      }
    }
  }
  seats
    .iter()
    .map(|l| l.iter().filter(|seat| **seat == '#').count())
    .sum()
}

fn seat_becomes_taken(data: &Vec<Vec<char>>, x: usize, y: usize, look_afar: bool) -> bool {
  data[y][x] == 'L'
    && ((!look_afar && count_occurrence(data, x, y) == 0)
      || (look_afar && count_occurrence_eagle_eye(data, x, y) == 0))
}

fn seat_becomes_free(data: &Vec<Vec<char>>, x: usize, y: usize, look_afar: bool) -> bool {
  let limit = if look_afar { 5 } else { 4 };
  data[y][x] == '#'
    && ((!look_afar && count_occurrence(data, x, y) >= limit)
      || (look_afar && count_occurrence_eagle_eye(data, x, y) >= limit))
}

fn count_occurrence(data: &Vec<Vec<char>>, x: usize, y: usize) -> u32 {
  let mut count = 0;
  for j in
    std::cmp::max(0, y as i32 - 1) as usize..std::cmp::min(data.len() as i32, y as i32 + 2) as usize
  {
    for i in std::cmp::max(0, x as i32 - 1) as usize
      ..std::cmp::min(data[y].len() as i32, x as i32 + 2) as usize
    {
      if i == x && j == y {
        continue;
      }
      if data[j][i] == '#' {
        count += 1;
      }
    }
  }
  count
}

fn count_occurrence_eagle_eye(data: &Vec<Vec<char>>, x: usize, y: usize) -> u32 {
  seat_taken_in_direction(data, (x, y), (1, 1))
    + seat_taken_in_direction(data, (x, y), (1, -1))
    + seat_taken_in_direction(data, (x, y), (-1, 1))
    + seat_taken_in_direction(data, (x, y), (-1, -1))
    + seat_taken_in_direction(data, (x, y), (1, 0))
    + seat_taken_in_direction(data, (x, y), (-1, 0))
    + seat_taken_in_direction(data, (x, y), (0, 1))
    + seat_taken_in_direction(data, (x, y), (0, -1))
}

fn seat_taken_in_direction(
  data: &Vec<Vec<char>>,
  from: (usize, usize),
  direction: (i32, i32),
) -> u32 {
  let mut current = ((from.0 as i32 + direction.0), (from.1 as i32 + direction.1));
  while current.1 >= 0
    && current.0 >= 0
    && current.1 < data.len() as i32
    && current.0 < data[current.1 as usize].len() as i32
  {
    if data[current.1 as usize][current.0 as usize] == 'L' {
      return 0;
    } else if data[current.1 as usize][current.0 as usize] == '#' {
      return 1;
    }
    current = (
      (current.0 as i32 + direction.0),
      (current.1 as i32 + direction.1),
    );
  }
  0
}

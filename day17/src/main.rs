use std::collections::HashSet;

fn main() {
  let before = std::time::Instant::now();
  let args: Vec<String> = std::env::args().collect();
  if args.len() < 2 {
    panic!("Not enough arguments");
  }
  let data = std::fs::read_to_string(&args[1]).unwrap();
  let before_parsing = std::time::Instant::now();
  let cubes: HashSet<(i64, i64, i64)> = data
    .lines()
    .enumerate()
    .map(|l| {
      l.1
        .chars()
        .enumerate()
        .filter(|c| c.1 == '#')
        .map(move |c| (c.0 as i64, l.0 as i64, 0))
    })
    .flatten()
    .collect();
  let parsing_time = before_parsing.elapsed();

  let before_p1 = std::time::Instant::now();
  let p1 = part1(&cubes);
  let p1_time = before_p1.elapsed();

  // let before_p2 = std::time::Instant::now();
  // let p2 = part2(&ticket_infos);
  // let p2_time = before_p2.elapsed();

  let total_time = before.elapsed();

  println!("Parsing: elapsed time: {:.2?}", parsing_time);
  println!("Part1: {:>4} | elapsed time: {:.2?}", p1, p1_time);
  // println!("Part2: {:>4} | elapsed time: {:.2?}", p2, p2_time,);
  println!("Total elapsed time: {:.2?}", total_time);
  println!("Print elapsed time: {:.2?}", before.elapsed());
}

fn part1(cubes: &HashSet<(i64, i64, i64)>) -> usize {
  let mut state = cubes.clone();
  for _i in 0..6 {
    state = cycle(&state);
    // print(&state);
  }
  state.len()
}

fn print(cubes: &HashSet<(i64, i64, i64)>) {
  let mut s = (0, 0, 0);
  let mut b = (0, 0, 0);
  for cube in cubes {
    if cube.0 < s.0 {
      s.0 = cube.0;
    }
    if cube.0 > b.0 {
      b.0 = cube.0;
    }
    if cube.1 < s.1 {
      s.1 = cube.1;
    }
    if cube.1 > b.1 {
      b.1 = cube.1;
    }
    if cube.2 < s.2 {
      s.2 = cube.2;
    }
    if cube.2 > b.2 {
      b.2 = cube.2;
    }
  }
  for z in s.2..b.2 + 1 {
    println!("z={}", z);
    for y in s.1..b.1 + 1 {
      for x in s.0..b.0 + 1 {
        print!("{}", if cubes.contains(&(x, y, z)) { '#' } else { '.' });
      }
      println!();
    }
    println!();
  }
  println!("=======");
}

fn cycle(state: &HashSet<(i64, i64, i64)>) -> HashSet<(i64, i64, i64)> {
  let mut new_state = HashSet::new();
  for active in state.iter() {
    let active_neighbours = active_neighbour_count(active, state);
    // println!("{}", active_neighbours);
    if active_neighbours == 2 || active_neighbours == 3 {
      new_state.insert(*active);
    }
    new_state.extend(valid_neighbours(active, state));
  }
  new_state
}

fn active_neighbour_count(cube: &(i64, i64, i64), state: &HashSet<(i64, i64, i64)>) -> usize {
  let mut count = 0;
  for x in -1..2 {
    for y in -1..2 {
      for z in -1..2 {
        let neighbour = (x + cube.0, y + cube.1, z + cube.2);
        if *cube != neighbour && state.contains(&neighbour) {
          // println!("{},{},{}", neighbour.0, neighbour.1, neighbour.2);
          count += 1;
        }
      }
    }
  }
  count
}

fn valid_neighbours(
  cube: &(i64, i64, i64),
  state: &HashSet<(i64, i64, i64)>,
) -> HashSet<(i64, i64, i64)> {
  let mut valid = HashSet::new();
  // println!(
  //   "Looking for inactive neighbours of: {},{},{}",
  //   cube.0, cube.1, cube.2
  // );
  for x in -1..2 {
    for y in -1..2 {
      for z in -1..2 {
        let neighbour = (x + cube.0, y + cube.1, z + cube.2);
        if *cube != neighbour && !state.contains(&neighbour) {
          // println!(
          //   "Looking for neighbours of: {},{},{}",
          //   neighbour.0, neighbour.1, neighbour.2
          // );
        }
        if *cube != neighbour
          && !state.contains(&neighbour)
          && active_neighbour_count(&neighbour, state) == 3
        {
          // println!("inserted");
          valid.insert(neighbour);
        }
      }
    }
  }
  valid
}

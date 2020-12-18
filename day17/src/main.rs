use std::collections::HashSet;

fn main() {
  let before = std::time::Instant::now();
  let args: Vec<String> = std::env::args().collect();
  if args.len() < 2 {
    panic!("Not enough arguments");
  }
  let data = std::fs::read_to_string(&args[1]).unwrap();
  let before_parsing = std::time::Instant::now();
  let cubes: HashSet<Vec<i64>> = data
    .lines()
    .enumerate()
    .map(|l| {
      l.1
        .chars()
        .enumerate()
        .filter(|c| c.1 == '#')
        .map(move |c| vec![c.0 as i64, l.0 as i64, 0])
    })
    .flatten()
    .collect();
  let parsing_time = before_parsing.elapsed();

  let before_p1 = std::time::Instant::now();
  let p1 = part1(&cubes);
  let p1_time = before_p1.elapsed();

  let before_p2 = std::time::Instant::now();
  let p2 = part2(&cubes);
  let p2_time = before_p2.elapsed();

  let total_time = before.elapsed();

  println!("Parsing: elapsed time: {:.2?}", parsing_time);
  println!("Part1: {:>4} | elapsed time: {:.2?}", p1, p1_time);
  println!("Part2: {:>4} | elapsed time: {:.2?}", p2, p2_time,);
  println!("Total elapsed time: {:.2?}", total_time);
  println!("Print elapsed time: {:.2?}", before.elapsed());
}

fn part1(cubes: &HashSet<Vec<i64>>) -> usize {
  let mut state = cubes.clone();
  for _i in 0..6 {
    state = cycle(&state);
  }
  state.len()
}

fn part2(cubes: &HashSet<Vec<i64>>) -> usize {
  let mut state = cubes
    .clone()
    .iter()
    .map(|c| c.iter().chain(std::iter::once(&0)).map(|p| *p).collect())
    .collect();
  for _i in 0..6 {
    state = cycle(&state);
  }
  state.len()
}

fn cycle(state: &HashSet<Vec<i64>>) -> HashSet<Vec<i64>> {
  let mut new_state = HashSet::new();
  for active in state.iter() {
    let active_neighbours = active_neighbour_count(active, state);
    if active_neighbours == 2 || active_neighbours == 3 {
      new_state.insert(active.clone());
    }
    new_state.extend(valid_neighbours(active, state));
  }
  new_state
}

fn active_neighbour_count(cube: &Vec<i64>, state: &HashSet<Vec<i64>>) -> usize {
  let mut relative_pos = vec![0; cube.len()];
  active_neighbour_count_rec(cube, state, 0, &mut relative_pos)
}

fn active_neighbour_count_rec(
  cube: &Vec<i64>,
  state: &HashSet<Vec<i64>>,
  depth: usize,
  relative_pos: &mut Vec<i64>,
) -> usize {
  if depth == cube.len() {
    let mut neighbour = cube.clone();
    for i in 0..cube.len() {
      neighbour[i] += relative_pos[i];
    }
    if *cube != neighbour && state.contains(&neighbour) {
      1
    } else {
      0
    }
  } else {
    let mut result = 0;
    relative_pos[depth] = -1;
    result += active_neighbour_count_rec(cube, state, depth + 1, relative_pos);
    relative_pos[depth] = 0;
    result += active_neighbour_count_rec(cube, state, depth + 1, relative_pos);
    relative_pos[depth] = 1;
    result += active_neighbour_count_rec(cube, state, depth + 1, relative_pos);
    result
  }
}

fn valid_neighbours(cube: &Vec<i64>, state: &HashSet<Vec<i64>>) -> HashSet<Vec<i64>> {
  let mut relative_pos = vec![0; cube.len()];
  valid_neighbours_rec(cube, state, 0, &mut relative_pos)
}

fn valid_neighbours_rec(
  cube: &Vec<i64>,
  state: &HashSet<Vec<i64>>,
  depth: usize,
  relative_pos: &mut Vec<i64>,
) -> HashSet<Vec<i64>> {
  if depth == cube.len() {
    let mut neighbour = cube.clone();
    for i in 0..cube.len() {
      neighbour[i] += relative_pos[i];
    }
    if *cube != neighbour && active_neighbour_count(&neighbour, state) == 3 {
      [neighbour].iter().cloned().collect()
    } else {
      HashSet::new()
    }
  } else {
    let mut result = HashSet::new();
    relative_pos[depth] = -1;
    result.extend(valid_neighbours_rec(cube, state, depth + 1, relative_pos));
    relative_pos[depth] = 0;
    result.extend(valid_neighbours_rec(cube, state, depth + 1, relative_pos));
    relative_pos[depth] = 1;
    result.extend(valid_neighbours_rec(cube, state, depth + 1, relative_pos));
    result
  }
}

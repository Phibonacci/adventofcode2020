fn main() {
  let before = std::time::Instant::now();
  let args: Vec<String> = std::env::args().collect();
  if args.len() < 2 {
    panic!("Not enough arguments");
  }
  let data = std::fs::read_to_string(&args[1]).unwrap();
  let before_parsing = std::time::Instant::now();
  let tiles = parse_data(&data);
  let parsing_time = before_parsing.elapsed();

  let before_p1 = std::time::Instant::now();
  let p1 = part1(&tiles);
  let p1_time = before_p1.elapsed();

  let before_p2 = std::time::Instant::now();
  let p2 = part2(&tiles);
  let p2_time = before_p2.elapsed();

  let total_time = before.elapsed();

  println!("Parsing: elapsed time: {:.2?}", parsing_time);
  println!("Part1: {:>4} | elapsed time: {:.2?}", p1, p1_time);
  println!("Part2: {:>4} | elapsed time: {:.2?}", p2, p2_time,);
  println!("Total elapsed time: {:.2?}", total_time);
  println!("Print elapsed time: {:.2?}", before.elapsed());
}

fn parse_data(data: &str) -> Vec<Tile> {
  let mut tiles = Vec::new();
  let mut iter_lines = data.lines();
  loop {
    let next = iter_lines.next();
    if next == None {
      break;
    }
    let id = next
      .unwrap()
      .split_at(next.unwrap().find(':').unwrap())
      .0
      .split_at(next.unwrap().find(' ').unwrap() + 1)
      .1
      .parse()
      .unwrap();
    let tile_lines = iter_lines.by_ref().take_while(|l| !l.is_empty()).collect();
    tiles.push((id, tile_lines))
  }
  tiles
}

type Tile<'a> = (TileId, TileData<'a>);
type TileId = u64;
type TileData<'a> = Vec<&'a str>;

#[derive(Clone)]
struct TileSetup<'a> {
  orientation: Orientation,
  flip_right: bool,
  flip_down: bool,
  tile: &'a Tile<'a>,
}

#[derive(Clone, PartialEq)]
enum Direction {
  UP,
  RIGHT,
  DOWN,
  LEFT,
}

type Orientation = Direction;
type Side = Direction;

fn part1(tiles: &Vec<Tile>) -> u64 {
  find_corners_sum(tiles)
}

fn part2(tiles: &Vec<Tile>) -> usize {
  let board = solve_board(tiles);
  let image = glue_pieces(&board);
  rocks(&image)
}

fn rocks(image: &Vec<String>) -> usize {
  let mut less_rocks: usize = image
    .iter()
    .map(|l| l.chars().filter(|c| *c == '#').count())
    .sum();
  let source_tile: (u64, Vec<&str>) = (0, image.iter().map(|line| line.as_str()).collect());
  for orientation in &[
    Orientation::UP,
    Orientation::RIGHT,
    Orientation::DOWN,
    Orientation::LEFT,
  ] {
    for flipping in &[(false, false), (false, true), (true, false), (true, true)] {
      let tile = TileSetup {
        orientation: orientation.clone(),
        flip_right: flipping.0,
        flip_down: flipping.1,
        tile: &source_tile,
      };
      let rocks = rocks_left(&tile.to_image());
      if less_rocks > rocks {
        less_rocks = rocks;
      }
    }
  }
  less_rocks
}

fn rocks_left(image: &Vec<String>) -> usize {
  let mut image_filtered = image.clone();
  for y in 0..image.len() {
    for x in 0..image[y].len() {
      remove_found_monster(&image, &mut image_filtered, &(x, y));
    }
  }
  image_filtered
    .iter()
    .map(|l| l.chars().filter(|c| *c == '#').count())
    .sum()
}

fn remove_found_monster(
  image: &Vec<String>,
  image_filtered: &mut Vec<String>,
  position: &(usize, usize),
) {
  if has_monster(image, &position) {
    remove_monster(image_filtered, &position);
  }
}

const MONSTER: &'static [&'static str] = &[
  "                  # ",
  "#    ##    ##    ###",
  " #  #  #  #  #  #   ",
];

fn remove_monster(image: &mut Vec<String>, position: &(usize, usize)) {
  if position.1 > image.len() - 3 || position.0 > image[position.1].len() - 20 {
    return;
  } else {
    for line_id in 0..3 {
      remove_monster_line(
        MONSTER[line_id],
        &mut image[position.1 + line_id][position.0..(position.0 + 20)],
      );
    }
  }
}

fn remove_monster_line(line_ref: &str, line_image: &mut str) {
  line_image
    .chars()
    .zip(line_ref.chars())
    .all(|(a, b)| a == ' ' || (a == '#' && b == '#'));
  for i in 0..line_image.len() {
    if line_ref.chars().nth(i).unwrap() == '#' {
      unsafe { line_image.as_bytes_mut()[i] = ' ' as u8 };
    }
  }
}

fn has_monster(image: &Vec<String>, position: &(usize, usize)) -> bool {
  if position.1 > image.len() - 3 || position.0 > image[position.1].len() - 20 {
    false
  } else {
    for line_id in 0..3 {
      if !has_monster_line(
        MONSTER[line_id],
        &image[position.1 + line_id][position.0..(position.0 + 20)],
      ) {
        return false;
      }
    }
    true
  }
}

fn has_monster_line(line_ref: &str, line_image: &str) -> bool {
  line_ref
    .chars()
    .zip(line_image.chars())
    .all(|(a, b)| a == ' ' || (a == '#' && b == '#'))
}

fn glue_pieces(board: &Vec<Vec<Option<TileSetup>>>) -> Vec<String> {
  let mut image = Vec::new();
  for row in board {
    for i in 1..9 {
      let mut line = String::new();
      for tile in row {
        use std::iter::FromIterator;
        let piece_line = String::from_iter(tile.as_ref().unwrap().iter_on_row(i));
        line = format!("{}{}", &line, &piece_line[1..9]);
      }
      image.push(line);
    }
  }
  image
}

fn solve_board<'a>(tiles: &'a Vec<Tile>) -> Vec<Vec<Option<TileSetup<'a>>>> {
  let board_size = (tiles.len() as f64).sqrt() as usize;
  let mut board: Vec<Vec<Option<TileSetup>>> = vec![vec![None; board_size]; board_size];
  let mut available_tiles: Vec<&Tile> = tiles.iter().collect();
  board[0][0] = Some(find_first_corner_as_top_left(&tiles));
  available_tiles.remove(
    available_tiles
      .iter()
      .position(|t| t.0 == board[0][0].as_ref().unwrap().tile.0)
      .unwrap(),
  );
  let mut last_tile = (0, 0);
  let mut look_at = Direction::RIGHT;
  let mut keep_looking = true;
  while !available_tiles.is_empty() {
    if !keep_looking {
      look_at = iterate_direction(&look_at);
    }
    keep_looking = false;
    let mut matching = None;
    let mut tile_index = 0;
    for tile in available_tiles.iter().enumerate() {
      matching = find_matching_side(
        board[last_tile.1][last_tile.0].as_ref().unwrap(),
        tile.1,
        &look_at,
      );
      if !matching.is_none() {
        tile_index = tile.0;
        break;
      }
    }
    if !matching.is_none() {
      last_tile = iterate_coordinate(&last_tile, &look_at);
      keep_looking = true;
      board[last_tile.1][last_tile.0] = matching;
      available_tiles.remove(tile_index);
    }
  }
  board
}

fn iterate_coordinate(coordinate: &(usize, usize), direction: &Direction) -> (usize, usize) {
  match direction {
    Direction::UP => (coordinate.0, coordinate.1 - 1),
    Direction::RIGHT => (coordinate.0 + 1, coordinate.1),
    Direction::DOWN => (coordinate.0, coordinate.1 + 1),
    Direction::LEFT => (coordinate.0 - 1, coordinate.1),
  }
}

fn iterate_direction(direction: &Direction) -> Direction {
  number_to_direction((direction_to_number(direction) + 1) % 4)
}

fn find_corners_sum(tiles: &Vec<Tile>) -> u64 {
  let mut result = 1;
  for i in 0..tiles.len() {
    let tile_setup = TileSetup {
      orientation: Orientation::UP,
      flip_right: false,
      flip_down: false,
      tile: &tiles[i],
    };
    let mut sides: Vec<Side> = Vec::new();
    for j in 0..tiles.len() {
      if i == j {
        continue;
      }
      let side_tile = find_any_matching_side(&tile_setup, &tiles[j]);
      if !side_tile.is_none() {
        sides.push(side_tile.unwrap().0);
      }
    }
    if sides.len() == 2 {
      result *= tiles[i].0;
    }
  }
  result
}

fn find_first_corner_as_top_left<'a>(tiles: &'a Vec<Tile>) -> TileSetup<'a> {
  for i in 0..tiles.len() {
    let mut tile_setup = TileSetup {
      orientation: Orientation::UP,
      flip_right: false,
      flip_down: false,
      tile: &tiles[i],
    };
    let mut sides: Vec<Side> = Vec::new();
    let mut matches: Vec<TileSetup> = Vec::new();
    for j in 0..tiles.len() {
      if i == j {
        continue;
      }
      let side_tile = find_any_matching_side(&tile_setup, &tiles[j]);
      if !side_tile.is_none() {
        sides.push(side_tile.as_ref().unwrap().0.clone());
        matches.push(side_tile.unwrap().1);
      }
    }
    if sides.len() == 2 {
      tile_setup.orientation = get_top_left_corner_orientation(&sides);
      return tile_setup;
    }
  }
  panic!("No corner found");
}

fn get_top_left_corner_orientation(sides: &Vec<Side>) -> Orientation {
  let mut sides_values = sides
    .iter()
    .map(|s| direction_to_number(s))
    .collect::<Vec<u64>>();
  sides_values.sort();
  match &sides_values[..] {
    [1, 2] => Orientation::UP,
    [2, 3] => Orientation::LEFT,
    [0, 3] => Orientation::DOWN,
    [0, 1] => Orientation::RIGHT,
    _ => unreachable!(),
  }
}

fn find_any_matching_side<'a>(a: &TileSetup, b: &'a Tile) -> Option<(Side, TileSetup<'a>)> {
  for side in [Side::UP, Side::RIGHT, Side::DOWN, Side::LEFT].iter() {
    let result = find_matching_side(a, b, side);
    if !result.is_none() {
      return Some((side.clone(), result.unwrap()));
    }
  }
  None
}

fn find_matching_side<'a>(a: &TileSetup, b: &'a Tile<'a>, side_a: &Side) -> Option<TileSetup<'a>> {
  for orientation in &[
    Orientation::UP,
    Orientation::RIGHT,
    Orientation::DOWN,
    Orientation::LEFT,
  ] {
    for flipping in &[(false, false), (false, true), (true, false), (true, true)] {
      let tile_setup = TileSetup::<'a> {
        orientation: orientation.clone(),
        flip_right: flipping.0,
        flip_down: flipping.1,
        tile: b,
      };
      if side_match(a, &tile_setup, side_a) {
        return Some(tile_setup);
      }
    }
  }
  None
}

fn opposite_side(side: &Side) -> Side {
  match side {
    Side::UP => Side::DOWN,
    Side::RIGHT => Side::LEFT,
    Side::DOWN => Side::UP,
    Side::LEFT => Side::RIGHT,
  }
}

fn side_match(a: &TileSetup, b: &TileSetup, side_a: &Side) -> bool {
  let side_b = opposite_side(side_a);
  a.iter_on_side(&side_a)
    .zip(b.iter_on_side(&side_b))
    .all(|(a, b)| a == b)
}

impl<'a> TileSetup<'a> {
  fn to_image(&self) -> Vec<String> {
    let mut image = Vec::new();
    for i in 0..self.tile.1.len() {
      use std::iter::FromIterator;
      image.push(String::from_iter(self.iter_on_row(i)));
    }
    image
  }

  fn iter_on_side(&'a self, side: &Side) -> Box<dyn Iterator<Item = char> + 'a> {
    self.iter_on_row_side(side, 0)
  }

  fn iter_on_row(&'a self, row: usize) -> Box<dyn Iterator<Item = char> + 'a> {
    self.iter_on_row_side(&Side::UP, row)
  }

  fn iter_on_row_side(&'a self, side: &Side, row: usize) -> Box<dyn Iterator<Item = char> + 'a> {
    match absolute_side(&self.orientation, &side) {
      Side::DOWN => Box::new(DownIter::<'a> {
        tile_setup: self,
        cur: 0,
        row: row,
        flip_down: self.flip_down,
        flip_right: if self.orientation == Orientation::DOWN
          || self.orientation == Orientation::LEFT
        {
          !self.flip_right
        } else {
          self.flip_right
        },
      }),
      Side::RIGHT => Box::new(RightIter::<'a> {
        tile_setup: self,
        cur: 0,
        row: row,
        flip_down: if self.orientation == Orientation::RIGHT
          || self.orientation == Orientation::DOWN
        {
          !self.flip_down
        } else {
          self.flip_down
        },
        flip_right: self.flip_right,
      }),
      Side::UP => Box::new(UpIter::<'a> {
        tile_setup: self,
        cur: 0,
        row: row,
        flip_down: self.flip_down,
        flip_right: if self.orientation == Orientation::DOWN
          || self.orientation == Orientation::LEFT
        {
          !self.flip_right
        } else {
          self.flip_right
        },
      }),
      Side::LEFT => Box::new(LeftIter::<'a> {
        tile_setup: self,
        cur: 0,
        row: row,
        flip_right: self.flip_right,
        flip_down: if self.orientation == Orientation::RIGHT
          || self.orientation == Orientation::DOWN
        {
          !self.flip_down
        } else {
          self.flip_down
        },
      }),
    }
  }
}

struct UpIter<'a> {
  tile_setup: &'a TileSetup<'a>,
  cur: usize,
  row: usize,
  flip_down: bool,
  flip_right: bool,
}

impl Iterator for UpIter<'_> {
  type Item = char;

  fn next(&mut self) -> Option<Self::Item> {
    self.cur += 1;
    let row = match self.flip_down {
      true => self
        .tile_setup
        .tile
        .1
        .get(self.tile_setup.tile.1.len() - 1 - self.row)
        .unwrap(),
      false => self.tile_setup.tile.1.get(self.row).unwrap(),
    };
    match self.flip_right {
      false => row.chars().nth(self.cur - 1),
      true => row.chars().nth(row.len() - 1 - (self.cur - 1)),
    }
  }
}

struct RightIter<'a> {
  tile_setup: &'a TileSetup<'a>,
  cur: usize,
  row: usize,
  flip_down: bool,
  flip_right: bool,
}

impl Iterator for RightIter<'_> {
  type Item = char;

  fn next(&mut self) -> Option<Self::Item> {
    if self.tile_setup.tile.1.len() <= self.cur {
      None
    } else {
      self.cur += 1;
      let row = match self.flip_down {
        true => self.tile_setup.tile.1[self.tile_setup.tile.1.len() - 1 - (self.cur - 1)],
        false => self.tile_setup.tile.1[self.cur - 1],
      };
      match self.flip_right {
        true => row.chars().nth(self.row),
        false => row.chars().nth(row.len() - 1 - self.row),
      }
    }
  }
}

struct DownIter<'a> {
  tile_setup: &'a TileSetup<'a>,
  cur: usize,
  row: usize,
  flip_down: bool,
  flip_right: bool,
}

impl Iterator for DownIter<'_> {
  type Item = char;

  fn next(&mut self) -> Option<Self::Item> {
    self.cur += 1;
    let row = match self.flip_down {
      true => self.tile_setup.tile.1.get(self.row).unwrap(),
      false => self
        .tile_setup
        .tile
        .1
        .get(self.tile_setup.tile.1.len() - 1 - self.row)
        .unwrap(),
    };
    match self.flip_right {
      false => row.chars().nth(self.cur - 1),
      true => row.chars().nth(row.len() - 1 - (self.cur - 1)),
    }
  }
}

struct LeftIter<'a> {
  tile_setup: &'a TileSetup<'a>,
  cur: usize,
  row: usize,
  flip_down: bool,
  flip_right: bool,
}

impl Iterator for LeftIter<'_> {
  type Item = char;

  fn next(&mut self) -> Option<Self::Item> {
    if self.tile_setup.tile.1.len() <= self.cur {
      None
    } else {
      self.cur += 1;
      let row = match self.flip_down {
        true => self.tile_setup.tile.1[self.tile_setup.tile.1.len() - 1 - (self.cur - 1)],
        false => self.tile_setup.tile.1[self.cur - 1],
      };
      match self.flip_right {
        true => row.chars().nth(row.len() - 1 - self.row),
        false => row.chars().nth(self.row),
      }
    }
  }
}

fn absolute_side(orientation: &Orientation, side: &Side) -> Side {
  let orientation_value = direction_to_number(orientation);
  let side_value = direction_to_number(side);
  number_to_direction((4 + side_value - orientation_value) % 4)
}

fn direction_to_number(direction: &Direction) -> u64 {
  match direction {
    Direction::UP => 0,
    Direction::RIGHT => 1,
    Direction::DOWN => 2,
    Direction::LEFT => 3,
  }
}

fn number_to_direction(number: u64) -> Direction {
  match number {
    0 => Direction::UP,
    1 => Direction::RIGHT,
    2 => Direction::DOWN,
    3 => Direction::LEFT,
    _ => unreachable!(),
  }
}

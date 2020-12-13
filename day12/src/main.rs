fn main() {
  let before = std::time::Instant::now();
  let before_parsing = std::time::Instant::now();
  let args: Vec<String> = std::env::args().collect();
  if args.len() < 2 {
    panic!("Not enough arguments");
  }
  let data: Vec<(char, i32)> = std::fs::read_to_string(&args[1])
    .unwrap()
    .lines()
    .map(|l| {
      let e = l.parse::<String>().unwrap();
      (
        e.chars().next().unwrap(),
        e.get(1..).unwrap().parse().unwrap(),
      )
    })
    .collect();
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

struct Coordinates {
  x: i32,
  y: i32,
}

fn part1(data: &Vec<(char, i32)>) -> i32 {
  let mut pos = Coordinates { x: 0, y: 0 };
  let mut direction = Coordinates { x: 1, y: 0 };
  for instruction in data {
    match instruction.0 {
      'N' => pos.y -= instruction.1,
      'S' => pos.y += instruction.1,
      'E' => pos.x += instruction.1,
      'W' => pos.x -= instruction.1,
      'F' => {
        pos.x += direction.x * instruction.1;
        pos.y += direction.y * instruction.1
      }
      'L' => direction = turn(&direction, -instruction.1),
      'R' => direction = turn(&direction, instruction.1),
      _ => unreachable!(),
    }
  }
  pos.x.abs() + pos.y.abs()
}

fn part2(data: &Vec<(char, i32)>) -> i32 {
  let mut ship = Coordinates { x: 0, y: 0 };
  let mut way_point = Coordinates { x: 10, y: -1 };
  for instruction in data {
    match instruction.0 {
      'N' => way_point.y -= instruction.1,
      'S' => way_point.y += instruction.1,
      'E' => way_point.x += instruction.1,
      'W' => way_point.x -= instruction.1,
      'F' => {
        ship.x += way_point.x * instruction.1;
        ship.y += way_point.y * instruction.1;
      }
      'L' => way_point = turn(&way_point, -instruction.1),
      'R' => way_point = turn(&way_point, instruction.1),
      _ => unreachable!(),
    }
  }
  ship.x.abs() + ship.y.abs()
}

fn turn(direction: &Coordinates, angle: i32) -> Coordinates {
  let radians = (angle as f32).to_radians();
  Coordinates {
    x: (direction.x as f32 * radians.cos() - direction.y as f32 * radians.sin()).round() as i32,
    y: (direction.x as f32 * radians.sin() + direction.y as f32 * radians.cos()).round() as i32,
  }
}

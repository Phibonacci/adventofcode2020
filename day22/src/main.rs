use std::collections::VecDeque;
type Deck = VecDeque<usize>;

fn main() {
  let path = std::env::args().skip(1).next().expect("Missing argument");
  let data = std::fs::read_to_string(&path).unwrap();
  let before = std::time::Instant::now();

  let before_parsing = std::time::Instant::now();
  let decks = parse_data(&data);
  let parsing_time = before_parsing.elapsed();

  let before_p1 = std::time::Instant::now();
  let p1 = combat(&mut decks.clone());
  let p1_time = before_p1.elapsed();

  let before_p2 = std::time::Instant::now();
  let p2 = recursive_combat(&mut decks.clone());
  let p2_time = before_p2.elapsed();

  let total_time = before.elapsed();

  println!("Parsing: elapsed time: {:.2?}", parsing_time);
  println!("Part1: {:>4} | elapsed time: {:.2?}", p1, p1_time);
  println!("Part2: {:>4} | elapsed time: {:.2?}", p2, p2_time,);
  println!("Total elapsed time: {:.2?}", total_time);
}

fn parse_data(data: &str) -> (Deck, Deck) {
  let mut iter = data
    .split("\n\n")
    .map(|s| s.lines().skip(1).map(|l| l.parse().unwrap()).collect());
  (iter.next().unwrap(), iter.next().unwrap())
}

fn combat(decks: &mut (Deck, Deck)) -> usize {
  while !decks.0.is_empty() && !decks.1.is_empty() {
    let cards = (decks.0.pop_front().unwrap(), decks.1.pop_front().unwrap());
    if cards.0 > cards.1 {
      decks.0.extend(vec![cards.0, cards.1]);
    } else {
      decks.1.extend(vec![cards.1, cards.0]);
    }
  }
  match decks.0.is_empty() {
    true => score(&decks.1),
    false => score(&decks.0),
  }
}

fn recursive_combat(decks: &mut (Deck, Deck)) -> usize {
  match combat_rec(decks) {
    true => score(&decks.0),
    false => score(&decks.1),
  }
}

fn combat_rec(decks: &mut (Deck, Deck)) -> bool {
  let mut draws = Vec::new();
  while !decks.0.is_empty() && !decks.1.is_empty() {
    let cards = (decks.0.pop_front().unwrap(), decks.1.pop_front().unwrap());
    match round_rec(decks, &cards) {
      true => decks.0.extend(&[cards.0, cards.1]),
      false => decks.1.extend(&[cards.1, cards.0]),
    }
    if draws.contains(decks) {
      return true;
    }
    draws.push(decks.clone());
  }
  decks.1.is_empty()
}

fn round_rec(decks: &(Deck, Deck), cards: &(usize, usize)) -> bool {
  if cards.0 <= decks.0.len() && cards.1 <= decks.1.len() {
    combat_rec(&mut (
      decks.0.iter().take(cards.0).cloned().collect(),
      decks.1.iter().take(cards.1).cloned().collect(),
    ))
  } else {
    cards.0 > cards.1
  }
}

fn score(deck: &Deck) -> usize {
  deck
    .iter()
    .rev()
    .enumerate()
    .fold(0, |r, i| r + (i.0 + 1) * i.1)
}
